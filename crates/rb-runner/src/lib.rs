//! Finding the learner's Rust toolchain and compiling/running code with it.

pub mod process;

use anyhow::{Context, Result, bail};
use process::{Limits, Output};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::AtomicBool;
use std::time::Duration;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Toolchain {
    pub rustc: PathBuf,
    pub cargo: PathBuf,
    /// e.g. "1.97.1"
    pub rustc_version: String,
    pub rust_analyzer: Option<PathBuf>,
}

/// Where to look for tools. GUI apps on macOS don't get the shell's PATH, so the
/// usual install locations are probed directly.
fn candidates(tool: &str) -> Vec<PathBuf> {
    let exe = if cfg!(windows) {
        format!("{tool}.exe")
    } else {
        tool.to_string()
    };
    let mut dirs: Vec<PathBuf> = Vec::new();
    if let Some(home) = std::env::var_os("CARGO_HOME") {
        dirs.push(PathBuf::from(home).join("bin"));
    }
    if let Some(home) = home_dir() {
        dirs.push(home.join(".cargo/bin"));
    }
    if let Some(path) = std::env::var_os("PATH") {
        dirs.extend(std::env::split_paths(&path));
    }
    dirs.extend(["/opt/homebrew/bin", "/usr/local/bin", "/usr/bin"].map(PathBuf::from));
    dirs.into_iter().map(|d| d.join(&exe)).collect()
}

fn home_dir() -> Option<PathBuf> {
    std::env::var_os(if cfg!(windows) { "USERPROFILE" } else { "HOME" }).map(PathBuf::from)
}

/// The first candidate that runs `--version` successfully.
fn find(tool: &str) -> Option<(PathBuf, String)> {
    for c in candidates(tool) {
        if !c.is_file() {
            continue;
        }
        let Ok(out) = Command::new(&c).arg("--version").output() else {
            continue;
        };
        if out.status.success() {
            return Some((c, String::from_utf8_lossy(&out.stdout).trim().to_string()));
        }
    }
    None
}

impl Toolchain {
    pub fn detect() -> Result<Toolchain> {
        let Some((rustc, version)) = find("rustc") else {
            bail!(
                "Rust isn't installed (no working `rustc` found). Install it from https://rustup.rs"
            );
        };
        let Some((cargo, _)) = find("cargo") else {
            bail!(
                "Cargo isn't installed (no working `cargo` found). Install Rust from https://rustup.rs"
            );
        };
        // "rustc 1.97.1 (8bab26f4f 2026-07-14)" → "1.97.1"
        let rustc_version = version.split_whitespace().nth(1).unwrap_or("").to_string();
        let rust_analyzer = find("rust-analyzer").map(|(p, _)| p);
        Ok(Toolchain {
            rustc,
            cargo,
            rustc_version,
            rust_analyzer,
        })
    }

    /// Is the toolchain at least `min` ("1.85.0")?
    pub fn at_least(&self, min: &str) -> bool {
        let parse = |v: &str| -> Vec<u32> {
            v.split(|c: char| !c.is_ascii_digit())
                .filter_map(|n| n.parse().ok())
                .take(3)
                .collect()
        };
        parse(&self.rustc_version) >= parse(min)
    }

    /// Environment shared by every tool invocation: no colours from the program
    /// under test, no backtraces, PATH containing the toolchain's directory.
    fn env(&self, cmd: &mut Command) {
        cmd.env("RUST_BACKTRACE", "0");
        if let Some(dir) = self.cargo.parent() {
            let mut path = vec![dir.to_path_buf()];
            if let Some(p) = std::env::var_os("PATH") {
                path.extend(std::env::split_paths(&p));
            }
            if let Ok(joined) = std::env::join_paths(path) {
                cmd.env("PATH", joined);
            }
        }
    }

