//! Dynamic checks: compile and run every solution, starter, wrong variant, quiz
//! program and lesson program with the real toolchain.

use anyhow::Result;
use rb_content::*;
use rb_grader::{Grader, Mode, TestStatus, Verdict};
use rb_runner::process::Limits;
use rb_runner::{Toolchain, strip_ansi};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// Scratch space for one course-check process; removed when dropped.
struct Work(PathBuf);

impl Work {
    fn new() -> Work {
        Work(std::env::temp_dir().join(format!("course-check-{}", std::process::id())))
    }
}

impl Drop for Work {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn grader<'a>(tc: &'a Toolchain, work: &Work, vendor: Option<&Path>) -> Grader<'a> {
    Grader {
        toolchain: tc,
        edition: "2024",
        work_root: work.0.join("grade"),
        shared_target: Some(work.0.join("target")),
        vendor: vendor.map(Path::to_path_buf),
        cancel: None,
    }
}

/// Fills generated suites with their cases and the reference solution's output.
pub fn fill_suites(ch: &mut Chapter, vendor: Option<&Path>) -> Result<()> {
    let tc = Toolchain::detect()?;
    let work = Work::new();
    let n = crate::suites::fill(ch, &grader(&tc, &work, vendor))?;
    if n > 0 {
        println!("   {}: filled {n} generated suites", ch.id);
    }
    Ok(())
}

pub fn run(ch: &Chapter, vendor: Option<&Path>) -> Result<bool> {
    let tc = Toolchain::detect()?;
    println!(
        "   toolchain: rustc {} ({})",
        tc.rustc_version,
        tc.rustc.display()
    );
    let work = Work::new();
    let grader = grader(&tc, &work, vendor);
    let mut ch = ch.clone();
    crate::suites::fill(&mut ch, &grader)?;
    let ch = &ch;
    let mut errors = Vec::new();

    // Problems in parallel, reported in order.
    // A quarter of the cores: busier machines make the 1 s test limits flaky.
    let threads = std::thread::available_parallelism().map_or(2, |n| (n.get() / 4).max(2));
    let next = std::sync::atomic::AtomicUsize::new(0);
    let results: std::sync::Mutex<Vec<Option<Result<ProblemReport>>>> =
        std::sync::Mutex::new((0..ch.problems.len()).map(|_| None).collect());
    std::thread::scope(|s| {
        for _ in 0..threads {
            s.spawn(|| {
                loop {
                    let i = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    let Some(p) = ch.problems.get(i) else { break };
                    let r = check_problem(&grader, p);
                    results.lock().unwrap()[i] = Some(r);
                }
            });
        }
    });
    for r in results.into_inner().unwrap().into_iter().flatten() {
        let (errs, line) = r?;
        println!("{line}");
        errors.extend(errs);
    }

    // Quiz programs.
    let qdir = work.0.join("quiz");
    std::fs::create_dir_all(&qdir)?;
    let questions = ch
        .question_sets
        .iter()
        .flat_map(|s| s.questions.iter())
        .chain(
            ch.lessons
                .iter()
                .flat_map(|l| l.screens.iter().filter_map(|s| s.question.as_ref())),
        );
    let mut quiz_count = 0;
    for q in questions {
        let Some(v) = &q.verify else { continue };
        let r = match (&q.code, q.files.is_empty()) {
            (Some(code), _) => verify_program(&tc, vendor, &qdir, &q.id, &code.text, v),
            (None, false) => verify_package(&tc, vendor, &qdir, &q.id, &q.files, v),
            (None, true) => continue,
        };
        quiz_count += 1;
        if let Err(e) = r {
            errors.push(format!("{}: {e}", q.id));
        }
    }

    // Lesson programs: complete Rust programs must compile; error screens must fail with the shown message.
    let mut lesson_count = 0;
    for l in &ch.lessons {
        for s in &l.screens {
            if !s.files.is_empty() {
                // A package: it builds, or fails with the error screen's first line.
                lesson_count += 1;
                let id = format!("{}-{}", l.id, s.key);
                let v = match (&s.kind, &s.diagnostic) {
                    (ScreenKind::Error | ScreenKind::Stepper, Some(diag)) => {
                        let first = diag.lines().next().unwrap_or("").to_string();
                        Verify {
                            compiles: false,
                            error_code: first
                                .split(['[', ']'])
                                .nth(1)
                                .map(str::to_string)
                                .filter(|c| c.starts_with('E')),
                            error_line: None,
                            error_message: Some(first),
                            stdout: None,
                            bin: None,
                        }
                    }
                    _ => Verify {
                        compiles: true,
                        error_code: None,
                        error_line: None,
                        error_message: None,
                        stdout: None,
                        bin: None,
                    },
                };
                if let Err(e) = verify_package(&tc, vendor, &qdir, &id, &s.files, &v) {
                    errors.push(format!("{id}: {e}"));
                }
                continue;
            }
            let Some(code) = &s.code else { continue };
            if code.lang != "rust" || !code.text.contains("fn main") {
                continue;
            }
            lesson_count += 1;
            let id = format!("{}-{}", l.id, s.key);
            let result = match (&s.kind, &s.diagnostic) {
                // A runtime error screen: it compiles, then panics with the shown message.
                (ScreenKind::Error | ScreenKind::Stepper, Some(diag))
                    if diag.contains("panicked at") =>
                {
                    verify_panic(&tc, &qdir, &id, &code.text, diag)
                }
                // A warning screen: it compiles, with the shown warning.
                (ScreenKind::Error | ScreenKind::Stepper, Some(diag))
                    if diag.starts_with("warning:") =>
                {
                    verify_warning(&tc, &qdir, &id, &code.text, diag)
                }
                (ScreenKind::Error | ScreenKind::Stepper, Some(diag)) => {
                    let first = diag.lines().next().unwrap_or("").to_string();
                    let code_id = first
                        .split(['[', ']'])
                        .nth(1)
                        .map(str::to_string)
                        .filter(|c| c.starts_with('E'));
                    verify_program(
                        &tc,
                        vendor,
                        &qdir,
                        &id,
                        &code.text,
                        &Verify {
                            compiles: false,
                            error_code: code_id,
                            error_line: None,
                            error_message: Some(first),
                            stdout: None,
                            bin: None,
                        },
                    )
                }
                _ => verify_program(
                    &tc,
                    vendor,
                    &qdir,
                    &id,
                    &code.text,
                    &Verify {
                        compiles: true,
                        error_code: None,
                        error_line: None,
                        error_message: None,
                        stdout: None,
                        bin: None,
                    },
                ),
            };
            if let Err(e) = result {
                errors.push(format!("{id}: {e}"));
            }
        }
    }
    println!(
        "   {} problems, {quiz_count} quiz programs, {lesson_count} lesson programs checked",
        ch.problems.len()
    );
    for e in &errors {
        println!("  ERROR {e}");
    }
    Ok(errors.is_empty())
}

