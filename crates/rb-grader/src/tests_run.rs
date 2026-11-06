//! Test-writing problems: `cargo test` on the subject (or a mutant of it) with the
//! learner's tests, each test binary run on its own with a time limit.

use crate::{Grader, write_files};
use anyhow::Result;
use rb_content::{Problem, SourceFile};
use rb_runner::Compiled;
use rb_runner::process::{Limits, Output};
use std::path::Path;
use std::time::Duration;

/// What `cargo test` did.
pub(crate) struct TestsObs {
    pub build: Compiled,
    pub targets: Vec<TargetRun>,
}

/// One test binary's run: the unit tests of `src/lib.rs`, or an integration test file.
pub(crate) struct TargetRun {
    /// `src/lib.rs`, `tests/shelf.rs`
    pub label: String,
    pub out: Output,
    pub tests: Vec<(String, Outcome)>,
    /// From the binary's summary line, when it printed one.
    pub summary: Option<Summary>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Outcome {
    Passed,
    Failed,
    Ignored,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct Summary {
    pub passed: u32,
    pub failed: u32,
    pub ignored: u32,
    pub filtered: u32,
}

impl TestsObs {
    /// Tests that ran: passed or failed.
    pub fn ran(&self) -> u32 {
        self.targets
            .iter()
            .map(|t| match t.summary {
                Some(s) => s.passed + s.failed,
                None => t.count(Outcome::Passed) + t.count(Outcome::Failed),
            })
            .sum()
    }

    pub fn ignored(&self) -> u32 {
        self.targets
            .iter()
            .map(|t| t.summary.map_or(t.count(Outcome::Ignored), |s| s.ignored))
            .sum()
    }

    pub fn filtered(&self) -> u32 {
        self.targets
            .iter()
            .map(|t| t.summary.map_or(0, |s| s.filtered))
            .sum()
    }

    /// Every test's full name and outcome, prefixed with its target when there are several.
    pub fn all(&self) -> impl Iterator<Item = (&TargetRun, &str, Outcome)> {
        self.targets
            .iter()
            .flat_map(|t| t.tests.iter().map(move |(n, o)| (t, n.as_str(), *o)))
    }

    /// A test binary that failed: a test failed, it ran out of time, or it crashed.
    pub fn failing(&self) -> Vec<&TargetRun> {
        self.targets.iter().filter(|t| t.failed()).collect()
    }
}

impl TargetRun {
    fn count(&self, o: Outcome) -> u32 {
        self.tests.iter().filter(|(_, x)| *x == o).count() as u32
    }

    pub fn failed(&self) -> bool {
        self.out.timed_out
            || self.out.output_limit
            || self.count(Outcome::Failed) > 0
            || self.summary.is_some_and(|s| s.failed > 0)
            || self.out.code != Some(0)
    }

    /// What libtest printed about the failures (the `failures:` section), or the end of
    /// its output.
    pub fn report(&self) -> String {
        let out = self.out.stdout_str();
        let body = match out.find("\nfailures:\n") {
            Some(i) => out[i + 1..].to_string(),
            None => out,
        };
        let mut text: String = body
            .lines()
            .filter(|l| !l.starts_with("note: run with `RUST_BACKTRACE=1`"))
            .map(strip_finished)
            .collect::<Vec<_>>()
            .join("\n");
        let err = self.out.stderr_str();
        if !err.trim().is_empty() {
            text.push_str(&format!("\n{}", err.trim_end()));
        }
        text.trim().to_string()
    }
}

/// Drops the `; finished in 0.00s` timing, which differs from run to run.
fn strip_finished(line: &str) -> String {
    match line.find("; finished in ") {
        Some(i) if line.starts_with("test result:") => line[..i].to_string(),
        _ => line.to_string(),
    }
}

/// The `test NAME ... ok|FAILED|ignored` lines and the `test result:` summary.
pub(crate) fn parse_libtest(stdout: &str) -> (Vec<(String, Outcome)>, Option<Summary>) {
    let mut tests = Vec::new();
    let mut summary = None;
    for line in stdout.lines() {
        if let Some(rest) = line.strip_prefix("test result: ") {
            let mut s = Summary::default();
            for part in rest.split(';') {
                let words: Vec<&str> = part.split_whitespace().collect();
                // "ok. 3 passed" | "1 failed" | "0 ignored" | "2 filtered out"
                let n = words.iter().rev().nth(if part.contains("filtered") { 2 } else { 1 });
                let Some(n) = n.and_then(|n| n.parse::<u32>().ok()) else {
                    continue;
                };
                if part.contains("passed") {
                    s.passed = n;
                } else if part.contains("failed") {
                    s.failed = n;
                } else if part.contains("ignored") {
                    s.ignored = n;
                } else if part.contains("filtered out") {
                    s.filtered = n;
                }
            }
            summary = Some(s);
            continue;
        }
        let Some(rest) = line.strip_prefix("test ") else {
            continue;
        };
        let Some((name, result)) = rest.split_once(" ... ") else {
            continue;
        };
        let outcome = if result.starts_with("ok") {
            Outcome::Passed
        } else if result.starts_with("FAILED") {
            Outcome::Failed
        } else if result.starts_with("ignored") {
            Outcome::Ignored
        } else {
            continue;
        };
        tests.push((name.trim().to_string(), outcome));
    }
    (tests, summary)
}

/// `cargo test` arguments split into Cargo's (`--test NAME`, `--lib`, a filter) and the
/// test binaries' (after `--`, plus the filter).
fn split_command(command: &str) -> (Vec<String>, Vec<String>) {
    let words: Vec<String> = command.split_whitespace().map(str::to_string).collect();
    let (before, after) = match words.iter().position(|w| w == "--") {
        Some(i) => (words[..i].to_vec(), words[i + 1..].to_vec()),
        None => (words, Vec::new()),
    };
    let mut cargo = Vec::new();
    let mut binary = Vec::new();
    let mut i = 0;
    while i < before.len() {
        let w = &before[i];
        if matches!(w.as_str(), "--test" | "--bin" | "-p" | "--package") {
            cargo.push(w.clone());
            if let Some(name) = before.get(i + 1) {
                cargo.push(name.clone());
            }
            i += 2;
            continue;
        }
        if w.starts_with('-') {
            cargo.push(w.clone());
        } else {
            binary.push(w.clone());
        }
        i += 1;
    }
    binary.extend(after);
    (cargo, binary)
}

impl Grader<'_> {
    /// Builds `files` as a package in `dir` and runs its tests as `cargo test <command>`
    /// would, without doc tests.
    pub(crate) fn run_tests(
        &self,
        problem: &Problem,
        files: &[SourceFile],
        command: &str,
        dir: &Path,
    ) -> Result<TestsObs> {
        let pkg = dir.join("pkg");
        let _ = std::fs::remove_dir_all(&pkg);
        std::fs::create_dir_all(&pkg)?;
        write_files(&pkg, files)?;
        let (cargo_args, binary_args) = split_command(command);
        if cargo_args.iter().any(|a| a == "--doc") {
            return self.run_doc_tests(&pkg, &cargo_args, &binary_args, dir);
        }
        let mut args: Vec<&str> = vec!["test", "--no-run"];
        // Unit and integration tests only (every target with tests): doc tests are Chapter 14's.
        if !cargo_args.iter().any(|a| a == "--test" || a == "--lib") {
            args.push("--tests");
        }
        args.extend(cargo_args.iter().map(String::as_str));
        let target = dir.join("target");
        let build = self.toolchain.cargo_args(
            &pkg,
            &args,
            Some(&target),
            self.vendor.as_deref(),
            self.cancel,
        )?;
        let mut targets = Vec::new();
        if build.ok {
            let mut exes = build.tests.clone();
            exes.sort();
            exes.dedup();
            for (label, exe) in exes {
                if self.cancelled() {
                    break;
                }
                let limits = Limits {
                    timeout: Duration::from_millis(problem.time_limit_ms.max(1000)),
                    max_output: 4 << 20,
                };
                let run = || {
                    self.toolchain.run_program(
                        &exe,
                        &pkg,
                        b"",
                        &binary_args,
                        &[],
                        &limits,
                        self.cancel,
                    )
                };
                // A new executable's first launch can be slow on macOS; confirm a timeout.
                let mut out = run()?;
                if out.timed_out && !self.cancelled() {
                    out = run()?;
                }
                let (tests, summary) = parse_libtest(&out.stdout_str());
                targets.push(TargetRun {
                    label,
                    out,
                    tests,
                    summary,
                });
            }
        }
        Ok(TestsObs { build, targets })
    }

    /// Doc tests can't be built ahead of time (`--no-run` skips them), so Cargo builds and
    /// runs them in one go; its report becomes one target, `doc tests`.
    fn run_doc_tests(
        &self,
        pkg: &Path,
        cargo_args: &[String],
        binary_args: &[String],
        dir: &Path,
    ) -> Result<TestsObs> {
        let mut args: Vec<&str> = vec!["test"];
        args.extend(cargo_args.iter().map(String::as_str));
        if !binary_args.is_empty() {
            args.push("--");
            args.extend(binary_args.iter().map(String::as_str));
        }
        let target = dir.join("target");
        let mut build = self.toolchain.cargo_args(
            pkg,
            &args,
            Some(&target),
            self.vendor.as_deref(),
            self.cancel,
        )?;
        let (tests, mut summary) = parse_libtest(&build.text);
        let ran = summary.is_some();
        // A workspace prints one summary per package: count the tests themselves then.
        if build.text.matches("test result:").count() > 1 {
            summary = None;
        }
        // A failing doc test also makes Cargo fail; only a missing report means the
        // library itself didn't compile.
        let mut targets = Vec::new();
        if ran {
            let out = Output {
                stdout: build.text.clone().into_bytes(),
                stderr: Vec::new(),
                code: build.exit,
                signal: None,
                timed_out: build.timed_out,
                output_limit: false,
                elapsed: Duration::ZERO,
            };
            targets.push(TargetRun {
                label: String::from("doc tests"),
                out,
                tests,
                summary,
            });
        }
        build.ok = ran;
        Ok(TestsObs { build, targets })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_libtest_output() {
        let out = "\nrunning 3 tests\ntest tests::slow ... ignored, needs a minute\ntest tests::a ... ok\ntest tests::b ... FAILED\n\nfailures:\n\n---- tests::b stdout ----\npanicked\n\ntest result: FAILED. 1 passed; 1 failed; 1 ignored; 0 measured; 2 filtered out; finished in 0.00s\n";
        let (tests, summary) = parse_libtest(out);
        assert_eq!(
            tests,
            vec![
                ("tests::slow".to_string(), Outcome::Ignored),
                ("tests::a".to_string(), Outcome::Passed),
                ("tests::b".to_string(), Outcome::Failed),
            ]
        );
        assert_eq!(
            summary,
            Some(Summary {
                passed: 1,
                failed: 1,
                ignored: 1,
                filtered: 2
            })
        );
    }

    #[test]
    fn splits_cargo_and_binary_arguments() {
        assert_eq!(
            split_command("--test shelf add -- --include-ignored"),
            (
                vec!["--test".to_string(), "shelf".to_string()],
                vec!["add".to_string(), "--include-ignored".to_string()]
            )
        );
        assert_eq!(split_command(""), (vec![], vec![]));
        assert_eq!(
            split_command("-p course-core --doc"),
            (
                vec!["-p".to_string(), "course-core".to_string(), "--doc".to_string()],
                vec![]
            )
        );
    }
}
