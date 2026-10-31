//! Generated test suites: inputs from the course's seeded generator, expected output
//! from the reference solution. Filled in before validating and bundling, so the app
//! only replays stored cases.
//!
//! A suite's inputs come from a [`Gen`] description, or from a generator program
//! (`generators/<name>.rs`) for inputs a description can't express. The program is
//! compiled after [`PRELUDE`], which supplies the course's LCG and the output format:
//! `call(text)` emits a function call, `stdin(text)` a program's input.

use anyhow::{Context, Result, bail};
use rb_content::{Case, Chapter, Gen, GenItem, Problem, Run};
use rb_grader::Grader;
use std::collections::BTreeMap;

/// The course's generator: state = 1664525 × state + 1013904223 (mod 2³²).
struct Lcg(u32);

impl Lcg {
    fn next(&mut self) -> u32 {
        self.0 = self.0.wrapping_mul(1664525).wrapping_add(1013904223);
        self.0
    }

    fn pick(&mut self, n: u32) -> u32 {
        self.next() % n
    }
}

/// Every case of a generator, in order, without expected output.
pub fn generate(g: &Gen) -> Vec<Case> {
    let mut r = Lcg(g.seed);
    (0..g.cases)
        .map(|_| {
            let mut fixture = BTreeMap::new();
            let mut value = None;
            if let Some(f) = &g.fixture {
                let v = f.add + r.pick(f.pick);
                fixture.insert(f.binding.clone(), v.to_string());
                value = Some(v);
            }
            let count = r.pick(g.lines);
            let lines: Vec<String> = (0..count)
                .map(|_| match &g.items[r.pick(g.items.len() as u32) as usize] {
                    GenItem::Text(t) => t.clone(),
                    GenItem::Number { number } => r.pick(*number).to_string(),
                    GenItem::OneOf { one_of } => {
                        one_of[r.pick(one_of.len() as u32) as usize].clone()
                    }
                    GenItem::Fixture { .. } => value.unwrap_or_default().to_string(),
                })
                .collect();
            let mut stdin = lines.join("\n");
            if r.pick(2) == 1 {
                stdin.push('\n');
            }
            Case {
                stdin,
                call: None,
                fixture,
                stdout: None,
                exit_code: None,
                stderr: None,
            }
        })
        .collect()
}

/// Supplied before a generator program.
pub const PRELUDE: &str = r#"#![allow(dead_code)]
/// The course's generator: state = 1664525 × state + 1013904223 (mod 2³²).
struct Lcg(u32);
impl Lcg {
    fn next(&mut self) -> u32 {
        self.0 = self.0.wrapping_mul(1664525).wrapping_add(1013904223);
        self.0
    }
    fn pick(&mut self, n: u32) -> u32 {
        self.next() % n
    }
}
fn escaped(s: &str) -> String {
    s.replace('\\', "\\\\").replace('\n', "\\n").replace('\r', "\\r")
}
/// One function-call case: a Rust call expression.
fn call(text: impl AsRef<str>) {
    println!("C {}", escaped(text.as_ref()));
}
/// One program case: the stdin to give it.
fn stdin(text: impl AsRef<str>) {
    println!("S {}", escaped(text.as_ref()));
}
"#;

fn unescape(s: &str) -> String {
    let mut out = String::new();
    let mut it = s.chars();
    while let Some(c) = it.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match it.next() {
            Some('n') => out.push('\n'),
            Some('r') => out.push('\r'),
            Some(c) => out.push(c),
            None => out.push('\\'),
        }
    }
    out
}