    /// `rustc` on one file, producing `out`.
    pub fn compile_rustc(
        &self,
        src: &Path,
        out: &Path,
        edition: &str,
        extra: &[&str],
    ) -> Result<Compiled> {
        let mut cmd = Command::new(&self.rustc);
        self.env(&mut cmd);
        cmd.arg("--edition").arg(edition).args([
            "--error-format=json",
            "--json=diagnostic-rendered-ansi",
            "-C",
            "debuginfo=0",
        ]);
        // Compile from the file's folder with a relative path, so messages say
        // `main.rs:2:5` rather than a temporary path.
        match (src.parent(), src.file_name()) {
            (Some(dir), Some(name)) => {
                cmd.args(extra)
                    .arg("-o")
                    .arg(out)
                    .arg(name)
                    .current_dir(dir);
            }
            _ => {
                cmd.args(extra).arg("-o").arg(out).arg(src);
            }
        }
        let o = process::run(
            cmd,
            b"",
            &Limits {
                timeout: Duration::from_secs(60),
                max_output: 4 << 20,
            },
            None,
        )?;
        let mut c = Compiled {
            ok: o.success(),
            ..Default::default()
        };
        for line in o.stderr_str().lines() {
            match serde_json::from_str::<serde_json::Value>(line) {
                Ok(v) => c.push(&v),
                Err(_) => c.log.push_str(&format!("{line}\n")),
            }
        }
        if o.timed_out {
            c.log
                .push_str("The compiler took longer than 60 seconds and was stopped.\n");
        }
        Ok(c)
    }

    /// A Cargo command in `dir` with JSON diagnostics. `target_dir` shares build output
    /// between runs; `vendor` is a folder of vendored crates that replaces crates.io.
    pub fn cargo(
        &self,
        dir: &Path,
        sub: &str,
        target_dir: Option<&Path>,
        vendor: Option<&Path>,
        cancel: Option<&AtomicBool>,
    ) -> Result<Compiled> {
        let mut cmd = Command::new(&self.cargo);
        self.env(&mut cmd);
        cmd.current_dir(dir).arg(sub).args([
            "--message-format=json-diagnostic-rendered-ansi",
            "--offline",
            "--color=always",
        ]);
        if let Some(v) = vendor {
            cmd.args(vendor_config(v));
        }
        if let Some(t) = target_dir {
            cmd.env("CARGO_TARGET_DIR", t);
        } else {
            cmd.env_remove("CARGO_TARGET_DIR");
        }
        cmd.env("CARGO_TERM_PROGRESS_WHEN", "never");
        let o = process::run(
            cmd,
            b"",
            &Limits {
                timeout: Duration::from_secs(300),
                max_output: 16 << 20,
            },
            cancel,
        )?;
        let mut c = Compiled {
            ok: o.success(),
            ..Default::default()
        };
        for line in o.stdout_str().lines() {
            let Ok(v) = serde_json::from_str::<serde_json::Value>(line) else {
                continue;
            };
            match v["reason"].as_str() {
                Some("compiler-message") => c.push(&v["message"]),
                Some("compiler-artifact") => {
                    if let Some(exe) = v["executable"].as_str() {
                        c.executable = Some(PathBuf::from(exe));
                    }
                }
                _ => {}
            }
        }
        // Cargo's own progress and errors ("Compiling …", manifest errors) go to stderr.
        c.log.push_str(&o.stderr_str());
        if o.timed_out {
            c.log
                .push_str("Cargo took longer than 5 minutes and was stopped.\n");
        }
        Ok(c)
    }

    /// Runs a compiled program with a clean environment plus `env`.
    #[allow(clippy::too_many_arguments)]
    pub fn run_program(
        &self,
        exe: &Path,
        cwd: &Path,
        stdin: &[u8],
        args: &[String],
        env: &[(String, String)],
        limits: &Limits,
        cancel: Option<&AtomicBool>,
    ) -> Result<Output> {
        let mut cmd = Command::new(exe);
        cmd.current_dir(cwd).args(args).env_clear();
        cmd.env("RUST_BACKTRACE", "0");
        if let Some(p) = std::env::var_os("PATH") {
            cmd.env("PATH", p);
        }
        cmd.envs(env.iter().map(|(k, v)| (k, v)));
        process::run(cmd, stdin, limits, cancel)
            .with_context(|| format!("running {}", exe.display()))
    }
}

