//! Grades a submission against a problem's tests.
//!
//! Each distinct [`Run`] is executed once; every test then checks its assertions
//! against that run's observation.

mod assert;
mod tests_run;

use anyhow::Result;
use rb_content::{Assertion, BuildMode, Case, Problem, Run, SourceFile, TestCase, expand};
use rb_runner::process::{Limits, Output};
use rb_runner::{Compiled, Toolchain};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};
use ts_rs::TS;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum Mode {
    /// Visible tests only.
    Run,
    /// Every test.
    Submit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum TestStatus {
    Passed,
    Failed,
    TimeLimit,
    OutputLimit,
    RuntimeError,
    NotRun,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TestResult {
    pub key: String,
    pub n: u32,
    pub visible: bool,
    pub category: String,
    pub status: TestStatus,
    /// What went wrong, in plain words.
    pub message: Option<String>,
    pub expected: Option<String>,
    pub actual: Option<String>,
    /// The test's input as written in the spec (shown when the test is visible or revealed).
    pub input: String,
    pub checks: String,
    pub ms: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum Verdict {
    /// Submit: every test passed.
    Accepted,
    /// Run: every visible test passed.
    Passed,
    CompileError,
    WrongAnswer,
    /// Only rule (constraint) tests failed.
    RuleBroken,
    TimeLimit,
    RuntimeError,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct GradeResult {
    pub mode: Mode,
    /// Checkpoint problems: the part this grade was for.
    pub part: Option<u32>,
    pub verdict: Verdict,
    pub compile: Compiled,
    pub tests: Vec<TestResult>,
    pub ms: u64,
}

/// Progress events while grading.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(tag = "event", rename_all = "snake_case")]
#[ts(export)]
pub enum GradeEvent {
    Phase { text: String },
    Compiled { compile: Compiled },
    Test { result: TestResult },
}

pub struct Grader<'a> {
    pub toolchain: &'a Toolchain,
    pub edition: &'a str,
    /// Scratch space; each grade gets a fresh folder inside it. Compiled checkers are
    /// cached here too.
    pub work_root: PathBuf,
    /// Shared Cargo target dir for non-isolated Cargo problems.
    pub shared_target: Option<PathBuf>,
    /// Vendored crates (offline `rand` etc.) that replace crates.io for Cargo builds.
    pub vendor: Option<PathBuf>,
    pub cancel: Option<&'a AtomicBool>,
}

/// One program execution and what it was given.
pub(crate) struct RunItem {
    /// The stdin, or for function calls the call expression.
    pub stdin: String,
    pub call: bool,
    /// Suite cases: the reference solution's output (or returned value).
    pub expected: Option<String>,
    /// Function calls: the returned value's `{:?}`, if the call returned. `out.stdout`
    /// then holds only what the function printed.
    pub value: Option<String>,
    pub out: Output,
    /// Program suite cases whose reference failed: its exit code and stderr.
    pub failure: Option<(i32, String)>,
}

/// A suite case's reference outcome: its stdout (or returned value), and for a program
/// whose `main` returned `Err`, the exit code and stderr.
pub struct CaseOutput {
    pub stdout: String,
    pub failure: Option<(i32, String)>,
}

/// Sets the modification time of every file under `dir` (except build output) to now.
fn touch_all(dir: &Path) -> Result<()> {
    let now = std::time::SystemTime::now();
    for e in std::fs::read_dir(dir)? {
        let p = e?.path();
        let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if p.is_dir() {
            if name != "target" && !name.starts_with('.') {
                touch_all(&p)?;
            }
        } else {
            std::fs::File::options()
                .append(true)
                .open(&p)?
                .set_modified(now)?;
        }
    }
    Ok(())
}

/// Serializes Cargo builds (see [`Grader::compile`]).
static CARGO_BUILD: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// Separates the harness's returned value from what the function printed.
const VALUE_MARK: char = '\u{1e}';
/// Ends one call's result in the harness's output.
const END_MARK: char = '\u{1d}';

/// What a run produced.
pub(crate) enum Observation {
    /// One or more executions (several for `times` and suites).
    Runs {
        items: Vec<RunItem>,
        suite: bool,
    },
    /// The program couldn't be run for this test (a fixture couldn't be applied).
    NotRunnable(String),
    /// A fixture program that didn't compile.
    CompileFailed(Compiled),
    Build {
        ok: bool,
        log: String,
        pkg_dir: PathBuf,
    },
    Source {
        info: Result<rb_srccheck::SourceInfo, String>,
        files: Vec<String>,
        /// Each Rust file on its own, for checks about one file.
        per_file: Vec<(String, Result<rb_srccheck::SourceInfo, String>)>,
    },
    Manifest(Result<toml::Value, String>),
    /// Test-writing problems: what `cargo test` did.
    Tests(tests_run::TestsObs),
}

/// The executables built for a grade.
struct Programs {
    plain: Option<PathBuf>,
    /// The function-call harness, and the index of each call in it.
    harness: Option<(PathBuf, HashMap<String, usize>)>,
    /// What each function call returned, made in one batch before the tests are checked.
    calls: HashMap<String, CallResult>,
    /// Built with fixture bindings read from the environment, or why it couldn't be.
    fixture: Option<Result<PathBuf, String>>,
}

fn has_fixture(run: &Run) -> bool {
    match run {
        Run::Process { fixture, .. } => !fixture.is_empty(),
        Run::Suite { cases, .. } => cases.iter().any(|c| !c.fixture.is_empty()),
        _ => false,
    }
}

fn runs_program(run: &Run) -> bool {
    match run {
        Run::Process { .. } | Run::Build { .. } => true,
        Run::Suite { cases, .. } => cases.iter().any(|c| c.call.is_none()),
        _ => false,
    }
}

/// The function calls a run makes, in order.
fn calls_of(run: &Run) -> Vec<String> {
    match run {
        Run::Call {
            call,
            expect,
            typed,
        } => std::iter::once(call.clone())
            .chain(expect.as_deref().map(|e| expect_call(call, e, *typed)))
            .collect(),
        Run::Suite { cases, .. } => cases.iter().filter_map(|c| c.call.clone()).collect(),
        _ => Vec::new(),
    }
}

/// The harness expression for an expected value. Typed, it takes its call's type: the
/// call sits in a branch that never runs, so a literal such as `4294967295` becomes a
/// `u32` instead of an `i32`.
fn expect_call(call: &str, expect: &str, typed: bool) -> String {
    if !typed {
        return expect.to_string();
    }
    // A call with setup runs it first, so the call's value may borrow what it binds.
    let (setup, call) =
        rb_srccheck::split_tail(call).unwrap_or_else(|| (String::new(), call.to_string()));
    format!(
        "{{ {setup} #[allow(unused_assignments, unreachable_code)] let mut __e = {expect}; if false {{ __e = {call}; }} __e }}"
    )
}

/// The harness program: the learner's file as a module, and a `main` that makes the
/// calls numbered in its arguments, in order, each caught and timed on its own. After
/// each call it prints, after anything the call printed: [`VALUE_MARK`], `=` and the
/// value's `{:?}` (or `!` and the panic message), the microseconds taken, and [`END_MARK`].
fn harness_source(file: &str, calls: &[&str], helpers: &str) -> String {
    harness_with(
        &format!(
            "#![allow(dead_code)]\n#[path = \"{file}\"]\nmod solution;\n#[allow(unused_imports)]\nuse solution::*;\n"
        ),
        calls,
        helpers,
    )
}

/// The harness for a Cargo package's library: the learner's `src/lib.rs` itself is the
/// crate root, so `crate::…` paths and file modules (`mod text;` → `src/text.rs`) mean
/// what they mean in the package, and the crate also answers to its own name.
fn library_harness_source(lib: &str, crate_name: &str, calls: &[&str], helpers: &str) -> String {
    harness_with(
        &format!("{lib}\n\n{}", self_alias(lib, crate_name)),
        calls,
        helpers,
    )
}

/// `extern crate self as NAME;`, unless the root already has an item called NAME (the
/// `library` package's `pub mod library;`): then calls must use `crate::` paths.
fn self_alias(lib: &str, crate_name: &str) -> String {
    let taken = syn::parse_file(lib).is_ok_and(|f| {
        f.items.iter().any(|i| {
            let id = match i {
                syn::Item::Mod(m) => &m.ident,
                syn::Item::Fn(f) => &f.sig.ident,
                syn::Item::Struct(s) => &s.ident,
                syn::Item::Enum(e) => &e.ident,
                syn::Item::Const(c) => &c.ident,
                _ => return false,
            };
            id == crate_name
        })
    });
    if taken {
        String::new()
    } else {
        format!("extern crate self as {crate_name};\n")
    }
}

/// `prelude` (the learner's code) followed by the helpers and the harness proper.
fn harness_with(prelude: &str, calls: &[&str], helpers: &str) -> String {
    let mut arms = String::new();
    for (i, c) in calls.iter().enumerate() {
        // `{ setup; tail }` is formatted inside its block: `tail` may borrow from `setup`.
        let arm = match rb_srccheck::split_tail(c) {
            Some((setup, tail)) => format!("{{ {setup} format!(\"{{:?}}\", {tail}) }}"),
            None => format!("format!(\"{{:?}}\", {c})"),
        };
        arms.push_str(&format!("        {i} => {arm},\n"));
    }
    format!(
        r#"{prelude}
{helpers}

fn __rb_case(i: usize) -> String {{
    match i {{
{arms}        _ => String::new(),
    }}
}}

thread_local! {{
    static __RB_PANIC: std::cell::RefCell<String> = Default::default();
}}

fn main() {{
    use std::io::Write;
    std::panic::set_hook(Box::new(|info| {{
        let p = info.payload();
        let msg = p
            .downcast_ref::<&str>()
            .map(|s| s.to_string())
            .or_else(|| p.downcast_ref::<String>().cloned())
            .unwrap_or_default();
        let at = info
            .location()
            .map(|l| format!(" at {{}}:{{}}:{{}}", l.file(), l.line(), l.column()))
            .unwrap_or_default();
        __RB_PANIC.with(|p| *p.borrow_mut() = format!("thread 'main' panicked{{at}}:\n{{msg}}"));
    }}));
    for a in std::env::args().skip(1) {{
        let Ok(i) = a.parse::<usize>() else {{ continue }};
        let t = std::time::Instant::now();
        let r = std::panic::catch_unwind(|| __rb_case(i));
        let us = t.elapsed().as_micros();
        let mut out = std::io::stdout().lock();
        let _ = match r {{
            Ok(v) => write!(out, "{mark}={{us}} {{v}}{end}"),
            Err(_) => write!(out, "{mark}!{{us}} {{}}{end}", __RB_PANIC.with(|p| p.borrow().clone())),
        }};
        let _ = out.flush();
    }}
}}
"#,
        mark = VALUE_MARK.escape_unicode(),
        end = END_MARK.escape_unicode(),
    )
}

/// The outcome of one function call: the returned value's `{:?}` (if it returned), and
/// the execution, whose stdout is only what the call printed.
type CallResult = (Option<String>, Output);

/// Parses a harness run's output into the calls it finished, in order. Also returns
/// what the unfinished call (if any) printed.
fn parse_batch(out: &Output, limit: Duration) -> (Vec<CallResult>, String) {
    let text = out.stdout_str();
    let mut chunks: Vec<&str> = text.split(END_MARK).collect();
    let rest = chunks.pop().unwrap_or("").to_string();
    let done = chunks
        .into_iter()
        .map(|chunk| {
            let (printed, result) = chunk.rsplit_once(VALUE_MARK).unwrap_or((chunk, "!0 "));
            let (head, payload) = result.split_once(' ').unwrap_or((result, ""));
            let us: u64 = head[1..].parse().unwrap_or(0);
            let elapsed = Duration::from_micros(us);
            let ok = head.starts_with('=');
            let o = Output {
                stdout: printed.as_bytes().to_vec(),
                stderr: if ok {
                    Vec::new()
                } else {
                    payload.as_bytes().to_vec()
                },
                code: Some(if ok { 0 } else { 101 }),
                signal: None,
                timed_out: elapsed > limit,
                output_limit: false,
                elapsed,
            };
            (ok.then(|| payload.to_string()), o)
        })
        .collect();
    (done, rest)
}

/// The tests a grade covers: visible ones for Run, all for Submit; for checkpoint
/// problems only the chosen part (the last part when none is given).
pub fn selected_tests(problem: &Problem, mode: Mode, part: Option<u32>) -> Vec<&TestCase> {
    let part = part.unwrap_or(problem.parts.len() as u32);
    problem
        .tests
        .iter()
        .filter(|t| mode == Mode::Submit || t.visible)
        .filter(|t| !problem.checkpoints || t.part == Some(part))
        .collect()
}

impl Grader<'_> {
    pub fn grade(
        &self,
        problem: &Problem,
        files: &[SourceFile],
        mode: Mode,
        part: Option<u32>,
        mut on_event: impl FnMut(GradeEvent),
    ) -> Result<GradeResult> {
        let start = Instant::now();
        let tests = selected_tests(problem, mode, part);
        let part = problem
            .checkpoints
            .then(|| part.unwrap_or(problem.parts.len() as u32));
        if !problem.subject.is_empty() {
            return self.grade_tests(problem, files, mode, part, &tests, start, on_event);
        }
        let dir = self.fresh_dir(&problem.id)?;
        write_files(&dir, files)?;

        // Compile if any selected test needs to run or build the program.
        let mut compile = Compiled {
            ok: true,
            ..Default::default()
        };
        let mut programs = Programs {
            plain: None,
            harness: None,
            calls: HashMap::new(),
            fixture: None,
        };
        let calls: Vec<String> = tests.iter().flat_map(|t| calls_of(&t.run)).collect();
        let calls: Vec<&str> = calls.iter().map(String::as_str).collect();
        let needs_plain = tests.iter().any(|t| runs_program(&t.run))
            || (problem.mode == BuildMode::Cargo && !calls.is_empty());
        if needs_plain || !calls.is_empty() {
            on_event(GradeEvent::Phase {
                text: "Compiling".into(),
            });
            if needs_plain {
                compile = self.compile(problem, &dir, "plain")?;
                programs.plain = compile.executable.clone();
            }
            if compile.ok && !calls.is_empty() {
                let (c, harness) = self.compile_harness(problem, &dir, &calls)?;
                if !c.ok || !needs_plain {
                    compile = c;
                }
                programs.harness = harness;
            }
            on_event(GradeEvent::Compiled {
                compile: compile.clone(),
            });
            if compile.ok && tests.iter().any(|t| has_fixture(&t.run)) {
                programs.fixture = Some(self.compile_fixture(problem, files, &dir)?);
            }
        }
        if !compile.ok {
            let tests = tests
                .iter()
                .map(|t| result(t, TestStatus::NotRun, None, None, None, 0))
                .collect();
            let _ = std::fs::remove_dir_all(&dir);
            return Ok(GradeResult {
                mode,
                part,
                verdict: Verdict::CompileError,
                compile,
                tests,
                ms: start.elapsed().as_millis() as u64,
            });
        }

        on_event(GradeEvent::Phase {
            text: "Running tests".into(),
        });
        if let Some((exe, index)) = &programs.harness {
            let mut unique: Vec<&str> = Vec::new();
            for c in &calls {
                if !unique.contains(c) {
                    unique.push(c);
                }
            }
            let results = self.run_calls(problem, exe, &dir, index, &unique)?;
            programs.calls = unique.iter().map(|c| c.to_string()).zip(results).collect();
        }
        let ctx = assert::Ctx {
            grader: self,
            problem,
            dir: &dir,
            checkers: Default::default(),
        };
        let mut cache: HashMap<Run, (Observation, u64)> = HashMap::new();
        let mut results = Vec::new();
        let mut cancelled = false;
        for t in &tests {
            if self.cancelled() {
                cancelled = true;
                break;
            }
            if !cache.contains_key(&t.run) {
                let t0 = Instant::now();
                let obs = self.observe(problem, &t.run, &dir, &programs, files)?;
                cache.insert(t.run.clone(), (obs, t0.elapsed().as_millis() as u64));
            }
            let (obs, ms) = &cache[&t.run];
            let r = check(problem, t, obs, *ms, &ctx);
            on_event(GradeEvent::Test { result: r.clone() });
            results.push(r);
        }
        let _ = std::fs::remove_dir_all(&dir);

        let verdict = if cancelled {
            Verdict::Cancelled
        } else {
            verdict(mode, &results)
        };
        Ok(GradeResult {
            mode,
            part,
            verdict,
            compile,
            tests: results,
            ms: start.elapsed().as_millis() as u64,
        })
    }

    /// Test-writing problems: the learner's tests run against the subject and each mutant
    /// (in parallel, each in its own package and target folder); rule tests look at the
    /// learner's part of the code only.
    #[allow(clippy::too_many_arguments)]
    fn grade_tests(
        &self,
        problem: &Problem,
        files: &[SourceFile],
        mode: Mode,
        part: Option<u32>,
        tests: &[&TestCase],
        start: Instant,
        mut on_event: impl FnMut(GradeEvent),
    ) -> Result<GradeResult> {
        let not_run = |compile: Compiled| GradeResult {
            mode,
            part,
            verdict: Verdict::CompileError,
            compile,
            tests: tests
                .iter()
                .map(|t| result(t, TestStatus::NotRun, None, None, None, 0))
                .collect(),
            ms: start.elapsed().as_millis() as u64,
        };
        let parts = match problem.learner_parts(files) {
            Ok(p) => p,
            Err(why) => {
                return Ok(not_run(Compiled {
                    ok: false,
                    rendered: format!("error: {why}\n"),
                    ..Default::default()
                }));
            }
        };
        let dir = self.fresh_dir(&problem.id)?;
        let mut runs: Vec<&Run> = Vec::new();
        for t in tests {
            if matches!(t.run, Run::Tests { .. }) && !runs.contains(&&t.run) {
                runs.push(&t.run);
            }
        }
        // The supplied code first: its build is the one shown, and decides compile errors.
        runs.sort_by_key(|r| matches!(r, Run::Tests { mutant: Some(_), .. }));
        on_event(GradeEvent::Phase {
            text: "Building and running your tests".into(),
        });
        let width = std::thread::available_parallelism()
            .map_or(2, |n| n.get())
            .clamp(1, 4);
        let mut observed: Vec<Option<(Result<tests_run::TestsObs>, u64)>> =
            (0..runs.len()).map(|_| None).collect();
        for (chunk_i, chunk) in runs.chunks(width).enumerate() {
            if self.cancelled() {
                break;
            }
            std::thread::scope(|scope| {
                let handles: Vec<_> = chunk
                    .iter()
                    .enumerate()
                    .map(|(i, run)| {
                        let n = chunk_i * width + i;
                        let rdir = dir.join(format!("run-{n}"));
                        let parts = &parts;
                        scope.spawn(move || {
                            let t0 = Instant::now();
                            let Run::Tests { mutant, command } = run else {
                                unreachable!()
                            };
                            let pkg = problem.assemble(parts, mutant.as_deref());
                            let r = std::fs::create_dir_all(&rdir)
                                .map_err(anyhow::Error::from)
                                .and_then(|_| self.run_tests(problem, &pkg, command, &rdir));
                            (n, r, t0.elapsed().as_millis() as u64)
                        })
                    })
                    .collect();
                for h in handles {
                    if let Ok((n, r, ms)) = h.join() {
                        observed[n] = Some((r, ms));
                    }
                }
            });
        }
        let mut cache: HashMap<Run, (Observation, u64)> = HashMap::new();
        let mut compile = Compiled {
            ok: true,
            ..Default::default()
        };
        let mut shown = false;
        for (run, o) in runs.iter().zip(observed) {
            let Some((r, ms)) = o else { continue };
            let obs = r?;
            if !shown && matches!(run, Run::Tests { mutant: None, .. }) {
                compile = obs.build.clone();
                shown = true;
            }
            cache.insert((*run).clone(), (Observation::Tests(obs), ms));
        }
        on_event(GradeEvent::Compiled {
            compile: compile.clone(),
        });
        if !compile.ok {
            let _ = std::fs::remove_dir_all(&dir);
            return Ok(not_run(compile));
        }
        let programs = Programs {
            plain: None,
            harness: None,
            calls: HashMap::new(),
            fixture: None,
        };
        let ctx = assert::Ctx {
            grader: self,
            problem,
            dir: &dir,
            checkers: Default::default(),
        };
        let mut results = Vec::new();
        for t in tests {
            if !cache.contains_key(&t.run) {
                let t0 = Instant::now();
                let obs = if self.cancelled() {
                    Observation::NotRunnable("Cancelled.".into())
                } else {
                    self.observe(problem, &t.run, &dir, &programs, &parts)?
                };
                cache.insert(t.run.clone(), (obs, t0.elapsed().as_millis() as u64));
            }
            let (obs, ms) = &cache[&t.run];
            let r = check(problem, t, obs, *ms, &ctx);
            on_event(GradeEvent::Test { result: r.clone() });
            results.push(r);
        }
        let _ = std::fs::remove_dir_all(&dir);
        let verdict = if self.cancelled() {
            Verdict::Cancelled
        } else {
            verdict(mode, &results)
        };
        Ok(GradeResult {
            mode,
            part,
            verdict,
            compile,
            tests: results,
            ms: start.elapsed().as_millis() as u64,
        })
    }

    /// Runs `files` (the reference solution) on each case and returns its stdout.
    /// Used by course-check to fill in suite expectations.
    pub fn reference_outputs(
        &self,
        problem: &Problem,
        files: &[SourceFile],
        cases: &[Case],
    ) -> Result<Vec<CaseOutput>> {
        let dir = self.fresh_dir(&format!("{}-ref", problem.id))?;
        write_files(&dir, files)?;
        if cases.iter().any(|c| c.call.is_some()) {
            let calls: Vec<&str> = cases.iter().filter_map(|c| c.call.as_deref()).collect();
            let (c, harness) = self.compile_harness(problem, &dir, &calls)?;
            let Some((exe, index)) = harness else {
                anyhow::bail!(
                    "the reference solution doesn't compile with the harness:\n{}{}",
                    rb_runner::strip_ansi(&c.rendered),
                    c.log
                );
            };
            let mut out = Vec::new();
            for (call, r) in calls
                .iter()
                .zip(self.run_calls(problem, &exe, &dir, &index, &calls)?)
            {
                match r {
                    (Some(v), o) if o.success() && o.stdout.is_empty() && o.stderr.is_empty() => {
                        out.push(CaseOutput {
                            stdout: v,
                            failure: None,
                        })
                    }
                    (_, o) => anyhow::bail!(
                        "the reference solution fails on {call}: {}{}",
                        o.stdout_str(),
                        o.stderr_str()
                    ),
                }
            }
            let _ = std::fs::remove_dir_all(&dir);
            return Ok(out);
        }
        let c = self.compile(problem, &dir, "plain")?;
        if !c.ok {
            anyhow::bail!(
                "the reference solution doesn't compile:\n{}{}",
                rb_runner::strip_ansi(&c.rendered),
                c.log
            );
        }
        let fixture = if cases.iter().any(|c| !c.fixture.is_empty()) {
            Some(
                self.compile_fixture(problem, files, &dir)?
                    .map_err(|e| anyhow::anyhow!("fixture build: {e}"))?,
            )
        } else {
            None
        };
        let mut out = Vec::new();
        for case in cases {
            let exe = match (&case.fixture.is_empty(), &fixture) {
                (false, Some(f)) => f.clone(),
                _ => c.executable.clone().unwrap_or_default(),
            };
            let o = self.execute(problem, &exe, &dir, &case.stdin, &[], &case.fixture)?;
            // `main` returning `Err` (exit code 1) is an expected outcome; a panic isn't.
            let failed = o.code == Some(1) && !o.timed_out;
            if !failed && (!o.success() || !o.stderr.is_empty()) {
                anyhow::bail!(
                    "the reference solution fails on input {:?}: {}",
                    case.stdin,
                    o.stderr_str()
                );
            }
            out.push(CaseOutput {
                stdout: o.stdout_str(),
                failure: failed.then(|| (1, o.stderr_str())),
            });
        }
        let _ = std::fs::remove_dir_all(&dir);
        Ok(out)
    }

    fn cancelled(&self) -> bool {
        self.cancel.is_some_and(|c| c.load(Ordering::Relaxed))
    }

    fn fresh_dir(&self, id: &str) -> Result<PathBuf> {
        std::fs::create_dir_all(&self.work_root)?;
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let dir = self.work_root.join(format!("{id}-{stamp}"));
        std::fs::create_dir_all(&dir)?;
        Ok(dir)
    }

    /// Builds the program in `dir`. The executable is copied to `dir/.bin/<name>`, so
    /// a later build in the shared target dir can't replace it.
    fn compile(&self, problem: &Problem, dir: &Path, name: &str) -> Result<Compiled> {
        let bin = dir.join(".bin");
        std::fs::create_dir_all(&bin)?;
        let keep = bin.join(if cfg!(windows) {
            format!("{name}.exe")
        } else {
            name.to_string()
        });
        match problem.mode {
            BuildMode::Rustc => {
                let src = dir.join(
                    problem
                        .starter
                        .first()
                        .map(|f| f.path.as_str())
                        .unwrap_or("main.rs"),
                );
                let mut c = self
                    .toolchain
                    .compile_rustc(&src, &keep, self.edition, &[])?;
                if c.ok {
                    c.executable = Some(keep);
                }
                Ok(c)
            }
            BuildMode::Cargo => {
                // Builds in a shared target dir write the same executable path; hold the
                // lock until it's copied out, so concurrent grades can't swap binaries.
                let _guard = CARGO_BUILD.lock().unwrap_or_else(|e| e.into_inner());
                // Cargo judges freshness by modification time, and every problem's package
                // has the same name: files written before waiting for the lock could look
                // older than another problem's build and be skipped.
                touch_all(dir)?;
                let mut c = self.cargo(problem, dir, "build")?;
                if let Some(exe) = c.executable.take() {
                    std::fs::copy(&exe, &keep)?;
                    c.executable = Some(keep);
                }
                Ok(c)
            }
        }
    }

    /// Makes these calls with the harness, as few processes as possible: one runs them
    /// all, unless a call hangs or crashes the process; that call is then judged on its
    /// own and the rest continue in a new process.
    fn run_calls(
        &self,
        problem: &Problem,
        exe: &Path,
        dir: &Path,
        index: &HashMap<String, usize>,
        calls: &[&str],
    ) -> Result<Vec<CallResult>> {
        let limit = Duration::from_millis(problem.time_limit_ms);
        let batch_limits = Limits {
            timeout: limit * 10,
            max_output: 32 << 20,
        };
        let mut results: Vec<CallResult> = Vec::new();
        while results.len() < calls.len() && !self.cancelled() {
            let pending = &calls[results.len()..];
            let args: Vec<String> = pending.iter().map(|c| index[*c].to_string()).collect();
            let out = self.toolchain.run_program(
                exe,
                dir,
                b"",
                &args,
                &[],
                &batch_limits,
                self.cancel,
            )?;
            let (done, rest) = parse_batch(&out, limit);
            let finished = done.len() == pending.len();
            results.extend(done);
            if finished || results.len() >= calls.len() {
                break;
            }
            // The next call didn't finish: judge it alone with its own limit.
            let call = calls[results.len()];
            let alone = self.execute(
                problem,
                exe,
                dir,
                "",
                &[index[call].to_string()],
                &BTreeMap::new(),
            )?;
            let (mut one, _) = parse_batch(&alone, limit);
            results.push(match one.pop() {
                Some(r) => r,
                None => {
                    // It hung, crashed the process or exited it: keep that execution.
                    let mut o = if out.timed_out { alone } else { out };
                    o.stdout = rest.into_bytes();
                    (None, o)
                }
            });
        }
        Ok(results)
    }

    /// Builds a fixture program: the learner's file as a module, plus `main`.
    fn compile_fixture_program(
        &self,
        problem: &Problem,
        dir: &Path,
        main: &str,
        remove: &[String],
    ) -> Result<(Compiled, Option<PathBuf>)> {
        if problem.mode == BuildMode::Cargo {
            return self.compile_consumer(dir, main, remove);
        }
        let file = problem
            .starter
            .iter()
            .find(|f| f.path.ends_with(".rs"))
            .map(|f| f.path.as_str())
            .unwrap_or("lib.rs");
        let n = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let src = dir.join(format!("fixture_{n}.rs"));
        std::fs::write(
            &src,
            format!(
                "#![allow(dead_code)]\n#[path = \"{file}\"]\nmod solution;\n#[allow(unused_imports)]\nuse solution::*;\n\n{main}\n"
            ),
        )?;
        let bin = dir.join(".bin");
        std::fs::create_dir_all(&bin)?;
        let exe = bin.join(format!("fixture_{n}"));
        let c = self
            .toolchain
            .compile_rustc(&src, &exe, self.edition, &[])?;
        let _ = std::fs::remove_file(&src);
        Ok((c.clone(), c.ok.then_some(exe)))
    }

    /// In a Cargo package: builds the library (after deleting the `remove` files from a
    /// copy of the package), then `main` as another crate that links it by crate name.
    fn compile_consumer(
        &self,
        dir: &Path,
        main: &str,
        remove: &[String],
    ) -> Result<(Compiled, Option<PathBuf>)> {
        let n = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let base = if remove.is_empty() {
            dir.to_path_buf()
        } else {
            let copy = dir.join(format!(".probe_{n}"));
            copy_tree(&dir.join("src"), &copy.join("src"))?;
            for r in remove {
                let _ = std::fs::remove_file(copy.join(r));
            }
            copy
        };
        let lib = base.join("src/lib.rs");
        if !lib.exists() {
            return Ok((no_library(), None));
        }
        let name = crate_name(dir);
        let bin = dir.join(".bin");
        std::fs::create_dir_all(&bin)?;
        let rlib = bin.join(format!("lib{name}_{n}.rlib"));
        let c = self.toolchain.compile_rustc(
            &lib,
            &rlib,
            self.edition,
            &[
                "--crate-type",
                "lib",
                "--crate-name",
                &name,
                "-A",
                "warnings",
            ],
        )?;
        if !c.ok {
            return Ok((c, None));
        }
        let main = if main.trim().is_empty() {
            "fn main() {}"
        } else {
            main
        };
        let src = bin.join(format!("probe_{n}.rs"));
        std::fs::write(&src, format!("#![allow(unused)]\n{main}\n"))?;
        let exe = bin.join(format!("probe_{n}"));
        let extern_arg = format!("{name}={}", rlib.display());
        let c =
            self.toolchain
                .compile_rustc(&src, &exe, self.edition, &["--extern", &extern_arg])?;
        let _ = std::fs::remove_file(&src);
        Ok((c.clone(), c.ok.then_some(exe)))
    }

    /// Builds the harness that calls the learner's functions. Returns the compile result
    /// and, when it compiled, the executable with each call's index.
    #[allow(clippy::type_complexity)]
    fn compile_harness(
        &self,
        problem: &Problem,
        dir: &Path,
        calls: &[&str],
    ) -> Result<(Compiled, Option<(PathBuf, HashMap<String, usize>)>)> {
        let mut unique: Vec<&str> = Vec::new();
        let mut index = HashMap::new();
        for c in calls {
            if !index.contains_key(*c) {
                index.insert(c.to_string(), unique.len());
                unique.push(c);
            }
        }
        let src = if problem.mode == BuildMode::Cargo {
            let Ok(lib) = std::fs::read_to_string(dir.join("src/lib.rs")) else {
                return Ok((no_library(), None));
            };
            let src = dir.join("src/__rb_harness.rs");
            std::fs::write(
                &src,
                library_harness_source(&lib, &crate_name(dir), &unique, &problem.helpers),
            )?;
            src
        } else {
            let file = problem
                .starter
                .iter()
                .find(|f| f.path.ends_with(".rs"))
                .map(|f| f.path.as_str())
                .unwrap_or("lib.rs");
            let src = dir.join("harness.rs");
            std::fs::write(&src, harness_source(file, &unique, &problem.helpers))?;
            src
        };
        let bin = dir.join(".bin");
        std::fs::create_dir_all(&bin)?;
        let exe = bin.join(if cfg!(windows) {
            "harness.exe"
        } else {
            "harness"
        });
        let c = self
            .toolchain
            .compile_rustc(&src, &exe, self.edition, &["-A", "dead_code"])?;
        let harness = c.ok.then_some((exe, index));
        Ok((c, harness))
    }

    fn cargo(&self, problem: &Problem, dir: &Path, command: &str) -> Result<Compiled> {
        let target = if problem.isolated_target {
            None
        } else {
            self.shared_target.as_deref()
        };
        self.toolchain
            .cargo(dir, command, target, self.vendor.as_deref(), self.cancel)
    }

    /// A second build whose fixture bindings read their values from the environment.
    fn compile_fixture(
        &self,
        problem: &Problem,
        files: &[SourceFile],
        dir: &Path,
    ) -> Result<std::result::Result<PathBuf, String>> {
        let fdir = dir.join(".fixture");
        let mut instrumented = Vec::new();
        for f in files {
            let mut f = f.clone();
            let main = f.path == "main.rs" || f.path == "src/main.rs";
            if main {
                match rb_srccheck::instrument(&f.text, &problem.fixtures) {
                    Ok(t) => f.text = t,
                    Err(e) => return Ok(Err(e)),
                }
            }
            instrumented.push(f);
        }
        write_files(&fdir, &instrumented)?;
        let c = self.compile(problem, &fdir, "fixture")?;
        Ok(match c.executable {
            Some(exe) if c.ok => Ok(exe),
            _ => Err(format!(
                "The tests couldn't put a test value into {}. Keep {} declared with a type, as in the starter code.\n{}",
                names(&problem.fixtures),
                names(&problem.fixtures),
                rb_runner::strip_ansi(&c.rendered)
            )),
        })
    }

    fn execute(
        &self,
        problem: &Problem,
        exe: &Path,
        dir: &Path,
        stdin: &str,
        args: &[String],
        fixture: &BTreeMap<String, String>,
    ) -> Result<Output> {
        let limits = Limits {
            timeout: Duration::from_millis(problem.time_limit_ms),
            max_output: 32 << 20,
        };
        let env: Vec<(String, String)> = fixture
            .iter()
            .map(|(k, v)| (format!("RB_FIXTURE_{k}"), v.clone()))
            .collect();
        let run = || {
            self.toolchain
                .run_program(exe, dir, stdin.as_bytes(), args, &env, &limits, self.cancel)
        };
        // A fresh binary's first launch can be slowed by the OS (macOS scans new
        // executables), so a timeout is confirmed by a second run before it counts.
        let out = run()?;
        if out.timed_out && !self.cancelled() {
            return run();
        }
        Ok(out)
    }

    fn observe(
        &self,
        problem: &Problem,
        run: &Run,
        dir: &Path,
        programs: &Programs,
        files: &[SourceFile],
    ) -> Result<Observation> {
        let exe_for = |fixture: bool| -> std::result::Result<PathBuf, String> {
            if fixture {
                match &programs.fixture {
                    Some(r) => r.clone(),
                    None => Err("no fixture build".into()),
                }
            } else {
                programs
                    .plain
                    .clone()
                    .ok_or_else(|| "no executable was built".to_string())
            }
        };
        Ok(match run {
            Run::Process {
                stdin,
                stdin_pieces,
                args,
                fixture,
                times,
            } => {
                let exe = match exe_for(!fixture.is_empty()) {
                    Ok(e) => e,
                    Err(e) => return Ok(Observation::NotRunnable(e)),
                };
                let input = format!("{stdin}{}", expand(stdin_pieces));
                let mut items = Vec::new();
                for _ in 0..(*times).max(1) {
                    if self.cancelled() {
                        break;
                    }
                    let out = self.execute(problem, &exe, dir, &input, args, fixture)?;
                    let stop = !out.success();
                    items.push(RunItem {
                        stdin: input.clone(),
                        call: false,
                        expected: None,
                        value: None,
                        out,
                        failure: None,
                    });
                    if stop {
                        break;
                    }
                }
                Observation::Runs {
                    items,
                    suite: false,
                }
            }
            Run::Call {
                call,
                expect,
                typed,
            } => {
                let Some((value, out)) = programs.calls.get(call).cloned() else {
                    return Ok(Observation::NotRunnable("no harness was built".into()));
                };
                // The expected value, as the harness printed it.
                let expected = match expect {
                    Some(e) => match programs.calls.get(&expect_call(call, e, *typed)) {
                        Some((Some(v), _)) => Some(v.clone()),
                        _ => {
                            return Ok(Observation::NotRunnable(format!(
                                "internal: the expected value `{e}` couldn't be computed"
                            )));
                        }
                    },
                    None => None,
                };
                Observation::Runs {
                    items: vec![RunItem {
                        stdin: call.clone(),
                        call: true,
                        expected,
                        value,
                        out,
                        failure: None,
                    }],
                    suite: false,
                }
            }
            Run::Suite { cases, .. } if cases.iter().any(|c| c.call.is_some()) => {
                let mut items = Vec::new();
                for case in cases {
                    let call = case.call.clone().unwrap_or_default();
                    let Some((value, out)) = programs.calls.get(&call).cloned() else {
                        return Ok(Observation::NotRunnable("no harness was built".into()));
                    };
                    let stop = out.timed_out || out.code != Some(0);
                    items.push(RunItem {
                        stdin: call,
                        call: true,
                        expected: case.stdout.clone(),
                        value,
                        out,
                        failure: None,
                    });
                    if stop {
                        break;
                    }
                }
                Observation::Runs { items, suite: true }
            }
            Run::Suite { cases, .. } => {
                let mut items = Vec::new();
                for case in cases {
                    if self.cancelled() {
                        break;
                    }
                    let exe = match exe_for(!case.fixture.is_empty()) {
                        Ok(e) => e,
                        Err(e) => return Ok(Observation::NotRunnable(e)),
                    };
                    let out = self.execute(problem, &exe, dir, &case.stdin, &[], &case.fixture)?;
                    // A crash or timeout fails the test; the remaining cases needn't run.
                    let stop = out.timed_out || out.output_limit;
                    items.push(RunItem {
                        stdin: case.stdin.clone(),
                        call: false,
                        expected: case.stdout.clone(),
                        value: None,
                        out,
                        failure: case
                            .exit_code
                            .map(|c| (c, case.stderr.clone().unwrap_or_default())),
                    });
                    if stop {
                        break;
                    }
                }
                Observation::Runs { items, suite: true }
            }
            Run::Fixture { main, remove } => {
                let (c, exe) = self.compile_fixture_program(problem, dir, main, remove)?;
                match exe {
                    Some(exe) if c.ok => {
                        let out = self.execute(problem, &exe, dir, "", &[], &BTreeMap::new())?;
                        Observation::Runs {
                            items: vec![RunItem {
                                stdin: String::new(),
                                call: false,
                                expected: None,
                                value: None,
                                out,
                                failure: None,
                            }],
                            suite: false,
                        }
                    }
                    _ => Observation::CompileFailed(c),
                }
            }
            Run::Tests { mutant, command } => {
                let parts = problem
                    .learner_parts(files)
                    .unwrap_or_else(|_| files.to_vec());
                let pkg = problem.assemble(&parts, mutant.as_deref());
                let rdir = dir.join("tests-run");
                std::fs::create_dir_all(&rdir)?;
                Observation::Tests(self.run_tests(problem, &pkg, command, &rdir)?)
            }
            Run::Build { command } => {
                let c = self.cargo(problem, dir, command)?;
                Observation::Build {
                    ok: c.ok,
                    log: rb_runner::strip_ansi(&format!("{}{}", c.rendered, c.log)),
                    pkg_dir: dir.to_path_buf(),
                }
            }
            Run::Source => {
                // Files the learner edits plus the full source for code checks.
                let rust: String = files
                    .iter()
                    .filter(|f| f.path.ends_with(".rs"))
                    .map(|f| f.text.as_str())
                    .collect::<Vec<_>>()
                    .join("\n");
                Observation::Source {
                    info: rb_srccheck::parse(&rust),
                    files: files.iter().map(|f| f.path.clone()).collect(),
                    per_file: files
                        .iter()
                        .filter(|f| f.path.ends_with(".rs"))
                        .map(|f| (f.path.clone(), rb_srccheck::parse(&f.text)))
                        .collect(),
                }
            }
            Run::Manifest => {
                let m = files
                    .iter()
                    .find(|f| f.path == "Cargo.toml")
                    .map(|f| f.text.as_str())
                    .unwrap_or("");
                Observation::Manifest(toml::from_str::<toml::Value>(m).map_err(|e| e.to_string()))
            }
            Run::Todo => Observation::Source {
                info: Err("this test hasn't been converted yet".into()),
                files: Vec::new(),
                per_file: Vec::new(),
            },
        })
    }
}

/// The package's library crate name: its `Cargo.toml` package name with `-` as `_`.
fn crate_name(dir: &Path) -> String {
    std::fs::read_to_string(dir.join("Cargo.toml"))
        .ok()
        .and_then(|t| toml::from_str::<toml::Value>(&t).ok())
        .and_then(|m| m.get("package")?.get("name")?.as_str().map(str::to_string))
        .unwrap_or_else(|| "main".into())
        .replace('-', "_")
}

/// The build result when tests call the library but there's no `src/lib.rs`.
fn no_library() -> Compiled {
    Compiled {
        ok: false,
        rendered: "error: these tests call the package's library crate, but `src/lib.rs` doesn't exist yet\n".into(),
        ..Default::default()
    }
}

fn copy_tree(from: &Path, to: &Path) -> Result<()> {
    std::fs::create_dir_all(to)?;
    for e in std::fs::read_dir(from)? {
        let e = e?;
        let p = e.path();
        if p.is_dir() {
            copy_tree(&p, &to.join(e.file_name()))?;
        } else {
            std::fs::copy(&p, to.join(e.file_name()))?;
        }
    }
    Ok(())
}

fn names(bindings: &[String]) -> String {
    bindings
        .iter()
        .map(|b| format!("`{b}`"))
        .collect::<Vec<_>>()
        .join(" and ")
}

fn write_files(dir: &Path, files: &[SourceFile]) -> Result<()> {
    for f in files {
        let rel = Path::new(&f.path);
        if rel.is_absolute()
            || rel
                .components()
                .any(|c| matches!(c, std::path::Component::ParentDir))
        {
            anyhow::bail!("file path `{}` leaves the project folder", f.path);
        }
        let p = dir.join(rel);
        if let Some(parent) = p.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&p, &f.text)?;
    }
    Ok(())
}

fn result(
    t: &TestCase,
    status: TestStatus,
    message: Option<String>,
    expected: Option<String>,
    actual: Option<String>,
    ms: u64,
) -> TestResult {
    TestResult {
        key: t.key.clone(),
        n: t.n,
        visible: t.visible,
        category: t.category.clone(),
        status,
        message,
        expected,
        actual,
        input: t.input_md.clone(),
        checks: t.checks_md.clone(),
        ms,
    }
}

fn check(
    problem: &Problem,
    t: &TestCase,
    obs: &Observation,
    ms: u64,
    ctx: &assert::Ctx,
) -> TestResult {
    let (items, suite) = match obs {
        Observation::Runs { items, suite } => (items, *suite),
        Observation::NotRunnable(why) => {
            return result(t, TestStatus::Failed, Some(why.clone()), None, None, ms);
        }
        other => {
            for a in &t.asserts {
                if let Err(f) = assert::check(a, other, problem) {
                    return result(
                        t,
                        TestStatus::Failed,
                        Some(f.message),
                        f.expected,
                        f.actual,
                        ms,
                    );
                }
            }
            return result(t, TestStatus::Passed, None, None, None, ms);
        }
    };
    let many = items.len() > 1;
    for (i, item) in items.iter().enumerate() {
        // Which execution failed, and with what input, when there are several.
        let label = |msg: String| -> String {
            match (many, suite) {
                (true, true) => format!(
                    "Case {} of {} failed. Input: {}\n{msg}",
                    i + 1,
                    items.len(),
                    if item.call {
                        format!("`{}`", item.stdin)
                    } else {
                        shown_input(&item.stdin)
                    }
                ),
                (true, false) => format!("Run {} of {} failed.\n{msg}", i + 1, items.len()),
                _ => msg,
            }
        };
        let o = &item.out;
        if o.timed_out {
            return result(
                t,
                TestStatus::TimeLimit,
                Some(label(format!(
                    "Your program ran longer than the {} ms limit and was stopped. Look for a loop that never ends, or work repeated for every line.",
                    problem.time_limit_ms
                ))),
                None,
                None,
                ms,
            );
        }
        if o.output_limit {
            return result(
                t,
                TestStatus::OutputLimit,
                Some(label(
                    "Your program printed far too much output and was stopped.".into(),
                )),
                None,
                None,
                ms,
            );
        }
        // Unless the test says otherwise, a run must exit 0 with nothing on stderr.
        let states_exit = t.asserts.iter().any(|a| {
            matches!(
                a,
                Assertion::ExitCode { .. }
                    | Assertion::StderrContains { .. }
                    | Assertion::StderrEmpty
            )
        });
        // A suite case whose reference failed expects that same failure instead.
        let implicit = match &item.failure {
            Some((code, stderr)) => vec![
                Assertion::ExitCode { code: *code },
                Assertion::StderrContains {
                    text: stderr.clone(),
                },
            ],
            None => vec![Assertion::StderrEmpty, Assertion::ExitCode { code: 0 }],
        };
        let implicit: &[Assertion] = if states_exit { &[] } else { &implicit };
        let expects_failure = item.failure.is_some();
        for a in t.asserts.iter().chain(implicit) {
            if let Err(f) = assert::check_run(a, item, ctx) {
                let expects_exit = t.asserts.iter().any(|a| {
                    matches!(
                        a,
                        Assertion::ExitCode { .. } | Assertion::StderrContains { .. }
                    )
                });
                let status = if o.code != Some(0) && !expects_exit && !expects_failure {
                    TestStatus::RuntimeError
                } else {
                    TestStatus::Failed
                };
                let mut message = f.message;
                if status == TestStatus::RuntimeError {
                    let err = o.stderr_str();
                    if !err.trim().is_empty() {
                        message = format!(
                            "{message}\n\nYour program stopped with an error:\n{}",
                            err.trim_end()
                        );
                    }
                }
                return result(t, status, Some(label(message)), f.expected, f.actual, ms);
            }
        }
    }
    result(t, TestStatus::Passed, None, None, None, ms)
}

/// A generated input, short enough to read in a message.
fn shown_input(s: &str) -> String {
    let shown: String = s.chars().take(300).collect();
    if shown.len() < s.len() {
        format!("{shown:?}…")
    } else {
        format!("{shown:?}")
    }
}

fn verdict(mode: Mode, results: &[TestResult]) -> Verdict {
    let failed: Vec<&TestResult> = results
        .iter()
        .filter(|r| r.status != TestStatus::Passed)
        .collect();
    if failed.is_empty() {
        return if mode == Mode::Submit {
            Verdict::Accepted
        } else {
            Verdict::Passed
        };
    }
    if failed.iter().any(|r| r.status == TestStatus::TimeLimit) {
        return Verdict::TimeLimit;
    }
    if failed.iter().any(|r| r.status == TestStatus::RuntimeError) {
        return Verdict::RuntimeError;
    }
    if failed.iter().all(|r| r.category == "constraint") {
        return Verdict::RuleBroken;
    }
    Verdict::WrongAnswer
}

pub use assert::Failure;
