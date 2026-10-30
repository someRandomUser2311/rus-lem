//! Checking one assertion against an observation.

use crate::{Grader, Observation, RunItem};
use rb_content::{Assertion, Problem, expand};
use rb_runner::process::Limits;
use sha2::{Digest, Sha256};
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Duration;

/// What output checks need beyond the run itself: compiled checker programs.
pub(crate) struct Ctx<'a> {
    pub grader: &'a Grader<'a>,
    pub problem: &'a Problem,
    pub dir: &'a Path,
    pub checkers: RefCell<HashMap<String, Result<PathBuf, String>>>,
}

impl Ctx<'_> {
    /// The checker's executable, compiled once and cached by its source hash.
    fn checker(&self, name: &str) -> Result<PathBuf, String> {
        if let Some(r) = self.checkers.borrow().get(name) {
            return r.clone();
        }
        let r = self.build_checker(name);
        self.checkers
            .borrow_mut()
            .insert(name.to_string(), r.clone());
        r
    }

    fn build_checker(&self, name: &str) -> Result<PathBuf, String> {
        let file = format!("{name}.rs");
        let src = self
            .problem
            .checkers
            .iter()
            .find(|c| c.path == file)
            .ok_or_else(|| format!("internal: checker `{name}` is missing"))?;
        let hash = format!("{:x}", Sha256::digest(src.text.as_bytes()));
        let dir = self.grader.work_root.join("checkers").join(&hash[..16]);
        let exe = dir.join(if cfg!(windows) {
            "checker.exe"
        } else {
            "checker"
        });
        if exe.exists() {
            return Ok(exe);
        }
        std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        let main = dir.join("main.rs");
        std::fs::write(&main, &src.text).map_err(|e| e.to_string())?;
        let c = self
            .grader
            .toolchain
            .compile_rustc(&main, &exe, "2024", &["-C", "opt-level=2"])
            .map_err(|e| e.to_string())?;
        if c.ok {
            Ok(exe)
        } else {
            Err(format!(
                "internal: checker `{name}` doesn't compile:\n{}",
                rb_runner::strip_ansi(&c.rendered)
            ))
        }
    }

    /// Runs the checker on one execution: exit 0 passes, otherwise its stdout says why.
    fn run_checker(&self, name: &str, item: &RunItem) -> Result<(), Failure> {
        let exe = self.checker(name).map_err(fail)?;
        let n = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let input = self.dir.join(format!(".check-in-{n}"));
        let output = self.dir.join(format!(".check-out-{n}"));
        std::fs::write(&input, &item.stdin).map_err(|e| fail(e.to_string()))?;
        std::fs::write(&output, &item.out.stdout).map_err(|e| fail(e.to_string()))?;
        let o = self
            .grader
            .toolchain
            .run_program(
                &exe,
                self.dir,
                b"",
                &[
                    input.to_string_lossy().into_owned(),
                    output.to_string_lossy().into_owned(),
                ],
                &[],
                &Limits {
                    timeout: Duration::from_secs(20),
                    max_output: 1 << 20,
                },
                self.grader.cancel,
            )
            .map_err(|e| fail(e.to_string()))?;
        let _ = std::fs::remove_file(&input);
        let _ = std::fs::remove_file(&output);
        if o.success() {
            Ok(())
        } else {
            let why = o.stdout_str().trim().to_string();
            Err(diff(
                if why.is_empty() {
                    "The output doesn't follow the rules.".to_string()
                } else {
                    why
                },
                "",
                item.out.stdout_str(),
            ))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Failure {
    pub message: String,
    pub expected: Option<String>,
    pub actual: Option<String>,
}

fn fail(message: impl Into<String>) -> Failure {
    Failure {
        message: message.into(),
        expected: None,
        actual: None,
    }
}

fn diff(
    message: impl Into<String>,
    expected: impl Into<String>,
    actual: impl Into<String>,
) -> Failure {
    Failure {
        message: message.into(),
        expected: Some(expected.into()),
        actual: Some(actual.into()),
    }
}

/// Output lines: a final newline ends the last line rather than starting an empty one.
fn lines(s: &str) -> Vec<&str> {
    if s.is_empty() {
        return Vec::new();
    }
    s.strip_suffix('\n').unwrap_or(s).split('\n').collect()
}

fn visible(s: &str) -> String {
    s.replace('\t', "→").replace(' ', "·")
}

/// Output checks on one execution.
pub(crate) fn check_run(a: &Assertion, item: &RunItem, ctx: &Ctx) -> Result<(), Failure> {
    let o = &item.out;
    let out = o.stdout_str();
    let ls = lines(&out);
    let line = |n: u32| -> Result<&str, Failure> {
        ls.get(n as usize - 1).copied().ok_or_else(|| {
            diff(
                format!(
                    "Your program printed {} line{}, so there is no line {n}.",
                    ls.len(),
                    if ls.len() == 1 { "" } else { "s" }
                ),
                format!("line {n}"),
                out.clone(),
            )
        })
    };
    match a {
        Assertion::StdoutEq { text } => same_output(text, &out),
        Assertion::LineEq { line: n, text } => {
            let Some(got) = ls.get(*n as usize - 1).copied() else {
                return Err(diff(
                    format!(
                        "Your program printed {} line{}, so there is no line {n}.",
                        ls.len(),
                        if ls.len() == 1 { "" } else { "s" }
                    ),
                    text.clone(),
                    out.clone(),
                ));
            };
            if got == text {
                Ok(())
            } else {
                Err(diff(
                    format!("Line {n} of the output is different."),
                    text.clone(),
                    got,
                ))
            }
        }
        Assertion::LineCount { count } => {
            if ls.len() == *count as usize {
                Ok(())
            } else {
                Err(diff(
                    format!("Expected {count} lines of output, got {}.", ls.len()),
                    count.to_string(),
                    ls.len().to_string(),
                ))
            }
        }
        Assertion::EndsWith { text } => {
            if out.ends_with(text.as_str()) {
                Ok(())
            } else {
                let tail: String = out
                    .chars()
                    .rev()
                    .take(text.chars().count().max(10))
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect();
                Err(diff(
                    "The output doesn't end the way it should.",
                    format!("…{text}"),
                    format!("…{tail}"),
                ))
            }
        }
        Assertion::Contains { text, line: n } => {
            let hay = match n {
                Some(n) => line(*n)?.to_string(),
                None => out.clone(),
            };
            if hay.contains(text.as_str()) {
                Ok(())
            } else {
                Err(diff(
                    format!("The output should contain {text:?}."),
                    text.clone(),
                    hay,
                ))
            }
        }
        Assertion::NotContains { text, line: n } => {
            let hay = match n {
                Some(n) => line(*n)?.to_string(),
                None => out.clone(),
            };
            if hay.contains(text.as_str()) {
                Err(diff(
                    format!("The output shouldn't contain {text:?}."),
                    format!("no {text:?}"),
                    hay,
                ))
            } else {
                Ok(())
            }
        }
        Assertion::NotStartsWith { line: n, any } => {
            let got = line(*n)?;
            match any.iter().find(|p| got.starts_with(p.as_str())) {
                Some(p) => Err(diff(
                    format!("Line {n} starts with {:?}, which it shouldn't.", visible(p)),
                    "",
                    visible(got),
                )),
                None => Ok(()),
            }
        }
        Assertion::LineChars { line: n, len, all } => {
            let got = line(*n)?;
            let count = got.chars().count();
            if count != *len as usize {
                return Err(diff(
                    format!("Line {n} has {count} characters; it should have {len}."),
                    len.to_string(),
                    count.to_string(),
                ));
            }
            if let Some(all) = all
                && let Some(c) = got.chars().find(|c| !all.contains(*c))
            {
                return Err(diff(
                    format!("Line {n} contains {c:?}; it should contain only {all:?}."),
                    all.clone(),
                    got,
                ));
            }
            Ok(())
        }
        Assertion::NoTrailingWhitespace => match ls
            .iter()
            .enumerate()
            .find(|(_, l)| l.ends_with(' ') || l.ends_with('\t'))
        {
            Some((i, l)) => Err(diff(
                format!("Line {} ends with a space or tab.", i + 1),
                visible(l.trim_end()),
                visible(l),
            )),
            None => Ok(()),
        },
        Assertion::StderrEmpty => {
            let err = o.stderr_str();
            if err.is_empty() {
                Ok(())
            } else {
                Err(diff(
                    "Your program wrote to stderr (for example, it panicked).",
                    "",
                    err,
                ))
            }
        }
        Assertion::ExitCode { code } => match (o.code, o.signal) {
            (Some(c), _) if c == *code => Ok(()),
            (Some(c), _) => Err(diff(
                format!("Your program exited with code {c}; expected {code}."),
                code.to_string(),
                c.to_string(),
            )),
            (None, Some(s)) => Err(fail(format!("Your program was stopped by signal {s}."))),
            (None, None) => Err(fail("Your program didn't finish.")),
        },
        Assertion::StdoutPieces { pieces } => same_output(&expand(pieces), &out),
        Assertion::SameAsReference => match &item.expected {
            Some(e) => same_output(e, &out),
            None => Err(fail("internal: this case has no reference output")),
        },
        Assertion::StdoutMatches { pattern } => {
            let re = regex::Regex::new(pattern)
                .map_err(|e| fail(format!("internal: bad pattern: {e}")))?;
            if re.is_match(&out) {
                Ok(())
            } else {
                Err(diff(
                    "The output isn't in the required form.",
                    pattern_shown(pattern),
                    out.clone(),
                ))
            }
        }
        Assertion::Checker { name } => ctx.run_checker(name, item),
        // A fixture that compiled, but shouldn't have.
        Assertion::CompileError { code, message } => Err(fail(message.clone().unwrap_or_else(|| {
            format!(
                "This test program should be rejected by the compiler with {code}, but it compiled. Check that your function's parameters and return type are as specified."
            )
        }))),
        Assertion::Returns { value, tol } => {
            let Some(expected) = value.as_ref().or(item.expected.as_ref()) else {
                return Err(fail("internal: this test has no expected value"));
            };
            let Some(got) = &item.value else {
                return Err(diff(
                    "Your function didn't return a value: the program stopped first.",
                    expected.clone(),
                    String::new(),
                ));
            };
            if !out.is_empty() {
                return Err(diff(
                    "Your function printed output. It should only return its result; remove the printing.",
                    String::new(),
                    out.clone(),
                ));
            }
            if values_match(expected, got, *tol) {
                Ok(())
            } else {
                Err(diff(
                    "Your function returned a different value.",
                    expected.clone(),
                    got.clone(),
                ))
            }
        }
        Assertion::StderrContains { text } => {
            let err = o.stderr_str();
            if err.contains(text.as_str()) {
                Ok(())
            } else {
                Err(diff(
                    format!(
                        "stderr should contain {text:?} (the program should stop with that message)."
                    ),
                    text.clone(),
                    err,
                ))
            }
        }
        other => Err(fail(format!(
            "internal: {} can't be checked on program output",
            name(other)
        ))),
    }
}

/// Checks on builds, source and the manifest.
pub(crate) fn check(a: &Assertion, obs: &Observation, problem: &Problem) -> Result<(), Failure> {
    match obs {
        Observation::Runs { .. } | Observation::NotRunnable(_) => Err(fail(
            "internal: program runs are checked one execution at a time",
        )),
        Observation::CompileFailed(c) => match a {
            Assertion::CompileError { code, .. } => {
                if c.errors().any(|e| e.code.as_deref() == Some(code.as_str())) {
                    Ok(())
                } else {
                    Err(diff(
                        format!("This test program should fail to compile with {code}, but the compiler reports something else."),
                        code.clone(),
                        rb_runner::strip_ansi(&c.rendered),
                    ))
                }
            }
            _ => Err(diff(
                "A test program that uses your functions doesn't compile. Check that their names, parameters and return types are as specified.",
                String::new(),
                rb_runner::strip_ansi(&c.rendered),
            )),
        },
        Observation::Build { ok, log, pkg_dir } => match a {
            Assertion::BuildOk => {
                if *ok {
                    Ok(())
                } else {
                    Err(diff(
                        "Cargo couldn't build the project.",
                        "a successful build",
                        log.clone(),
                    ))
                }
            }
            Assertion::FileExists { path } => {
                let name = std::fs::read_to_string(pkg_dir.join("Cargo.toml"))
                    .ok()
                    .and_then(|t| t.parse::<toml::Table>().ok())
                    .and_then(|t| t.get("package")?.get("name")?.as_str().map(str::to_string))
                    .unwrap_or_default();
                let rel = path
                    .replace("{name}", &name)
                    .replace("{exe}", if cfg!(windows) { ".exe" } else { "" });
                if pkg_dir.join(&rel).exists() {
                    Ok(())
                } else {
                    Err(diff(
                        format!("After the build, `{rel}` doesn't exist."),
                        rel.clone(),
                        "missing",
                    ))
                }
            }
            other => Err(fail(format!(
                "internal: {} can't be checked on a build",
                name(other)
            ))),
        },
        Observation::Source { info, files } => {
            if let Assertion::FilesAllowlist { files: allowed } = a {
                let extra: Vec<&String> = files.iter().filter(|f| !allowed.contains(f)).collect();
                return if extra.is_empty() {
                    Ok(())
                } else {
                    Err(fail(format!(
                        "Only these files may be edited: {}. Found also: {}.",
                        allowed.join(", "),
                        extra
                            .iter()
                            .map(|s| s.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    )))
                };
            }
            let info = info
                .as_ref()
                .map_err(|e| fail(format!("Your code couldn't be read for this check: {e}")))?;
            let r = match a {
                Assertion::MacroCount { name, eq, ge } => info.macro_count(name, *eq, *ge),
                Assertion::MacroSequence {
                    first,
                    then,
                    exactly_once,
                } => info.macro_sequence(
                    (&first.name, &first.fmt),
                    (&then.name, &then.fmt),
                    *exactly_once,
                ),
                Assertion::FmtLiteralForbid {
                    macros,
                    substrings,
                    digits,
                } => info.fmt_literal_forbid(macros, substrings, *digits),
                Assertion::LetsUnchanged { lets } => info.lets_unchanged(lets),
                Assertion::FmtInlinePlaceholders { macros, names } => {
                    info.fmt_inline_placeholders(macros, names)
                }
                Assertion::BannedKeyword { word } => info.banned_keyword(word),
                Assertion::RequiredKeyword { word } => info.required_keyword(word),
                Assertion::CallCount { name, eq, ge } => info.call_count(name, *eq, *ge),
                Assertion::CallChain { calls, arg } => info.call_chain(calls, arg.as_deref()),
                Assertion::LetInit {
                    name,
                    ty,
                    exprs,
                    in_loop,
                    after,
                    shadows,
                } => info.let_init(
                    name,
                    ty.as_deref(),
                    exprs,
                    *in_loop,
                    after.as_deref(),
                    *shadows,
                ),
                Assertion::MatchOn { method } => info.match_on(method),
                Assertion::BannedMethod { name } => info.banned_method(name),
                Assertion::BannedArithmetic => info.banned_arithmetic(),
                Assertion::MacroMentionForbid { macros, name } => {
                    info.macro_mention_forbid(macros, name)
                }
                Assertion::CallInLoop { name } => info.call_in_loop(name),
                Assertion::Uses {
                    what,
                    within,
                    inside,
                    message,
                } => info.uses(
                    what,
                    within.as_deref(),
                    inside.as_deref(),
                    message.as_deref(),
                ),
                Assertion::Forbids {
                    what,
                    within,
                    message,
                } => info.forbids(what, within.as_deref(), message.as_deref()),
                Assertion::Todo { note } => Err(format!("This test isn't converted yet: {note}")),
                other => Err(format!(
                    "internal: {} can't be checked on source",
                    name(other)
                )),
            };
            r.map_err(fail)
        }
        Observation::Manifest(m) => {
            let m = m
                .as_ref()
                .map_err(|e| fail(format!("`Cargo.toml` isn't valid TOML: {e}")))?;
            let get = |key: &str| -> Option<&toml::Value> {
                let mut v = m;
                for part in key.split('.') {
                    v = v.get(part)?;
                }
                Some(v)
            };
            match a {
                Assertion::ManifestEq { key, value } => match get(key) {
                    Some(toml::Value::String(s)) if s == value => Ok(()),
                    Some(other) => Err(diff(
                        format!("In `Cargo.toml`, `{key}` should be {value:?}."),
                        format!("{key} = {value:?}"),
                        format!("{key} = {other}"),
                    )),
                    None => Err(diff(
                        format!("`Cargo.toml` has no `{key}`."),
                        format!("{key} = {value:?}"),
                        "missing",
                    )),
                },
                Assertion::ManifestTableEmpty { table } => match get(table) {
                    Some(toml::Value::Table(t)) if t.is_empty() => Ok(()),
                    Some(toml::Value::Table(t)) => Err(fail(format!(
                        "`[{table}]` should be empty, but it lists: {}.",
                        t.keys().cloned().collect::<Vec<_>>().join(", ")
                    ))),
                    _ => Err(fail(format!(
                        "`Cargo.toml` should keep its `[{table}]` section."
                    ))),
                },
                Assertion::ManifestForbidKeys { keys } => {
                    match keys.iter().find(|k| get(k).is_some()) {
                        Some(k) => Err(fail(format!(
                            "`Cargo.toml` mustn't set `{k}` in this problem."
                        ))),
                        None => Ok(()),
                    }
                }
                other => Err(fail(format!(
                    "internal: {} can't be checked on Cargo.toml",
                    name(other)
                ))),
            }
        }
    }
    .map_err(|mut f| {
        let _ = problem;
        if f.message.is_empty() {
            f.message = "This test failed.".into();
        }
        f
    })
}

fn same_output(expected: &str, out: &str) -> Result<(), Failure> {
    if out == expected {
        Ok(())
    } else {
        Err(diff(
            first_difference(expected, out),
            expected.to_string(),
            out.to_string(),
        ))
    }
}

/// Two `{:?}` values are equal; with `tol`, numbers written with a fraction or exponent
/// may differ by max(tol, 1e-12 × |expected|).
pub(crate) fn values_match(expected: &str, got: &str, tol: Option<f64>) -> bool {
    if expected == got {
        return true;
    }
    let Some(tol) = tol else {
        return false;
    };
    let (e, g) = (tokens(expected), tokens(got));
    e.len() == g.len()
        && e.iter().zip(&g).all(|(a, b)| match (a, b) {
            (Tok::Num(x, fx), Tok::Num(y, fy)) if *fx || *fy => {
                (x - y).abs() <= tol.max(x.abs() * 1e-12)
            }
            (Tok::Num(x, _), Tok::Num(y, _)) => x == y,
            (Tok::Text(x), Tok::Text(y)) => x == y,
            _ => false,
        })
}

#[derive(Debug)]
enum Tok {
    /// A number, and whether it was written as a float.
    Num(f64, bool),
    Text(String),
}

/// Splits `{:?}` text into numbers and the text between them.
fn tokens(s: &str) -> Vec<Tok> {
    let re = regex::Regex::new(r"-?[0-9]+(\.[0-9]+)?(e-?[0-9]+)?").unwrap();
    let mut out = Vec::new();
    let mut at = 0;
    for m in re.find_iter(s) {
        if m.start() > at {
            out.push(Tok::Text(s[at..m.start()].to_string()));
        }
        let t = m.as_str();
        out.push(match t.parse::<f64>() {
            Ok(v) => Tok::Num(v, t.contains(['.', 'e'])),
            Err(_) => Tok::Text(t.to_string()),
        });
        at = m.end();
    }
    if at < s.len() {
        out.push(Tok::Text(s[at..].to_string()));
    }
    out
}

/// A pattern as the learner sees it: anchors dropped, escapes shown as typed.
fn pattern_shown(p: &str) -> String {
    p.trim_start_matches('^')
        .trim_end_matches('$')
        .replace("\\n", "↵\n")
}

fn name(a: &Assertion) -> String {
    serde_json::to_value(a)
        .ok()
        .and_then(|v| v["check"].as_str().map(str::to_string))
        .unwrap_or_default()
}

/// A friendly description of where two outputs first differ.
fn first_difference(expected: &str, actual: &str) -> String {
    let e = lines(expected);
    let a = lines(actual);
    for i in 0..e.len().max(a.len()) {
        match (e.get(i), a.get(i)) {
            (Some(x), Some(y)) if x == y => continue,
            (Some(_), Some(_)) => return format!("The output differs on line {}.", i + 1),
            (Some(_), None) => {
                return format!(
                    "Your output stops after {} line{}; more was expected.",
                    a.len(),
                    if a.len() == 1 { "" } else { "s" }
                );
            }
            (None, Some(_)) => {
                return format!("Your output has extra lines after line {}.", e.len());
            }
            (None, None) => break,
        }
    }
    if expected.ends_with('\n') != actual.ends_with('\n') {
        return if expected.ends_with('\n') {
            "The last line should end with a newline (use `println!`).".into()
        } else {
            "The output has an extra newline at the end.".into()
        };
    }
    "The output is different.".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_splitting() {
        assert_eq!(lines("a\nb\n"), vec!["a", "b"]);
        assert_eq!(lines("a\nb"), vec!["a", "b"]);
        assert_eq!(lines(""), Vec::<&str>::new());
        assert_eq!(lines("\n"), vec![""]);
        assert_eq!(lines("a\n\n"), vec!["a", ""]);
    }

    #[test]
    fn value_tolerance() {
        assert!(values_match("12.0", "12.0", None));
        assert!(!values_match("0.3", "0.30000000000000004", None));
        assert!(values_match("0.3", "0.30000000000000004", Some(1e-6)));
        assert!(values_match("(1, 2.5)", "(1, 2.5000000001)", Some(1e-6)));
        assert!(!values_match("(1, 2.5)", "(2, 2.5)", Some(1e-6)));
        assert!(!values_match("12.0", "12.1", Some(1e-6)));
    }

    #[test]
    fn differences() {
        assert_eq!(
            first_difference("a\nb\n", "a\nc\n"),
            "The output differs on line 2."
        );
        assert_eq!(
            first_difference("a\n", "a"),
            "The last line should end with a newline (use `println!`)."
        );
    }
}