/// Cargo `--config` arguments that replace crates.io with a vendored folder.
pub fn vendor_config(vendor: &Path) -> Vec<String> {
    let dir = vendor.to_string_lossy().replace('\\', "/");
    vec![
        "--config".into(),
        "source.crates-io.replace-with='rusticoder-vendor'".into(),
        "--config".into(),
        format!("source.rusticoder-vendor.directory='{dir}'"),
    ]
}

/// The same source replacement as a `.cargo/config.toml`, for tools started without
/// our arguments (rust-analyzer).
pub fn vendor_config_toml(vendor: &Path) -> String {
    let dir = vendor.to_string_lossy().replace('\\', "/");
    format!(
        "[source.crates-io]\nreplace-with = \"rusticoder-vendor\"\n\n[source.rusticoder-vendor]\ndirectory = '{dir}'\n\n[net]\noffline = true\n"
    )
}

/// The result of a compile: success, diagnostics and the rendered output to show.
#[derive(Debug, Clone, Default, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Compiled {
    pub ok: bool,
    pub diagnostics: Vec<Diagnostic>,
    /// Rendered compiler output with ANSI colours, exactly as rustc prints it.
    pub rendered: String,
    /// Other tool output (Cargo progress, unparsed lines).
    pub log: String,
    #[ts(type = "string | null")]
    pub executable: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Diagnostic {
    /// error | warning | …
    pub level: String,
    pub code: Option<String>,
    pub message: String,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub column: Option<u32>,
}

impl Compiled {
    fn push(&mut self, v: &serde_json::Value) {
        if let Some(r) = v["rendered"].as_str() {
            self.rendered.push_str(r);
        }
        let level = v["level"].as_str().unwrap_or("").to_string();
        if level.is_empty() || level == "failure-note" {
            return;
        }
        let primary = v["spans"].as_array().and_then(|s| {
            s.iter()
                .find(|s| s["is_primary"].as_bool() == Some(true))
                .cloned()
        });
        self.diagnostics.push(Diagnostic {
            level,
            code: v["code"]["code"].as_str().map(str::to_string),
            message: v["message"].as_str().unwrap_or("").to_string(),
            file: primary
                .as_ref()
                .and_then(|s| s["file_name"].as_str().map(str::to_string)),
            line: primary
                .as_ref()
                .and_then(|s| s["line_start"].as_u64())
                .map(|n| n as u32),
            column: primary
                .as_ref()
                .and_then(|s| s["column_start"].as_u64())
                .map(|n| n as u32),
        });
    }

    pub fn errors(&self) -> impl Iterator<Item = &Diagnostic> {
        self.diagnostics.iter().filter(|d| d.level == "error")
    }
}

/// Removes ANSI escape sequences.
pub fn strip_ansi(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut it = s.chars().peekable();
    while let Some(c) = it.next() {
        if c == '\u{1b}' && it.peek() == Some(&'[') {
            it.next();
            for c in it.by_ref() {
                if c.is_ascii_alphabetic() {
                    break;
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiles_and_reports_errors() {
        let tc = Toolchain::detect().unwrap();
        let dir = std::env::temp_dir().join(format!("rb-runner-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let src = dir.join("main.rs");
        std::fs::write(&src, "fn main() {\n    println(\"hi\");\n}\n").unwrap();
        let c = tc
            .compile_rustc(&src, &dir.join("main"), "2024", &[])
            .unwrap();
        assert!(!c.ok);
        let e = c.errors().next().unwrap();
        assert_eq!(e.code.as_deref(), Some("E0423"));
        assert_eq!(e.line, Some(2));
        assert!(strip_ansi(&c.rendered).contains("expected function, found macro"));

        std::fs::write(&src, "fn main() {\n    println!(\"hi\");\n}\n").unwrap();
        let c = tc
            .compile_rustc(&src, &dir.join("main"), "2024", &[])
            .unwrap();
        assert!(c.ok, "{}", c.rendered);
        let out = tc
            .run_program(
                &dir.join("main"),
                &dir,
                b"",
                &[],
                &[],
                &Limits::default(),
                None,
            )
            .unwrap();
        assert_eq!(out.stdout, b"hi\n");
        std::fs::remove_dir_all(&dir).ok();
    }
}
