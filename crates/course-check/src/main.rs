//! course-check: import chapters from the course plan, validate them, bundle them for the app.

mod book;
mod dynamic;
mod source;
mod suites;
mod validate;

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use rb_specparse::{Report, course::Plan};
use serde_json::json;
use source::Source;
use std::path::PathBuf;

#[derive(Parser)]
#[command(about = "Import, validate and bundle Rusticoder course content")]
struct Cli {
    /// The course plan folder (the reviewed Markdown spec).
    #[arg(long, default_value = "../rust-basics-course-plan")]
    plan: PathBuf,
    /// The authored content folder.
    #[arg(long, default_value = "content/rust-basics")]
    content: PathBuf,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Regenerate the generated layer (`*.gen.json`) of chapters from the plan.
    Import { chapters: Vec<u32> },
    /// Check chapters: static rules, and with --dynamic compile and run everything.
    Validate {
        chapters: Vec<u32>,
        #[arg(long)]
        dynamic: bool,
        /// Check only these problems (comma-separated ids), without quizzes and lessons.
        #[arg(long, value_delimiter = ',')]
        only: Vec<String>,
    },
    /// Copy the Rust Book's chapters (from a clone of rust-lang/book) into the content folder.
    Book { from: PathBuf },
    /// Write the merged course into the app's resources.
    Bundle {
        #[arg(long, default_value = "app/src-tauri/resources/content")]
        out: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let src = Source::new(&cli.content);
    match cli.cmd {
        Cmd::Import { chapters } => import(&cli.plan, &src, &chapters),
        Cmd::Validate {
            chapters,
            dynamic,
            only,
        } => {
            let chapters = if chapters.is_empty() {
                src.chapters_present()
            } else {
                chapters
            };
            let ok = validate::run(&src, &chapters, dynamic, &only)?;
            if !ok {
                std::process::exit(1);
            }
            Ok(())
        }
        Cmd::Book { from } => {
            let n = book::import(&from, &src.dir.join("book"))?;
            println!(
                "copied {n} book chapters into {}",
                src.dir.join("book").display()
            );
            Ok(())
        }
        Cmd::Bundle { out } => bundle(&src, &out),
    }
}

fn import(plan_dir: &std::path::Path, src: &Source, chapters: &[u32]) -> Result<()> {
    let plan = Plan::open(plan_dir)?;
    std::fs::create_dir_all(&src.dir)?;
    let course = json!({
        "parts": plan.parts,
        "concepts": plan.concepts,
        "chapters": plan.chapters.iter().map(|c| json!({
            "num": c.num, "id": format!("ch{:02}", c.num), "title": c.title, "part": c.part, "available": false,
        })).collect::<Vec<_>>(),
    });
    std::fs::write(
        src.dir.join("course.gen.json"),
        serde_json::to_string_pretty(&course)?,
    )?;
    if chapters.is_empty() {
        bail!("name the chapters to import, e.g. `course-check import 1`");
    }
    let mut failed = Vec::new();
    for &n in chapters {
        let mut report = Report::default();
        let ch = plan
            .chapter(n, &mut report)
            .with_context(|| format!("importing chapter {n}"))?;
        let todos: usize = ch
            .problems
            .iter()
            .flat_map(|p| &p.tests)
            .filter(|t| validate::is_todo(t))
            .count();
        println!(
            "chapter {n}: {} lessons, {} question sets ({} questions), {} problems, {} test rows ({todos} to author)",
            ch.lessons.len(),
            ch.question_sets.len(),
            ch.question_sets
                .iter()
                .map(|s| s.questions.len())
                .sum::<usize>(),
            ch.problems.len(),
            ch.problems.iter().map(|p| p.tests.len()).sum::<usize>(),
        );
        for w in &report.warnings {
            println!("  warn  {w}");
        }
        for e in &report.errors {
            println!("  ERROR {e}");
        }
        // A spec the importer couldn't read in full must not replace the generated layer.
        if !report.errors.is_empty() {
            println!(
                "  chapter {n} not written: {} import errors",
                report.errors.len()
            );
            failed.push(n);
            continue;
        }
        let mut ch = ch;
        format_starters(&mut ch);
        let dir = src.chapter_dir(n);
        std::fs::create_dir_all(&dir)?;
        std::fs::write(
            dir.join("chapter.gen.json"),
            serde_json::to_string_pretty(&ch)?,
        )?;
    }
    if !failed.is_empty() {
        bail!("import failed for chapter(s) {failed:?}");
    }
    Ok(())
}

/// Lays out starter and solution code with rustfmt (the plan writes some of it without indentation).
/// Code rustfmt can't parse (a syntax-repair task) is left as written.
fn format_starters(ch: &mut rb_content::Chapter) {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let rustfmt = std::env::var_os("HOME")
        .map(|h| PathBuf::from(h).join(".cargo/bin/rustfmt"))
        .filter(|p| p.exists())
        .unwrap_or_else(|| PathBuf::from("rustfmt"));
    let format = |text: &str| -> Option<String> {
        let mut child = Command::new(&rustfmt)
            .args(["--edition", "2024", "--emit", "stdout"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .ok()?;
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(text.as_bytes());
        }
        let o = child.wait_with_output().ok()?;
        if !o.status.success() {
            return None;
        }
        String::from_utf8(o.stdout).ok()
    };
    for p in &mut ch.problems {
        // Test-writing problems: the supplied code and the learner's part are laid out
        // on their own, so the starter still begins with the supplied code exactly.
        if !p.subject.is_empty() {
            let (Ok(starter), Ok(solution)) =
                (p.learner_parts(&p.starter), p.learner_parts(&p.solution))
            else {
                continue;
            };
            let supplied: Vec<String> = p.subject.iter().map(|s| s.path.clone()).collect();
            let tidy = |files: Vec<rb_content::SourceFile>| -> Vec<rb_content::SourceFile> {
                files
                    .into_iter()
                    .map(|mut f| {
                        if f.path.ends_with(".rs")
                            && !f.text.trim().is_empty()
                            && let Some(t) = format(&f.text)
                        {
                            f.text = t;
                        }
                        f
                    })
                    .filter(|f| !(f.text.trim().is_empty() && supplied.contains(&f.path)))
                    .collect()
            };
            p.subject = tidy(p.subject.clone());
            p.starter = rb_content::with_subject(&p.subject, tidy(starter));
            p.solution = rb_content::with_subject(&p.subject, tidy(solution));
            continue;
        }
        for f in p
            .starter
            .iter_mut()
            .chain(p.solution.iter_mut())
            .filter(|f| f.path.ends_with(".rs"))
        {
            let Ok(mut child) = Command::new(&rustfmt)
                .args(["--edition", "2024", "--emit", "stdout"])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()
            else {
                return;
            };
            if let Some(mut stdin) = child.stdin.take() {
                let _ = stdin.write_all(f.text.as_bytes());
            }
            if let Ok(o) = child.wait_with_output()
                && o.status.success()
                && let Ok(text) = String::from_utf8(o.stdout)
            {
                f.text = text;
            }
        }
    }
}

fn copy_dir(from: &std::path::Path, to: &std::path::Path) -> Result<usize> {
    std::fs::create_dir_all(to)?;
    let mut n = 0;
    for e in std::fs::read_dir(from)? {
        let p = e?.path();
        let dest = to.join(p.file_name().unwrap());
        if p.is_dir() {
            n += copy_dir(&p, &dest)?;
        } else {
            std::fs::copy(&p, &dest)?;
            n += 1;
        }
    }
    Ok(n)
}

fn bundle(src: &Source, out: &std::path::Path) -> Result<()> {
    let course = src.course()?;
    let dir = out.join(&course.id);
    if dir.exists() {
        std::fs::remove_dir_all(&dir)?;
    }
    std::fs::create_dir_all(&dir)?;
    for n in src.chapters_present() {
        let mut ch = src.chapter(n)?;
        dynamic::fill_suites(&mut ch, src.vendor_dir().as_deref())?;
        let errors = validate::static_errors(&course, &ch);
        if !errors.is_empty() {
            for e in &errors {
                eprintln!("ERROR {e}");
            }
            bail!(
                "chapter {n} has {} errors; fix them before bundling",
                errors.len()
            );
        }
        std::fs::write(
            dir.join(format!("{}.json", ch.id)),
            serde_json::to_string(&ch)?,
        )?;
        println!("bundled {}", ch.id);
    }
    std::fs::write(dir.join("course.json"), serde_json::to_string(&course)?)?;
    // Vendored crates for offline Cargo builds.
    if let Some(v) = src.vendor_dir() {
        let n = copy_dir(&v, &dir.join("vendor"))?;
        println!("bundled {n} vendored crate files");
    }
    // The Rust Book for "Read more", shared by every course.
    let book_src = src.dir.join("book");
    if book_src.exists() {
        let book_out = out.parent().unwrap_or(out).join("book");
        if book_out.exists() {
            std::fs::remove_dir_all(&book_out)?;
        }
        std::fs::create_dir_all(&book_out)?;
        let mut n = 0;
        for e in std::fs::read_dir(&book_src)? {
            let p = e?.path();
            if p.is_file() {
                std::fs::copy(&p, book_out.join(p.file_name().unwrap()))?;
                n += 1;
            }
        }
        println!("bundled {n} book files");
    }
    println!("wrote {}", dir.display());
    Ok(())
}