/// Runs generator program `name` of problem `p` and returns its cases.
fn run_script(p: &Problem, name: &str, grader: &Grader) -> Result<Vec<Case>> {
    let file = format!("{name}.rs");
    let src = p
        .generators
        .iter()
        .find(|g| g.path == file)
        .with_context(|| format!("{}: generators/{file} is missing", p.id))?;
    let dir = grader.work_root.join(format!("gen-{}-{name}", p.id));
    std::fs::create_dir_all(&dir)?;
    let main = dir.join("main.rs");
    std::fs::write(&main, format!("{PRELUDE}\n{}", src.text))?;
    let exe = dir.join("gen");
    let c = grader
        .toolchain
        .compile_rustc(&main, &exe, "2024", &["-C", "opt-level=1"])?;
    if !c.ok {
        bail!(
            "{}: generators/{file} doesn't compile:\n{}",
            p.id,
            rb_runner::strip_ansi(&c.rendered)
        );
    }
    let o = grader.toolchain.run_program(
        &exe,
        &dir,
        b"",
        &[],
        &[],
        &rb_runner::process::Limits {
            timeout: std::time::Duration::from_secs(60),
            max_output: 64 << 20,
        },
        None,
    )?;
    if !o.success() {
        bail!("{}: generators/{file} failed: {}", p.id, o.stderr_str());
    }
    let _ = std::fs::remove_dir_all(&dir);
    o.stdout_str()
        .lines()
        .map(|l| match l.split_at_checked(2) {
            Some(("C ", t)) => Ok(Case {
                stdin: String::new(),
                call: Some(unescape(t)),
                fixture: BTreeMap::new(),
                stdout: None,
                exit_code: None,
                stderr: None,
            }),
            Some(("S ", t)) => Ok(Case {
                stdin: unescape(t),
                call: None,
                fixture: BTreeMap::new(),
                stdout: None,
                exit_code: None,
                stderr: None,
            }),
            _ => bail!(
                "{}: generators/{file} printed `{l}`; use call() or stdin()",
                p.id
            ),
        })
        .collect()
}

/// Generates the cases of every suite that has a generator and no cases yet, running the
/// reference solution for their expected output when the suite compares against it.
pub fn fill(ch: &mut Chapter, grader: &Grader) -> Result<usize> {
    let mut filled = 0;
    for p in &mut ch.problems {
        for i in 0..p.tests.len() {
            let Run::Suite {
                script: Some(name),
                cases,
                ..
            } = &p.tests[i].run
            else {
                continue;
            };
            if !cases.is_empty() {
                continue;
            }
            let name = name.clone();
            let mut made = run_script(p, &name, grader)?;
            let solution = match (p.checkpoints, p.tests[i].part) {
                (true, Some(n)) => p.part_solutions.get(&n).unwrap_or(&p.solution),
                _ => &p.solution,
            };
            let outs = grader
                .reference_outputs(p, solution, &made)
                .with_context(|| format!("{} {}", p.id, p.tests[i].key))?;
            for (case, out) in made.iter_mut().zip(outs) {
                case.stdout = Some(out.stdout);
                if let Some((code, stderr)) = out.failure {
                    case.exit_code = Some(code);
                    case.stderr = Some(stderr);
                }
            }
            p.tests[i].run = Run::Suite {
                generator: None,
                script: Some(name),
                cases: made,
            };
            filled += 1;
        }
        let mut done: Vec<(Gen, Vec<Case>)> = Vec::new();
        for i in 0..p.tests.len() {
            let Run::Suite {
                generator: Some(g),
                cases,
                ..
            } = &p.tests[i].run
            else {
                continue;
            };
            if !cases.is_empty() {
                continue;
            }
            let g = g.clone();
            let made = match done.iter().find(|(d, _)| *d == g) {
                Some((_, c)) => c.clone(),
                None => {
                    let mut c = generate(&g);
                    if g.reference {
                        let outs = grader
                            .reference_outputs(p, &p.solution, &c)
                            .with_context(|| format!("{} {}", p.id, p.tests[i].key))?;
                        for (case, out) in c.iter_mut().zip(outs) {
                            case.stdout = Some(out.stdout);
                        }
                    }
                    done.push((g.clone(), c.clone()));
                    c
                }
            };
            p.tests[i].run = Run::Suite {
                generator: Some(g),
                script: None,
                cases: made,
            };
            filled += 1;
        }
    }
    Ok(filled)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lcg_matches_the_spec() {
        let mut r = Lcg(0);
        assert_eq!(r.next(), 1013904223);
        assert_eq!(r.next(), 1196435762);
        let g = Gen {
            seed: 0x0206,
            cases: 3,
            lines: 9,
            items: vec![GenItem::Text("a".into()), GenItem::Text("b".into())],
            fixture: None,
            reference: false,
        };
        let a = generate(&g);
        assert_eq!(a, generate(&g), "deterministic");
        assert_eq!(a.len(), 3);
    }
}