/// Compiles `code`, runs it, and checks it panics with the message in `diag` (the line
/// after "panicked at …").
fn verify_warning(
    tc: &Toolchain,
    work: &Path,
    id: &str,
    code: &str,
    diag: &str,
) -> std::result::Result<(), String> {
    let dir = work.join(id);
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let src = dir.join("main.rs");
    std::fs::write(&src, code).map_err(|e| e.to_string())?;
    let c = tc
        .compile_rustc(&src, &dir.join("main"), "2024", &[])
        .map_err(|e| e.to_string())?;
    let rendered = strip_ansi(&c.rendered);
    let want = diag.lines().next().unwrap_or("");
    if !c.ok {
        return Err(format!(
            "should compile with a warning, but doesn't:\n{rendered}"
        ));
    }
    if !rendered.contains(want) {
        return Err(format!(
            "should warn {want:?}, but rustc printed {rendered:?}"
        ));
    }
    Ok(())
}

fn verify_panic(
    tc: &Toolchain,
    work: &Path,
    id: &str,
    code: &str,
    diag: &str,
) -> std::result::Result<(), String> {
    let dir = work.join(id);
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let src = dir.join("main.rs");
    std::fs::write(&src, code).map_err(|e| e.to_string())?;
    let exe = dir.join("main");
    let c = tc
        .compile_rustc(&src, &exe, "2024", &["-A", "warnings"])
        .map_err(|e| e.to_string())?;
    if !c.ok {
        return Err(format!(
            "should compile and then panic, but doesn't compile:\n{}",
            strip_ansi(&c.rendered)
        ));
    }
    let o = tc
        .run_program(&exe, &dir, b"", &[], &[], &Limits::default(), None)
        .map_err(|e| e.to_string())?;
    let want = diag
        .lines()
        .skip_while(|l| !l.contains("panicked at"))
        .nth(1)
        .unwrap_or("")
        .trim();
    if o.success() {
        return Err("should panic, but runs successfully".into());
    }
    if !o.stderr_str().contains(want) {
        return Err(format!(
            "should panic with {want:?}, but stderr is {:?}",
            o.stderr_str()
        ));
    }
    Ok(())
}

/// Programs that use `rand` are built as a Cargo package with the vendored crates.
fn uses_rand(code: &str) -> bool {
    code.contains("rand::")
}

