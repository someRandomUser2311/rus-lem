//! The authored content tree (`content/<course>/`): generated layer + overlay.
//!
//! ```text
//! content/rust-basics/
//!   course.toml                     id, title, version, toolchain
//!   course.gen.json                 concepts, chapters, parts (from the plan)
//!   ch01/chapter.gen.json           the chapter as parsed from the plan
//!   ch01/overlay/*.toml             authored partial chapters, merged in name order
//!   ch01/problems/P01-03/
//!     overlay.toml                  authored partial problem (tests by key, wrong variants)
//!     starter/<files>               starter code the spec can't give whole
//!     solution/<files>              reference solution
//!     part-<n>/<files>              checkpoint problems: reference solution of part n
//!     wrong/<name>/<files>          deliberately wrong variants
//!     checkers/<name>.rs            checker programs for output a fixed expectation can't judge
//!     generators/<name>.rs          generator programs for suites (`script = "<name>"`)
//!     helpers.rs                    Rust items added to the call harness
//!   ch02/Cargo.toml                 the manifest of the chapter's Cargo problems (`mode = "cargo"`)
//!   ch07/problems/P07-01/Cargo.toml  a Cargo problem's own manifest (instead of the chapter's)
//!   vendor/                         vendored crates (`cargo vendor`) for offline builds
//! ```
//!
//! Files a solution or wrong variant leaves out (`Cargo.toml`) come from the starter.

use anyhow::{Context, Result, bail};
use rb_content::{Chapter, Course, SourceFile, merge::merge};
use serde::Deserialize;
use serde_json::Value;
use std::path::{Path, PathBuf};

#[derive(Deserialize)]
struct CourseToml {
    id: String,
    title: String,
    version: String,
    toolchain: rb_content::Toolchain,
}

pub struct Source {
    pub dir: PathBuf,
}

impl Source {
    pub fn new(dir: &Path) -> Source {
        Source {
            dir: dir.to_path_buf(),
        }
    }

    /// Vendored crates for offline Cargo builds, if the course has any.
    pub fn vendor_dir(&self) -> Option<PathBuf> {
        // Absolute: Cargo resolves it from each build's own folder.
        std::fs::canonicalize(self.dir.join("vendor")).ok()
    }

    pub fn chapter_dir(&self, num: u32) -> PathBuf {
        self.dir.join(format!("ch{num:02}"))
    }

    pub fn chapters_present(&self) -> Vec<u32> {
        let mut out: Vec<u32> = (1..=99)
            .filter(|n| self.chapter_dir(*n).join("chapter.gen.json").exists())
            .collect();
        out.sort();
        out
    }

    pub fn course(&self) -> Result<Course> {
        let t: CourseToml = toml::from_str(
            &std::fs::read_to_string(self.dir.join("course.toml"))
                .context("reading course.toml")?,
        )?;
        let mut generated: Value = serde_json::from_str(
            &std::fs::read_to_string(self.dir.join("course.gen.json"))
                .context("reading course.gen.json")?,
        )?;
        let present = self.chapters_present();
        if let Some(chs) = generated.get_mut("chapters").and_then(Value::as_array_mut) {
            for c in chs {
                let n = c["num"].as_u64().unwrap_or(0) as u32;
                c["available"] = Value::Bool(present.contains(&n));
            }
        }
        generated["id"] = t.id.into();
        generated["title"] = t.title.into();
        generated["version"] = t.version.into();
        generated["toolchain"] = serde_json::to_value(t.toolchain)?;
        Ok(serde_json::from_value(generated)?)
    }

