//! Grade a problem from the terminal: `rb-cli grade P01-06 path/to/folder [--submit]`.

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use rb_content::{Bundle, SourceFile};
use rb_grader::{GradeEvent, Grader, Mode, TestStatus};
use rb_runner::{Toolchain, strip_ansi};
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    /// The bundled course folder.
    #[arg(long, default_value = "app/src-tauri/resources/content/rust-basics")]
    bundle: PathBuf,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Grade the files in a folder against a problem.
    Grade {
        problem: String,
        /// Folder with the files (`main.rs`, or `Cargo.toml` + `src/`).
        dir: PathBuf,
        /// Run every test, not only the visible ones.
        #[arg(long)]
        submit: bool,
        /// Checkpoint problems: the part to grade (default: the last).
        #[arg(long)]
        part: Option<u32>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let Cmd::Grade {
        problem,
        dir,
        submit,
        part,
    } = cli.cmd;
    let course = Bundle::course(&cli.bundle)?;
    let p = course
        .chapters
        .iter()
        .filter(|c| c.available)
        .filter_map(|c| Bundle::chapter(&cli.bundle, &c.id).ok())
        .find_map(|ch| ch.problem(&problem).cloned())
        .with_context(|| format!("no problem {problem} in the bundle"))?;

    let mut files = Vec::new();
    for f in &p.starter {
        let path = dir.join(&f.path);
        let text = std::fs::read_to_string(&path)
            .with_context(|| format!("reading {}", path.display()))?;
        files.push(SourceFile {
            path: f.path.clone(),
            text,
        });
    }
    if files.is_empty() {
        bail!("{problem} has no files");
    }
    let tc = Toolchain::detect()?;
    let grader = Grader {
        toolchain: &tc,
        edition: &course.toolchain.edition,
        work_root: std::env::temp_dir().join("rb-cli"),
        shared_target: Some(std::env::temp_dir().join("rb-cli-target")),
        vendor: Some(cli.bundle.join("vendor")).filter(|v| v.is_dir()),
        cancel: None,
    };
    let mode = if submit { Mode::Submit } else { Mode::Run };
    let r = grader.grade(&p, &files, mode, part, |e| match e {
        GradeEvent::Phase { text } => eprintln!("… {text}"),
        GradeEvent::Compiled { compile } => {
            let out = strip_ansi(&compile.rendered);
            if !out.is_empty() {
                eprint!("{out}");
            }
        }
        GradeEvent::Test { result } => {
            let mark = if result.status == TestStatus::Passed {
                "PASS"
            } else {
                "FAIL"
            };
            println!(
                "{mark} #{:<2} {:<10} {}",
                result.n,
                result.category,
                result.message.as_deref().unwrap_or("")
            );
        }
    })?;
    println!("{:?} in {} ms", r.verdict, r.ms);
    Ok(())
}