fn verify_program(
    tc: &Toolchain,
    vendor: Option<&Path>,
    dir: &std::path::Path,
    id: &str,
    code: &str,
    v: &Verify,
) -> std::result::Result<(), String> {
    // Each program is compiled as `main.rs` in its own folder, so the crate is named `main`
    // exactly as in the lessons (`rustc main.rs`).
    let dir = dir.join(id.replace('.', "_"));
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let dir = dir.as_path();
    let (c, exe) = if uses_rand(code) {
        std::fs::create_dir_all(dir.join("src")).map_err(|e| e.to_string())?;
        std::fs::write(
            dir.join("Cargo.toml"),
            "[package]\nname = \"main\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nrand = \"0.10.1\"\n",
        )
        .map_err(|e| e.to_string())?;
        std::fs::write(dir.join("src/main.rs"), code).map_err(|e| e.to_string())?;
        let c = tc
            .cargo(dir, "build", Some(&dir.join("target")), vendor, None)
            .map_err(|e| e.to_string())?;
        let exe = c.executable.clone().unwrap_or_default();
        (c, exe)
    } else {
        let src = dir.join("main.rs");
        std::fs::write(&src, code).map_err(|e| e.to_string())?;
        let exe: PathBuf = dir.join("main");
        let c = tc
            .compile_rustc(&src, &exe, "2024", &["-A", "warnings"])
            .map_err(|e| e.to_string())?;
        (c, exe)
    };
    check_build(tc, &c, &exe, dir, v)
}

/// A multi-file program: a Cargo package from these files (a `Cargo.toml` among them, or
/// the package `module-lab`, or for a library and a binary the crate `src/main.rs` uses,
/// as `use aggregator::Summary;` names `aggregator`). Its output is the package's own binary's.
fn verify_package(
    tc: &Toolchain,
    vendor: Option<&Path>,
    dir: &std::path::Path,
    id: &str,
    files: &[CodeBlock],
    v: &Verify,
) -> std::result::Result<(), String> {
    let dir = dir.join(id.replace('.', "_"));
    let _ = std::fs::remove_dir_all(dir.join("src"));
    std::fs::create_dir_all(dir.join("src")).map_err(|e| e.to_string())?;
    let package = lib_crate_name(files).unwrap_or_else(|| "module-lab".into());
    let mut manifest = format!(
        "[package]\nname = \"{package}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\n"
    );
    for f in files {
        let Some(path) = f.file.as_deref() else {
            return Err("a file of this package has no path".into());
        };
        if path == "Cargo.toml" {
            manifest = f.text.clone();
            continue;
        }
        let p = dir.join(path);
        if let Some(parent) = p.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        std::fs::write(&p, &f.text).map_err(|e| e.to_string())?;
    }
    std::fs::write(dir.join("Cargo.toml"), &manifest).map_err(|e| e.to_string())?;
    let name = toml::from_str::<toml::Value>(&manifest)
        .ok()
        .and_then(|m| m["package"]["name"].as_str().map(str::to_string))
        .unwrap_or_else(|| "module-lab".into());
    // One target folder for every package: they share no dependencies, only build time.
    let target = dir.parent().unwrap_or(&dir).join("target");
    let c = tc
        .cargo(&dir, "build", Some(&target), vendor, None)
        .map_err(|e| e.to_string())?;
    let exe = target.join("debug").join(v.bin.as_deref().unwrap_or(&name));
    check_build(tc, &c, &exe, &dir, v)
}

/// The library crate a package's `src/main.rs` imports from, when it has a `src/lib.rs`.
fn lib_crate_name(files: &[CodeBlock]) -> Option<String> {
    let file = |p: &str| files.iter().find(|f| f.file.as_deref() == Some(p));
    file("src/lib.rs")?;
    file("src/main.rs")?.text.lines().find_map(|l| {
        let root = l.trim().strip_prefix("use ")?.split("::").next()?;
        let taken = ["std", "core", "alloc", "crate", "self", "super"].contains(&root);
        (!taken && root.chars().all(|c| c.is_alphanumeric() || c == '_')).then(|| root.to_string())
    })
}