    /// The chapter with every overlay merged and solution/wrong files attached.
    pub fn chapter(&self, num: u32) -> Result<Chapter> {
        let dir = self.chapter_dir(num);
        let mut v: Value = serde_json::from_str(
            &std::fs::read_to_string(dir.join("chapter.gen.json"))
                .with_context(|| format!("reading chapter {num}"))?,
        )?;

        let overlay_dir = dir.join("overlay");
        if overlay_dir.exists() {
            let mut files: Vec<PathBuf> = std::fs::read_dir(&overlay_dir)?
                .filter_map(|e| e.ok().map(|e| e.path()))
                .filter(|p| p.extension().is_some_and(|e| e == "toml"))
                .collect();
            files.sort();
            for f in files {
                let o: Value = toml_to_json(&f)?;
                merge(&mut v, o);
            }
        }

        let problems_dir = dir.join("problems");
        if let Some(problems) = v.get_mut("problems").and_then(Value::as_array_mut) {
            for p in problems {
                let id = p["id"].as_str().unwrap_or("").to_string();
                let pd = problems_dir.join(&id);
                if !pd.exists() {
                    continue;
                }
                let ov = pd.join("overlay.toml");
                if ov.exists() {
                    merge(p, toml_to_json(&ov)?);
                }
                // A starter the spec can't give whole (one built on an earlier milestone).
                let st = pd.join("starter");
                if st.exists() {
                    p["starter"] = serde_json::to_value(read_files(&st)?)?;
                }
                if p["mode"] == "cargo" {
                    cargo_starter(p, &dir, &pd, &id)?;
                }
                let starter: Vec<SourceFile> = serde_json::from_value(p["starter"].clone())?;
                let sol = pd.join("solution");
                if sol.exists() {
                    p["solution"] = serde_json::to_value(complete(read_files(&sol)?, &starter))?;
                } else if p["mode"] == "cargo" {
                    // The spec's solution, with the starter's manifest.
                    let given: Vec<SourceFile> = serde_json::from_value(p["solution"].clone())?;
                    p["solution"] = serde_json::to_value(complete(given, &starter))?;
                }
                let mut parts = serde_json::Map::new();
                for n in 1..=20 {
                    let pdir = pd.join(format!("part-{n}"));
                    if pdir.exists() {
                        parts.insert(
                            n.to_string(),
                            serde_json::to_value(complete(read_files(&pdir)?, &starter))?,
                        );
                    }
                }
                if !parts.is_empty() {
                    p["part_solutions"] = Value::Object(parts);
                }
                let checkers = pd.join("checkers");
                if checkers.exists() {
                    p["checkers"] = serde_json::to_value(read_files(&checkers)?)?;
                }
                if let Ok(h) = std::fs::read_to_string(pd.join("helpers.rs")) {
                    p["helpers"] = Value::String(h);
                }
                let generators = pd.join("generators");
                if generators.exists() {
                    p["generators"] = serde_json::to_value(read_files(&generators)?)?;
                }
                if let Some(wrong) = p.get_mut("wrong").and_then(Value::as_array_mut) {
                    for w in wrong {
                        let name = w["name"].as_str().unwrap_or("").to_string();
                        let wd = pd.join("wrong").join(&name);
                        if !wd.exists() {
                            bail!(
                                "{id}: wrong variant `{name}` has no folder {}",
                                wd.display()
                            );
                        }
                        w["files"] = serde_json::to_value(read_files(&wd)?)?;
                    }
                }
            }
        }
        serde_json::from_value(v)
            .with_context(|| format!("chapter {num}: merged content doesn't match the model"))
    }
}

/// A Cargo problem's starter: the spec gives only `main.rs`; it becomes `src/main.rs`
/// next to the problem's own `Cargo.toml`, or else the chapter's.
fn cargo_starter(p: &mut Value, chapter_dir: &Path, problem_dir: &Path, id: &str) -> Result<()> {
    let Some(files) = p["starter"].as_array_mut() else {
        return Ok(());
    };
    for f in files.iter_mut() {
        if f["path"] == "main.rs" {
            f["path"] = "src/main.rs".into();
        }
    }
    if !files.iter().any(|f| f["path"] == "Cargo.toml") {
        let own = problem_dir.join("Cargo.toml");
        let manifest = if own.exists() {
            own
        } else {
            chapter_dir.join("Cargo.toml")
        };
        let text = std::fs::read_to_string(&manifest).with_context(|| {
            format!("{id} is a Cargo problem; {} is missing", manifest.display())
        })?;
        files.push(serde_json::json!({ "path": "Cargo.toml", "text": text }));
    }
    Ok(())
}

/// Adds the starter files a solution leaves out (the supplied `Cargo.toml`).
fn complete(mut files: Vec<SourceFile>, starter: &[SourceFile]) -> Vec<SourceFile> {
    for s in starter {
        if !files.iter().any(|f| f.path == s.path) {
            files.push(s.clone());
        }
    }
    files.sort_by(|a, b| a.path.cmp(&b.path));
    files
}

fn toml_to_json(path: &Path) -> Result<Value> {
    let text = std::fs::read_to_string(path)?;
    let t: toml::Value =
        toml::from_str(&text).with_context(|| format!("parsing {}", path.display()))?;
    Ok(serde_json::to_value(t)?)
}

/// Every file under `dir`, with paths relative to it, sorted.
pub fn read_files(dir: &Path) -> Result<Vec<SourceFile>> {
    let mut out = Vec::new();
    walk(dir, dir, &mut out)?;
    out.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(out)
}

fn walk(root: &Path, dir: &Path, out: &mut Vec<SourceFile>) -> Result<()> {
    for e in std::fs::read_dir(dir)? {
        let p = e?.path();
        if p.file_name().is_some_and(|n| n == ".DS_Store") {
            continue;
        }
        if p.is_dir() {
            walk(root, &p, out)?;
        } else {
            let rel = p.strip_prefix(root)?.to_string_lossy().replace('\\', "/");
            out.push(SourceFile {
                path: rel,
                text: std::fs::read_to_string(&p)?,
            });
        }
    }
    Ok(())
}