/// Checks a build against the expected facts, running the program for its output.
fn check_build(
    tc: &Toolchain,
    c: &rb_runner::Compiled,
    exe: &Path,
    dir: &Path,
    v: &Verify,
) -> std::result::Result<(), String> {
    if c.ok != v.compiles {
        return Err(if c.ok {
            "compiles, but the answer says it doesn't".into()
        } else {
            format!(
                "doesn't compile, but the answer says it does:\n{}",
                strip_ansi(&c.rendered)
            )
        });
    }
    if !c.ok {
        let first = c.errors().next();
        if let (Some(want), Some(e)) = (&v.error_code, first)
            && e.code.as_deref() != Some(want.as_str())
        {
            return Err(format!("expected {want}, rustc reports {:?}", e.code));
        }
        if let (Some(want), Some(e)) = (v.error_line, first)
            && e.line != Some(want)
        {
            return Err(format!(
                "expected the error on line {want}, rustc reports line {:?}",
                e.line
            ));
        }
        if let Some(want) = &v.error_message {
            // The first error's headline (a package build may print warnings first).
            let got = strip_ansi(&c.rendered);
            let got_first = got
                .lines()
                .find(|l| l.starts_with("error"))
                .or_else(|| got.lines().next())
                .unwrap_or("");
            // A spec message in a code span can't hold rustc's own backticks.
            if got_first.trim() != want.trim() && got_first.replace('`', "").trim() != want.trim() {
                return Err(format!(
                    "expected the message {want:?}, rustc says {got_first:?}"
                ));
            }
        }
        return Ok(());
    }
    if let Some(want) = &v.stdout {
        let out = tc
            .run_program(exe, dir, b"", &[], &[], &Limits::default(), None)
            .map_err(|e| e.to_string())?;
        let norm = |s: &str| {
            s.lines()
                .map(str::trim_end)
                .collect::<Vec<_>>()
                .join("\n")
                .trim_end()
                .to_string()
        };
        if norm(&out.stdout_str()) != norm(want) {
            return Err(format!(
                "expected output {want:?}, the program prints {:?}",
                out.stdout_str()
            ));
        }
    }
    Ok(())
}

/// A problem's errors and its summary line.
type ProblemReport = (Vec<String>, String);

/// Checks one problem: its solution passes, its starter doesn't, its wrong variants fail
/// as declared. Returns the errors and the summary line.
fn check_problem(grader: &Grader, p: &Problem) -> Result<ProblemReport> {
    let mut errors = Vec::new();
    // The reference solution passes everything (each part's, for checkpoint problems).
    let parts: Vec<Option<u32>> = if p.checkpoints {
        (1..=p.parts.len() as u32).map(Some).collect()
    } else {
        vec![None]
    };
    for part in parts {
        let last = part.is_none_or(|n| n as usize == p.parts.len());
        let files = match part {
            Some(n) if !last => match p.part_solutions.get(&n) {
                Some(f) => f,
                None => {
                    errors.push(format!("{}: no reference solution for part {n}", p.id));
                    continue;
                }
            },
            _ => &p.solution,
        };
        let r = grader.grade(p, files, Mode::Submit, part, |_| {})?;
        if r.verdict != Verdict::Accepted {
            let failed: Vec<String> = r
                .tests
                .iter()
                .filter(|t| t.status != TestStatus::Passed)
                .map(|t| format!("#{} {}", t.n, t.message.clone().unwrap_or_default()))
                .collect();
            errors.push(format!(
                "{}{}: solution is {:?}: {}{}",
                p.id,
                part.map(|n| format!(" part {n}")).unwrap_or_default(),
                r.verdict,
                failed.join("; "),
                if r.verdict == Verdict::CompileError {
                    format!("{}{}", strip_ansi(&r.compile.rendered), r.compile.log)
                } else {
                    String::new()
                }
            ));
        }
    }
    // The starter doesn't.
    let r = grader.grade(p, &p.starter, Mode::Submit, None, |_| {})?;
    if r.verdict == Verdict::Accepted {
        errors.push(format!(
            "{}: the starter code already passes every test",
            p.id
        ));
    }
    if r.verdict == Verdict::CompileError && p.kind != "fix-compile" && p.starter_stage == "Guided"
    {
        errors.push(format!(
            "{}: Guided starter doesn't compile: {}",
            p.id,
            strip_ansi(&r.compile.rendered)
        ));
    }
    // Each wrong variant fails exactly the tests it names.
    for w in &p.wrong {
        let mut files = p.starter.clone();
        for f in &w.files {
            match files.iter_mut().find(|s| s.path == f.path) {
                Some(s) => s.text = f.text.clone(),
                None => files.push(f.clone()),
            }
        }
        let r = grader.grade(p, &files, Mode::Submit, w.part, |_| {})?;
        let failed: BTreeSet<String> = r
            .tests
            .iter()
            .filter(|t| t.status != TestStatus::Passed)
            .map(|t| t.key.clone())
            .collect();
        let want: BTreeSet<String> = w.fails.iter().cloned().collect();
        let unexplained: BTreeSet<&String> = failed
            .iter()
            .filter(|k| !want.contains(*k) && !w.may_fail.contains(k))
            .collect();
        if !want.is_subset(&failed) || !unexplained.is_empty() {
            errors.push(format!(
                "{} wrong `{}`: expected failures {want:?}, got {failed:?}",
                p.id, w.name
            ));
        }
    }
    let line = format!(
        "   {} ok-checked ({} tests, {} wrong variants)",
        p.id,
        p.tests.len(),
        p.wrong.len()
    );
    Ok((errors, line))
}
