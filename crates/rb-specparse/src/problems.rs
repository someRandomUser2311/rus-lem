//! Problems (problems.md) and projects/milestones (project.md).

use crate::Report;
use crate::md::{self, Section};
use rb_content::*;

/// A chapter whose test rows are complete Rust ("Each runtime row supplies complete Rust
/// setup followed by a final expression") with exact Rust expected values, and the named
/// text fixtures those rows may use (`text_1` → a Rust expression building it).
pub struct RustRows {
    names: Vec<(String, String)>,
}

pub fn rust_rows(problems_md: &str) -> Option<RustRows> {
    let markers = [
        "supplies complete Rust setup followed by a final expression",
        "gives exact setup, a final Rust expression and its expected value",
    ];
    if !markers.iter().any(|m| problems_md.contains(m)) {
        return None;
    }
    let lines: Vec<&str> = problems_md.lines().collect();
    let names = md::table_with_header(&lines, "Symbol")
        .into_iter()
        .filter_map(|r| {
            let name = md::code_spans(r.first()?).into_iter().next()?;
            Some((name, fixture_text(r.get(1)?)?))
        })
        .collect();
    Some(RustRows { names })
}

/// `Prefix "!"; ASCII `a` repeated exactly 99999 times; suffix "A"` → a Rust `String`
/// (the unit may also be bare: `ASCII a repeated …`).
fn fixture_text(cell: &str) -> Option<String> {
    let mut parts = Vec::new();
    for piece in cell.split("; ") {
        let piece = piece.trim();
        let quoted = || piece.split('"').nth(1).map(|q| format!("{q:?}"));
        if piece.starts_with("Prefix") || piece.starts_with("suffix") {
            parts.push(quoted()?);
        } else if piece.contains(" repeated exactly ") {
            let unit = md::code_spans(piece).into_iter().next().or_else(|| {
                let head = piece.split(" repeated exactly ").next()?;
                Some(head.strip_prefix("ASCII ")?.trim().to_string())
            })?;
            let n: usize = piece
                .split(" repeated exactly ")
                .nth(1)?
                .split_whitespace()
                .next()?
                .parse()
                .ok()?;
            parts.push(format!("{unit:?}.repeat({n}).as_str()"));
        } else {
            return None;
        }
    }
    Some(format!("[{}].concat()", parts.join(", ")))
}

pub fn problems(text: &str, rust_rows: Option<&RustRows>, report: &mut Report) -> Vec<Problem> {
    let lines: Vec<&str> = text.lines().collect();
    md::sections(&lines, 3)
        .into_iter()
        .filter(|s| s.heading.starts_with('P'))
        .filter_map(|s| {
            let h = md::dot_parts(&s.heading);
            let level = level(h.get(1).map(String::as_str).unwrap_or(""))?;
            problem(
                h[0].clone(),
                h.get(2).cloned().unwrap_or_default(),
                level,
                &s,
                rust_rows,
                report,
            )
        })
        .collect()
}

/// project.md: `## Chapter project · Title` and `## Library manager milestone LMn · Title`.
pub fn projects(
    text: &str,
    chapter: u32,
    rust_rows: Option<&RustRows>,
    report: &mut Report,
) -> Vec<Problem> {
    let lines: Vec<&str> = text.lines().collect();
    md::sections(&lines, 2)
        .into_iter()
        .filter_map(|s| {
            let h = md::dot_parts(&s.heading);
            let title = h.get(1).cloned().unwrap_or_default();
            if h[0].starts_with("Chapter project") {
                problem(
                    format!("PRJ{chapter:02}"),
                    title,
                    Level::Project,
                    &s,
                    rust_rows,
                    report,
                )
            } else if let Some(lm) = h[0].strip_prefix("Library manager milestone ") {
                problem(
                    lm.trim().to_string(),
                    title,
                    Level::Milestone,
                    &s,
                    rust_rows,
                    report,
                )
            } else {
                None
            }
        })
        .collect()
}

fn level(s: &str) -> Option<Level> {
    Some(match s {
        "Warm-up" => Level::WarmUp,
        "Easy" => Level::Easy,
        "Medium" => Level::Medium,
        "Hard" => Level::Hard,
        _ => return None,
    })
}

fn problem(
    id: String,
    title: String,
    level: Level,
    sec: &Section,
    rust_rows: Option<&RustRows>,
    report: &mut Report,
) -> Option<Problem> {
    let lines = &sec.lines;
    let fields = md::fields(lines);
    let get = |k: &str| md::field(&fields, k).unwrap_or("").to_string();
    let block = |label: &str| {
        md::block_after(lines, label)
            .map(|b| md::text(&b))
            .unwrap_or_default()
    };

    let kind = get("Kind");
    let test_writing = kind.contains("test-writing");
    let mode = if kind.contains("multi-file") || kind.contains("Cargo") || test_writing {
        BuildMode::Cargo
    } else {
        BuildMode::Rustc
    };
    let xp = get("XP").parse().unwrap_or(0);
    let time_limit_ms = get("Time limit")
        .split_whitespace()
        .next()
        .and_then(|n| n.parse::<f64>().ok())
        .map(|s| (s * 1000.0) as u64)
        .unwrap_or(1000);

    let approach_full = block("**Solution approach**");
    let (approach, idiomatic) = match approach_full.split_once("**Idiomatic note:**") {
        Some((a, i)) => (a.trim().to_string(), Some(i.trim().to_string())),
        None => (approach_full, None),
    };
    let hints: Vec<String> = md::block_after(lines, "**Hints**")
        .unwrap_or_default()
        .iter()
        .filter_map(|l| {
            let (n, rest) = l.trim().split_once(". ")?;
            n.parse::<u32>().ok()?;
            Some(rest.trim().to_string())
        })
        .collect();
    if hints.len() != 3 {
        report.warn(format!("{id}: {} hints (expected 3)", hints.len()));
    }

    let starter = starter_files(
        &md::block_after(lines, "**Starter code**")
            .or_else(|| md::block_after(lines, "**Starter files**"))
            .unwrap_or_default(),
        mode,
    );
    let solution = starter_files(
        &md::block_after(lines, "**Reference solution**").unwrap_or_default(),
        mode,
    );
    if starter.is_empty() {
        report.error(format!("{id}: no starter code"));
    }
    // Test-writing problems: the provided code under test is read-only; the starter and
    // reference files are the learner's tests, written below it in the same file.
    let subject = starter_files(
        &md::block_after(lines, "**Provided code**").unwrap_or_default(),
        mode,
    );
    if test_writing && subject.is_empty() {
        // A milestone's provided code is the previous milestone's package, given in the
        // overlay (`subject/`); course-check requires it there.
        report.warn(format!("{id}: no **Provided code** fences; expecting subject/ in the overlay"));
    }
    let mutants = if test_writing {
        mutant_rows(&id, lines, &subject, report)
    } else {
        Vec::new()
    };
    let (starter, mut solution) = if test_writing {
        (
            rb_content::with_subject(&subject, starter),
            rb_content::with_subject(&subject, solution),
        )
    } else {
        (starter, solution)
    };

    // A file of functions without `main` is a library the tests call.
    let mut starter = starter;
    if mode == BuildMode::Rustc {
        for f in &mut starter {
            if f.path == "main.rs" && !f.text.contains("fn main") {
                f.path = "lib.rs".into();
            }
        }
    }
    // A one-file reference solution goes in the starter's file.
    if let ([s], [f]) = (starter.as_slice(), solution.as_mut_slice())
        && !test_writing
    {
        f.path = s.path.clone();
    }
    let signatures: Vec<Signature> = starter.iter().flat_map(|f| signatures(&f.text)).collect();

    let mut description = block("**Description**");
    if description.is_empty() {
        // "Part 1 — Title: text" paragraphs; the title is made bold so it reads (and parses)
        // like the other chapters' `**Part 1 — Title:**`.
        description = block("**Description and numbered parts**")
            .lines()
            .map(
                |l| match l.strip_prefix("Part ").and_then(|r| r.split_once(": ")) {
                    Some((head, rest)) if head.contains(" — ") => {
                        format!("**Part {head}:** {rest}")
                    }
                    _ => l.to_string(),
                },
            )
            .collect::<Vec<_>>()
            .join("\n");
    }
    // Projects describe their parts in `### Part N · Title` sections.
    let mut part_titles = Vec::new();
    for (i, l) in lines.iter().enumerate() {
        if let Some(h) = l.strip_prefix("### Part ") {
            // A bare `### Part 2` has no title.
            let title = h
                .split_once(" · ")
                .map_or("", |(_, t)| t)
                .trim()
                .to_string();
            let body = md::block_after(&lines[i..], l)
                .map(|b| md::text(&b))
                .unwrap_or_default();
            description.push_str(&format!("\n\n**Part {}**\n\n{body}", h.trim()));
            part_titles.push(title);
        }
    }
    let parts: Vec<String> = description
        .lines()
        .filter_map(|l| {
            let t = l.trim_start_matches(|c: char| c.is_ascii_digit() || c == '.' || c == ' ');
            let t = t.strip_prefix("**Part ")?;
            let t = t.split("**").next()?.trim_end_matches(':').trim();
            // "1 — Identity" → "Identity"
            let t = t.split_once(" — ").map(|(_, title)| title).unwrap_or(t);
            Some(t.to_string())
        })
        .collect();
    let parts = if part_titles.is_empty() {
        parts
    } else {
        part_titles
    };

    let mut tests = Vec::new();
    let test_lines = tests_region(lines);
    let fixtures = fixture_programs(lines);
    for row in md::table_with_header(&test_lines, "#") {
        // `cargo test …` rows: the learner's own tests (test-first problems) are run too.
        let runs_tests = md::code_spans(row.get(3).map_or("", |c| c.as_str()))
            .first()
            .is_some_and(|c| c.starts_with("cargo test"));
        let t = if test_writing || runs_tests {
            test_writing_row(&id, &row, &mutants, report)
        } else {
            test_row(&id, &row, &tests, &signatures, &fixtures, rust_rows, report)
        };
        if let Some(t) = t {
            tests.push(t);
        }
    }
    // Every numbered row in the Tests block must have been read, numbered 1, 2, 3, …
    let numbered = numbered_rows(&test_lines);
    if numbered != tests.len() {
        report.error(format!(
            "{id}: Tests has {numbered} numbered rows but {} were read; check the table's layout",
            tests.len()
        ));
    }
    for (i, t) in tests.iter().enumerate() {
        if t.n as usize != i + 1 {
            report.error(format!(
                "{id}: test row {} found where #{} was expected",
                t.n,
                i + 1
            ));
            break;
        }
    }
    // Rows may name an output rule defined in the approach ("DIE: stdout is exactly …").
    let names = output_names(&approach);
    for t in &mut tests {
        t.expected_md = expand_names(&t.expected_md, &names);
    }
    if tests.is_empty() {
        report.error(format!("{id}: no tests"));
    }
    let visible = tests.iter().filter(|t| t.visible).count();
    if visible < 2 {
        report.error(format!("{id}: {visible} visible tests (at least 2)"));
    }
    let min = match level {
        Level::WarmUp => 5,
        Level::Easy => 8,
        Level::Medium => 12,
        Level::Hard => 18,
        Level::Project => 20,
        Level::Milestone => 12,
    };
    if tests.len() < min {
        report.error(format!("{id}: {} test rows (minimum {min})", tests.len()));
    }

    Some(Problem {
        uid: id.clone(),
        id,
        title,
        level,
        xp,
        comes_after: get("Comes after"),
        kind,
        concepts: md::concept_ids(&get("Concepts tested")),
        reuses: md::code_spans(&get("Reuses"))
            .into_iter()
            .filter(|c| c.contains('.'))
            .collect(),
        starter_stage: get("Starter stage"),
        time_limit_ms,
        mode,
        isolated_target: false,
        description_md: description,
        examples_md: block("**Examples**"),
        constraints_md: block("**Constraints**"),
        hints,
        solution_approach_md: approach,
        idiomatic_md: idiomatic,
        random_md: block("**Random tests**"),
        source_checks_md: block("**Source checks**"),
        notes_md: block("**Notes**"),
        parts,
        checkpoints: false,
        fixtures: Vec::new(),
        checkers: Vec::new(),
        generators: Vec::new(),
        helpers: String::new(),
        starter,
        solution,
        part_solutions: Default::default(),
        wrong: Vec::new(),
        subject,
        mutants,
        tests,
    })
}

/// The **Mutants** table: `| M01 | title | what changed | a test that catches it |`. What
/// changed is one or more `` `before` → `after` `` replacements, separated by `;`, in the
/// subject file named first (`` `src/lib.rs`: … ``; the first subject file by default).
/// Each `before` must occur exactly once. A mutant too large for a cell is authored in the
/// overlay as whole files (`mutants/M05/src/lib.rs`), with the cell saying so.
fn mutant_rows(id: &str, lines: &[&str], subject: &[SourceFile], report: &mut Report) -> Vec<Mutant> {
    let Some(block) = md::block_after(lines, "**Mutants**") else {
        report.error(format!("{id}: a test-writing problem needs a **Mutants** table"));
        return Vec::new();
    };
    let mut out = Vec::new();
    for row in md::table_with_header(&block, "#") {
        if row.len() < 4 {
            report.error(format!("{id}: mutant row with {} cells", row.len()));
            continue;
        }
        let mid = row[0].trim().to_string();
        // Without the provided code here (it comes from the overlay), course-check applies
        // the change later.
        let files = if subject.is_empty() {
            Vec::new()
        } else {
            match rb_content::apply_change(subject, &row[2]) {
                Ok(f) => f,
                Err(e) => {
                    report.error(format!("{id} {mid}: {e}"));
                    continue;
                }
            }
        };
        out.push(Mutant {
            id: mid,
            title: row[1].clone(),
            change_md: row[2].clone(),
            files,
        });
    }
    if out.is_empty() {
        report.error(format!("{id}: the **Mutants** table has no rows"));
    }
    out
}

/// A test-writing problem's test row: `cargo test …` on the provided code (Expected:
/// `Every test passes`, `At least 3 tests run`, `2 tests run`, `1 test ignored`, joined
/// with `;`), or a mutant (`M01`, Expected `A test fails`). Other rows are authored.
fn test_writing_row(id: &str, row: &[String], mutants: &[Mutant], report: &mut Report) -> Option<TestCase> {
    if row.len() < 6 {
        report.error(format!("{id}: test row with {} cells", row.len()));
        return None;
    }
    let Ok(n) = row[0].parse::<u32>() else {
        report.error(format!("{id}: test row number `{}` isn't a number", row[0]));
        return None;
    };
    let (input, expected, checks) = (&row[3], &row[4], &row[5]);
    let concept = md::concept_ids(checks).into_iter().next().unwrap_or_default();
    if concept.is_empty() {
        report.error(format!("{id} #{n}: Checks cell has no concept ID"));
    }
    let mut input_md = input.clone();
    let spans = md::code_spans(input);
    let first = spans.first().map(String::as_str).unwrap_or("");
    let mutant = mutants.iter().find(|m| {
        input.trim() == m.id
            || first == m.id
            || input.trim().ends_with(&format!(": {}", m.id))
            || spans.iter().any(|s| s == &m.id)
    });
    let (run, asserts) = if let Some(m) = mutant {
        // Shown when revealed: what the planted bug is.
        input_md = format!("{} · {}: {}", m.id, m.title, m.change_md);
        let asserts = if expected.trim().trim_end_matches('.') == "A test fails" {
            vec![Assertion::MutantCaught]
        } else {
            vec![Assertion::Todo {
                note: expected.clone(),
            }]
        };
        (
            Run::Tests {
                mutant: Some(m.id.clone()),
                command: "-- --include-ignored".into(),
            },
            asserts,
        )
    } else if let Some(command) = first.strip_prefix("cargo test") {
        let mut asserts = Vec::new();
        for part in expected.split(';') {
            let p = part.trim().trim_end_matches('.');
            let words: Vec<&str> = p.split_whitespace().collect();
            let a = match words.as_slice() {
                ["Every", "test", "passes"] => Some(Assertion::TestsPass),
                ["At", "least", n, "tests" | "test", "run"] => n
                    .parse()
                    .ok()
                    .map(|n| Assertion::TestsRan { min: Some(n), eq: None }),
                [n, "tests" | "test", "run"] => n
                    .parse()
                    .ok()
                    .map(|n| Assertion::TestsRan { min: None, eq: Some(n) }),
                [n, "tests" | "test", "ignored"] => {
                    n.parse().ok().map(|eq| Assertion::TestsIgnored { eq })
                }
                [n, "tests" | "test", "filtered", "out"] => {
                    n.parse().ok().map(|eq| Assertion::TestsFiltered { eq })
                }
                // "`slow_check` is ignored", "`cheap_zero` passes", "`cheap_zero` doesn't run"
                [name, rest @ ..] if name.starts_with('`') && name.ends_with('`') && name.len() > 2 => {
                    let outcome = match rest {
                        ["passes"] => Some("passed"),
                        ["fails"] => Some("failed"),
                        ["is", "ignored"] => Some("ignored"),
                        ["doesn't", "run"] | ["is", "filtered", "out"] => Some("absent"),
                        _ => None,
                    };
                    outcome.map(|o| Assertion::TestOutcome {
                        name: name.trim_matches('`').to_string(),
                        outcome: o.to_string(),
                    })
                }
                _ => None,
            };
            match a {
                Some(a) => asserts.push(a),
                None => {
                    asserts = vec![Assertion::Todo {
                        note: expected.clone(),
                    }];
                    break;
                }
            }
        }
        (
            Run::Tests {
                mutant: None,
                command: command.trim().to_string(),
            },
            asserts,
        )
    } else {
        (
            Run::Todo,
            vec![Assertion::Todo {
                note: expected.clone(),
            }],
        )
    };
    let part_of = |cell: &str| -> Option<u32> {
        cell.split("Part ")
            .nth(1)
            .and_then(|s| s.split(|c: char| !c.is_ascii_digit()).next()?.parse().ok())
    };
    let part = part_of(&row[3]).or_else(|| part_of(checks));
    Some(TestCase {
        key: format!("t{n:02}"),
        n,
        visible: row[1] == "V",
        category: row[2].clone(),
        input_md,
        expected_md: expected.clone(),
        checks_md: checks.clone(),
        concept,
        part,
        run,
        asserts,
    })
}

/// Output rules defined as "NAME: text" (NAME is 3+ capitals); the text runs to the paragraph's end.
fn output_names(text: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for para in text.split("\n\n") {
        let para = para.trim();
        let mut from = 0;
        while let Some(i) = para[from..].find(": ").map(|i| i + from) {
            let start = para[..i]
                .rfind(|c: char| !(c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_'))
                .map_or(0, |s| s + 1);
            let name = &para[start..i];
            let at_boundary = start == 0 || para[..start].ends_with(' ');
            if name.len() >= 3 && name.starts_with(|c: char| c.is_ascii_uppercase()) && at_boundary
            {
                out.push((name.to_string(), para[i + 2..].trim().to_string()));
                break;
            }
            from = i + 2;
        }
    }
    out
}

/// Replace a cell that is just a NAME with its rule; append the rule where a cell mentions one.
fn expand_names(cell: &str, names: &[(String, String)]) -> String {
    let mut out = cell.to_string();
    for (name, def) in names {
        if cell.trim() == name {
            let mut c = def.chars();
            return c
                .next()
                .map_or(String::new(), |f| f.to_uppercase().chain(c).collect());
        }
        let mentioned = cell.match_indices(name.as_str()).any(|(i, _)| {
            let word = |c: char| c.is_ascii_alphanumeric() || c == '_';
            !cell[..i].ends_with(word) && !cell[i + name.len()..].starts_with(word)
        });
        if mentioned {
            out = format!("{out}. {name}: {}", def.trim_end_matches('.'));
            out.push('.');
        }
    }
    out
}

/// Starter code: one fence (rustc → `main.rs`) or several, each preceded by "File `path`:".
fn starter_files(lines: &[&str], mode: BuildMode) -> Vec<SourceFile> {
    let mut out = Vec::new();
    let mut pending: Option<String> = None;
    let mut i = 0;
    while i < lines.len() {
        let l = lines[i];
        // "File `src/main.rs`:" or "`src/main.rs` — …" names the next fence's file.
        let t = l.trim_start();
        if t.starts_with("File ") || (t.starts_with('`') && !md::is_fence(l)) {
            pending = md::code_spans(l).into_iter().next();
        }
        if md::is_fence(l) {
            let f = md::fences(&lines[i..]).into_iter().next().unwrap();
            let n = f.text.lines().count();
            let mut text = f.text.clone();
            let path = match pending.take() {
                Some(p) => p,
                None => {
                    // A leading `// src/main.rs` comment names the file.
                    let first = text.lines().next().unwrap_or("").trim().to_string();
                    match first
                        .strip_prefix("// ")
                        .filter(|p| p.ends_with(".rs") && !p.contains(' '))
                    {
                        Some(p) => {
                            let p = p.to_string();
                            text = text.lines().skip(1).collect::<Vec<_>>().join("\n");
                            p
                        }
                        None if mode == BuildMode::Cargo => "src/main.rs".into(),
                        None => "main.rs".into(),
                    }
                }
            };
            let path = if mode == BuildMode::Rustc {
                path.rsplit('/').next().unwrap_or(&path).to_string()
            } else {
                path
            };
            if !text.ends_with('\n') {
                text.push('\n');
            }
            out.push(SourceFile { path, text });
            i += n + 2;
            continue;
        }
        i += 1;
    }
    out
}

/// A test program from a `**P04-04-A1 · compiler fixture**` (or runtime fixture) section.
struct Fixture {
    id: String,
    main: String,
    /// Compiler fixtures: the error code compilation must fail with.
    error: Option<String>,
}

fn fixture_programs(lines: &[&str]) -> Vec<Fixture> {
    let mut out = Vec::new();
    for (i, l) in lines.iter().enumerate() {
        let Some(h) = l.strip_prefix("**").and_then(|h| h.strip_suffix("**")) else {
            continue;
        };
        let Some((id, kind)) = h.split_once(" · ") else {
            continue;
        };
        if kind != "compiler fixture" && kind != "runtime fixture" {
            continue;
        }
        let rest = &lines[i + 1..];
        let end = rest
            .iter()
            .position(|l| l.starts_with("**") && l.ends_with(" fixture**") || l.starts_with('|'))
            .unwrap_or(rest.len());
        let section = &rest[..end];
        let Some(code) = md::fences(section).into_iter().next() else {
            continue;
        };
        let error = section
            .iter()
            .find(|l| l.starts_with("Expected: compilation fails"))
            .and_then(|l| md::code_spans(l).into_iter().next());
        out.push(Fixture {
            id: id.trim().to_string(),
            main: code.text.trim_end().to_string(),
            error: if kind == "compiler fixture" {
                error
            } else {
                None
            },
        });
    }
    out
}

/// The Tests table's lines: from `**Tests**` to `**Random tests**` (or the next section),
/// keeping table lines outside code fences. Rows may continue after prose and fixture
/// listings between them.
fn tests_region<'a>(lines: &[&'a str]) -> Vec<&'a str> {
    let Some(start) = lines.iter().position(|l| l.starts_with("**Tests**")) else {
        return Vec::new();
    };
    let ends = [
        "**Mutants**",
        "**Random tests**",
        "**Source checks**",
        "**Notes**",
        "## ",
        "### ",
    ];
    let mut out = Vec::new();
    let mut in_fence = false;
    for &l in &lines[start + 1..] {
        if md::is_fence(l) {
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            continue;
        }
        if ends.iter().any(|e| l.starts_with(e)) {
            break;
        }
        if l.trim_start().starts_with('|') {
            out.push(l);
        }
    }
    out
}

/// Table rows outside code fences whose first cell is a number.
fn numbered_rows(lines: &[&str]) -> usize {
    let mut in_fence = false;
    lines
        .iter()
        .filter(|l| {
            if md::is_fence(l) {
                in_fence = !in_fence;
            }
            !in_fence
                && l.trim_start().starts_with('|')
                && md::split_row(l)
                    .first()
                    .is_some_and(|c| c.parse::<u32>().is_ok())
        })
        .count()
}

fn test_row(
    id: &str,
    row: &[String],
    earlier: &[TestCase],
    signatures: &[Signature],
    fixtures: &[Fixture],
    rust_rows: Option<&RustRows>,
    report: &mut Report,
) -> Option<TestCase> {
    if row.len() < 6 {
        report.error(format!("{id}: test row with {} cells", row.len()));
        return None;
    }
    let Ok(n) = row[0].parse::<u32>() else {
        report.error(format!("{id}: test row number `{}` isn't a number", row[0]));
        return None;
    };
    let visible = row[1] == "V";
    let category = row[2].clone();
    let (input, expected, checks) = (&row[3], &row[4], &row[5]);
    let concept = md::concept_ids(checks)
        .into_iter()
        .next()
        .unwrap_or_default();
    if concept.is_empty() {
        report.error(format!("{id} #{n}: Checks cell has no concept ID"));
    }
    let part_of = |cell: &str| -> Option<u32> {
        cell.split("Part ")
            .nth(1)
            .and_then(|s| s.split(|c: char| !c.is_ascii_digit()).next()?.parse().ok())
    };
    let part = part_of(input).or_else(|| part_of(checks));

    let fixture = md::code_spans(input)
        .first()
        .and_then(|id| fixtures.iter().find(|f| &f.id == id));
    let rust = rust_rows.and_then(|r| rust_row(input, expected, &r.names));
    let (run, asserts) = match (fixture, call_of(input, signatures)) {
        _ if rust.is_some() => rust.unwrap(),
        (Some(f), _) => (
            Run::Fixture {
                main: f.main.clone(),
                remove: Vec::new(),
            },
            match &f.error {
                Some(code) => vec![Assertion::CompileError {
                    code: code.clone(),
                    message: None,
                }],
                None => vec![Assertion::StdoutEq {
                    text: String::new(),
                }],
            },
        ),
        (None, call) => match call {
            Some(row) if needs_harness(expected, &row) => {
                match expected_expr(expected).map(|e| with_bindings(&e, &row.bindings)) {
                    Some(e) => (
                        Run::Call {
                            call: row.call,
                            expect: Some(e),
                            typed: false,
                        },
                        vec![Assertion::Returns {
                            value: None,
                            tol: None,
                        }],
                    ),
                    None => (
                        Run::Call {
                            call: row.call,
                            expect: None,
                            typed: false,
                        },
                        vec![Assertion::Todo {
                            note: expected.to_string(),
                        }],
                    ),
                }
            }
            Some(row) => {
                let value = (!expected.contains("stdout"))
                    .then(|| expected_expr(expected))
                    .flatten();
                let asserts = match value {
                    Some(value) => vec![Assertion::Returns {
                        value: Some(plain_number(&value)),
                        tol: None,
                    }],
                    None => vec![Assertion::Todo {
                        note: expected.to_string(),
                    }],
                };
                (
                    Run::Call {
                        call: row.call,
                        expect: None,
                        typed: false,
                    },
                    asserts,
                )
            }
            None => (run_of(input), asserts_of(expected, earlier)),
        },
    };
    Some(TestCase {
        key: format!("t{n:02}"),
        n,
        visible,
        category,
        input_md: input.clone(),
        expected_md: expected.clone(),
        checks_md: checks.clone(),
        concept,
        part,
        run,
        asserts,
    })
}

/// A row written as Rust: `setup; final expression` with an exact Rust expected value
/// (optionally "(absolute tolerance ±1e-9)"), or "Compiler probe: `caller`" with the
/// error code the compiler must give. Other rows (source rules, suites) are left to author.
fn rust_row(
    input: &str,
    expected: &str,
    names: &[(String, String)],
) -> Option<(Run, Vec<Assertion>)> {
    // Binds the named text fixtures an expression uses.
    let bind = |code: &str| -> String {
        let lets: String = names
            .iter()
            .filter(|(n, _)| mentions(code, n))
            .map(|(n, e)| format!("let {n}: &'static str = Box::leak({e}.into_boxed_str()); "))
            .collect();
        if lets.is_empty() {
            code.to_string()
        } else if let Some(inner) = code.strip_prefix('{').and_then(|c| c.strip_suffix('}')) {
            // Inside the row's own block, so its setup and final expression stay together.
            format!("{{ {lets}{} }}", inner.trim())
        } else {
            format!("{{ {lets}{code} }}")
        }
    };
    let only_span = |cell: &str| -> Option<String> {
        let spans = md::code_spans(cell);
        (spans.len() == 1 && cell.trim().starts_with('`')).then(|| spans[0].trim().to_string())
    };
    if let Some(probe) = input.trim().strip_prefix("Compiler probe:") {
        let caller = only_span(probe)?;
        let code = expected
            .split(|c: char| !c.is_ascii_alphanumeric())
            .find(|w| {
                w.len() == 5 && w.starts_with('E') && w[1..].bytes().all(|b| b.is_ascii_digit())
            })?;
        return Some((
            Run::Fixture {
                main: format!("fn main() {{\n    {caller}\n}}"),
                remove: Vec::new(),
            },
            vec![Assertion::CompileError {
                code: code.to_string(),
                message: None,
            }],
        ));
    }
    let code = only_span(input)?;
    // Command-line rows (`args: […]`, `file …`, `env …`, `stdin: …`) aren't Rust.
    if ["args:", "file ", "env ", "stdin:"]
        .iter()
        .any(|p| code.starts_with(p))
    {
        return None;
    }
    // Manifest rows (`Cargo.toml`, `cli/Cargo.toml`) and Cargo command rows aren't Rust.
    if code == "Cargo.toml" || code.ends_with("/Cargo.toml") || code.starts_with("cargo ") {
        return None;
    }
    let t = expected.trim();
    let call = if code.starts_with('{') && code.ends_with('}') {
        code.clone()
    } else {
        format!("{{ {code} }}")
    };
    // `panics with "…"`: the call panics, and its message contains the text.
    if let Some(text) = md::quoted_after(t, "panics with") {
        return Some((
            Run::Call {
                call: bind(&call),
                expect: None,
                typed: false,
            },
            vec![
                Assertion::StderrContains { text },
                Assertion::ExitCode { code: 101 },
            ],
        ));
    }
    let value = t.strip_prefix('`')?;
    let (value, rest) = value.split_once('`')?;
    let rest = rest.trim();
    let tol = match rest.strip_prefix("(absolute tolerance ±") {
        Some(t) => Some(t.trim_end_matches(')').trim().parse::<f64>().ok()?),
        None if rest.is_empty() => None,
        None => return None,
    };
    Some((
        Run::Call {
            call: bind(&call),
            expect: Some(bind(value.trim())),
            typed: true,
        },
        vec![Assertion::Returns { value: None, tol }],
    ))
}

/// A public function's name, parameters (name, type) and return type, from the starter code.
#[derive(Debug)]
struct Signature {
    name: String,
    params: Vec<(String, String)>,
    ret: String,
}

fn signatures(code: &str) -> Vec<Signature> {
    let mut out = Vec::new();
    for l in code.lines() {
        let Some(rest) = l.trim_start().strip_prefix("pub fn ") else {
            continue;
        };
        let Some((name, rest)) = rest.split_once('(') else {
            continue;
        };
        // The parameter list ends at the `)` matching the opening one.
        let mut depth = 0;
        let Some(close) = rest.char_indices().find_map(|(i, c)| {
            match c {
                '(' | '[' => depth += 1,
                ')' | ']' if depth > 0 => depth -= 1,
                ')' => return Some(i),
                _ => {}
            }
            None
        }) else {
            continue;
        };
        let (params, after) = (&rest[..close], &rest[close + 1..]);
        let params = split_top(params)
            .iter()
            .filter_map(|p| p.split_once(':'))
            .map(|(n, t)| {
                let n = n.trim().trim_start_matches("mut ").trim().to_string();
                (n, t.trim().to_string())
            })
            .collect();
        let ret = after
            .trim()
            .strip_prefix("->")
            .map(|r| r.trim().trim_end_matches('{').trim().to_string())
            .unwrap_or_else(|| "()".into());
        out.push(Signature {
            name: name.trim().to_string(),
            params,
            ret,
        });
    }
    out
}

/// Splits on commas outside brackets, parentheses and quotes.
fn split_top(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut cur = String::new();
    let mut quote: Option<char> = None;
    let mut escaped = false;
    for c in s.chars() {
        if let Some(q) = quote {
            cur.push(c);
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == q {
                quote = None;
            }
            continue;
        }
        match c {
            '\'' | '"' => quote = Some(c),
            '(' | '[' | '{' | '<' => depth += 1,
            ')' | ']' | '}' | '>' => depth -= 1,
            ',' if depth == 0 => {
                out.push(cur.trim().to_string());
                cur.clear();
                continue;
            }
            _ => {}
        }
        cur.push(c);
    }
    if !cur.trim().is_empty() {
        out.push(cur.trim().to_string());
    }
    out
}

/// A converted function-call row: the call expression and, when the expected value
/// isn't plain text a returned value prints as, a Rust expression for it.
struct CallRow {
    call: String,
    /// `let name = value;` for each argument, for expected values that mention them.
    bindings: Vec<(String, String)>,
}

/// A function-call row: `width = 3.0, height = 4.0` (arguments by name) or a call
/// `gcd(48, 18)` of one of the file's public functions.
///
/// Arguments become values of the parameter's type: text becomes `String::from(…)`
/// (borrowed for `&str`/`&String`), `"x" repeated 3 times` becomes `"x".repeat(3)`.
/// A `&mut` parameter gets a local the call changes; the row's result is then that
/// local's final value (with the returned value, unless it's `()`).
fn call_of(input: &str, sigs: &[Signature]) -> Option<CallRow> {
    let spans = md::code_spans(input);
    let [span] = spans.as_slice() else {
        return None;
    };
    let span = plain_counts(span.trim());
    let mut args = Vec::new();
    let sig = if let Some(sig) = sigs.iter().find(|s| {
        span.strip_prefix(s.name.as_str())
            .is_some_and(|r| r.starts_with('(') && r.ends_with(')'))
    }) {
        // `gcd(48, 18)`: arguments by position.
        let inner = &span[sig_name_len(&span, sig)..span.len() - 1];
        let values = split_top(inner);
        if values.len() != sig.params.len() {
            return None;
        }
        for ((p, _), v) in sig.params.iter().zip(values) {
            args.push((p.clone(), v));
        }
        sig
    } else {
        for part in split_top(&span) {
            let (name, value) = part.split_once(" = ")?;
            args.push((name.trim().to_string(), value.trim().to_string()));
        }
        // The function whose parameters are exactly the named arguments.
        sigs.iter().find(|s| {
            s.params.len() == args.len()
                && s.params
                    .iter()
                    .all(|(p, _)| args.iter().any(|(n, _)| n == p))
        })?
    };
    let mut bindings = Vec::new();
    let mut lets = Vec::new();
    let mut passed = Vec::new();
    let mut changed = Vec::new();
    let mut converted = false;
    for (i, (p, ty)) in sig.params.iter().enumerate() {
        let value = &args.iter().find(|(n, _)| n == p)?.1;
        let text = value_expr(value);
        let ty = ty.replace(' ', "");
        let owned = if ty.ends_with("String") || ty == "&str" {
            format!("String::from({text})")
        } else {
            text.clone()
        };
        bindings.push((p.clone(), owned));
        let arg = match ty.as_str() {
            "String" => format!("String::from({text})"),
            "&str" | "&String" => format!("&String::from({text})"),
            "&mutString" => {
                lets.push(format!("let mut a{i} = String::from({text});"));
                changed.push(format!("a{i}"));
                format!("&mut a{i}")
            }
            t if t.starts_with("&mut[") => {
                let elem = t.trim_start_matches("&mut[").trim_end_matches(']');
                // The array's length, so its element type can be written out.
                let inner = value.trim().trim_start_matches('[').trim_end_matches(']');
                let len = match inner.rsplit_once("; ") {
                    Some((_, n)) if !inner.contains(',') => n.replace(',', ""),
                    _ => split_top(inner).len().to_string(),
                };
                let decl = format!("let mut a{i}: [{elem}; {len}] = {text};");
                lets.push(decl);
                changed.push(format!("a{i}"));
                format!("&mut a{i}")
            }
            t if t.starts_with("&[") => format!("&{text}"),
            _ => text.clone(),
        };
        converted |= arg != *value;
        passed.push(arg);
    }
    let call = format!("{}({})", sig.name, passed.join(", "));
    if !converted {
        return Some(CallRow { call, bindings });
    }
    let call = if changed.is_empty() {
        call
    } else {
        let mut result = changed.clone();
        let unit = sig.ret.replace(' ', "") == "()";
        if !unit {
            result.push("r".into());
        }
        let result = if result.len() == 1 {
            result[0].clone()
        } else {
            format!("({})", result.join(", "))
        };
        let r = if unit { "_" } else { "r" };
        format!("{{ {} let {r} = {call}; {result} }}", lets.join(" "))
    };
    Some(CallRow { call, bindings })
}

/// Length of `name` at the start of a direct call `name(…)`.
fn sig_name_len(span: &str, sig: &Signature) -> usize {
    debug_assert!(span.starts_with(&sig.name));
    sig.name.len() + 1
}

/// `4_294_967_295` → `4294967295`: values print without digit separators.
fn plain_number(v: &str) -> String {
    let chars: Vec<char> = v.chars().collect();
    chars
        .iter()
        .enumerate()
        .filter(|&(i, &c)| {
            !(c == '_'
                && i > 0
                && chars[i - 1].is_ascii_digit()
                && chars.get(i + 1).is_some_and(|n| n.is_ascii_digit()))
        })
        .map(|(_, c)| *c)
        .collect()
}

/// The expected value must be made by the harness too: it's text (whose `{:?}` the spec
/// doesn't spell out), built up, an `[x; n]` array, or mentions the arguments.
fn needs_harness(expected: &str, row: &CallRow) -> bool {
    let Some(e) = expected_expr(expected) else {
        return expected.contains('"');
    };
    e.contains('"')
        || e.contains(".repeat(")
        || (e.contains('[') && e.contains("; "))
        || row.bindings.iter().any(|(n, _)| mentions(&e, n))
}

/// `cell` uses `name` as a word outside string literals.
fn mentions(cell: &str, name: &str) -> bool {
    let mut code = String::new();
    let mut in_str = false;
    for c in cell.chars() {
        if c == '"' {
            in_str = !in_str;
        } else if !in_str {
            code.push(c);
        }
    }
    code.split(|c: char| !(c.is_alphanumeric() || c == '_'))
        .any(|w| w == name)
}

/// `expr` in a block that first binds the arguments it mentions (`text + "!"`).
fn with_bindings(expr: &str, bindings: &[(String, String)]) -> String {
    let mut code = String::new();
    let mut in_str = false;
    let mut escaped = false;
    for c in expr.chars() {
        if in_str {
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_str = false;
            }
        } else if c == '"' {
            in_str = true;
        } else {
            code.push(c);
        }
    }
    let used: Vec<String> = bindings
        .iter()
        .filter(|(name, _)| {
            code.split(|c: char| !(c.is_alphanumeric() || c == '_'))
                .any(|w| w == name)
        })
        .map(|(name, value)| format!("let {name} = {value};"))
        .collect();
    if used.is_empty() {
        expr.to_string()
    } else {
        format!("{{ {} {expr} }}", used.join(" "))
    }
}

/// "repeated 100,000 times" → "repeated 100000 times" (and `[2; 100,000]` → `[2; 100000]`),
/// so counts don't split arguments.
fn plain_counts(s: &str) -> String {
    let s = array_counts(s);
    let s = s.as_str();
    let mut out = String::new();
    let mut rest = s;
    while let Some(i) = rest.find("repeated ") {
        let after = &rest[i + "repeated ".len()..];
        let n = after.find(" times").unwrap_or(0);
        out.push_str(&rest[..i + "repeated ".len()]);
        out.push_str(&after[..n].replace(',', ""));
        rest = &after[n..];
    }
    out.push_str(rest);
    out
}

fn array_counts(s: &str) -> String {
    let mut out = String::new();
    let mut rest = s;
    while let Some(i) = rest.find("; ") {
        let after = &rest[i + 2..];
        let n = after.find(']').unwrap_or(0);
        out.push_str(&rest[..i + 2]);
        let count = &after[..n];
        if count.chars().all(|c| c.is_ascii_digit() || c == ',') {
            out.push_str(&count.replace(',', ""));
        } else {
            out.push_str(count);
        }
        rest = &after[n..];
    }
    out.push_str(rest);
    out
}

/// A cell value as a Rust expression: tuples element by element, and text built from
/// `"a" repeated 3 times + "b"` pieces.
fn value_expr(v: &str) -> String {
    let v = plain_counts(v.trim());
    let v = v.trim();
    if v.starts_with('(') && v.ends_with(')') {
        let parts = split_top(&v[1..v.len() - 1]);
        if parts.len() > 1 {
            let items: Vec<String> = parts.iter().map(|p| value_expr(p)).collect();
            return format!("({})", items.join(", "));
        }
    }
    if !v.contains("repeated") {
        return v.to_string();
    }
    let pieces: Vec<String> = v
        .split(" + ")
        .map(|t| match t.split_once(" repeated ") {
            Some((lit, n)) => {
                let n = n.trim_end_matches(" times").trim();
                if n.chars().all(|c| c.is_ascii_digit()) {
                    format!("{}.repeat({n})", lit.trim())
                } else {
                    format!("{}.repeat({n} as usize)", lit.trim())
                }
            }
            None => t.trim().to_string(),
        })
        .collect();
    if pieces.len() == 1 {
        return pieces[0].clone();
    }
    format!(
        "format!(\"{}\", {})",
        "{}".repeat(pieces.len()),
        pieces.join(", ")
    )
}

/// The expected value of a call row as a Rust expression, from cells such as
/// `("hi##", 4)`, `"[" repeated 3 times + "x"`, `Caller-owned value after call: "Ana!"; return ()`
/// and `(caller-owned text after call, returned old byte length) = ("one two!", 7)`.
fn expected_expr(cell: &str) -> Option<String> {
    let spans = md::code_spans(cell);
    let [span] = spans.as_slice() else {
        return None;
    };
    let s = span.trim();
    let s = match s.strip_prefix("Caller-owned value after call:") {
        Some(rest) => rest
            .trim()
            .strip_suffix("; return ()")
            .unwrap_or(rest)
            .trim(),
        None => match s.starts_with('(').then(|| s.split_once(") = ")).flatten() {
            Some((_, value)) => value.trim(),
            None => s,
        },
    };
    // A repeated group, `("|" + message) repeated 3 times`, is written by hand.
    if s.contains("+ (") {
        return None;
    }
    // Prose ("the exact result constructed by …") isn't a value.
    let mut code = String::new();
    let mut quote = None;
    let mut escaped = false;
    for c in s.chars() {
        match quote {
            Some(q) => {
                if escaped {
                    escaped = false;
                } else if c == '\\' {
                    escaped = true;
                } else if c == q {
                    quote = None;
                }
            }
            None if c == '"' || c == '\'' => quote = Some(c),
            None => code.push(c),
        }
    }
    let words = code
        .split(|c: char| !(c.is_alphanumeric() || c == '_'))
        .filter(|w| w.chars().next().is_some_and(|c| c.is_alphabetic()))
        .filter(|w| !["true", "false", "repeated", "times"].contains(w));
    if words.count() > 3 || code.contains('…') {
        return None;
    }
    Some(value_expr(s))
}

fn run_of(input: &str) -> Run {
    if let Some(stdin) = md::quoted_after(input, "stdin:") {
        return Run::process(stdin);
    }
    let spans = md::code_spans(input);
    if spans
        .iter()
        .any(|s| s.starts_with("args:") || s.starts_with("file ") || s.starts_with("env "))
    {
        return command_run(&spans).unwrap_or(Run::Todo);
    }
    let first = spans.first().map(String::as_str).unwrap_or("");
    if first == "cargo run" {
        let stdin = spans
            .iter()
            .find_map(|s| md::quoted_after(s, "stdin:"))
            .unwrap_or_default();
        return Run::process(stdin);
    }
    if first == "cargo build"
        || first == "cargo check"
        || first.starts_with("cargo build ")
        || first.starts_with("cargo check ")
        || first.starts_with("cargo doc")
    {
        return Run::Build {
            command: first.trim_start_matches("cargo ").to_string(),
        };
    }
    if first == "Cargo.toml" || first.ends_with("/Cargo.toml") {
        return Run::Manifest;
    }
    if input.contains("learner's source")
        || input.contains("submitted")
        || input.contains("parsed manifest")
    {
        return Run::Source;
    }
    Run::Todo
}

/// A command-line run: `` `args: ["-n", "2", "poem.txt"]` ``, `` `file poem.txt: "a\nb\n"` ``
/// (or `` `file data: bytes [255, 0]` ``), `` `env IGNORE_CASE: "1"` `` and `` `stdin: "…"` ``,
/// each in its own code span. The program runs in a fresh folder holding only those files.
fn command_run(spans: &[String]) -> Option<Run> {
    let mut args = Vec::new();
    let mut files = Vec::new();
    let mut env = std::collections::BTreeMap::new();
    let mut stdin = String::new();
    for s in spans {
        if let Some(rest) = s.strip_prefix("args:") {
            args = serde_json::from_str::<Vec<String>>(rest.trim()).ok()?;
        } else if let Some(rest) = s.strip_prefix("file ") {
            let (path, value) = rest.split_once(": ")?;
            let value = value.trim();
            let file = match value.strip_prefix("bytes ") {
                Some(b) => FileFixture {
                    path: path.trim().to_string(),
                    text: String::new(),
                    bytes: Some(serde_json::from_str::<Vec<u8>>(b.trim()).ok()?),
                },
                None => FileFixture {
                    path: path.trim().to_string(),
                    text: md::quoted_after(value, "")?,
                    bytes: None,
                },
            };
            files.push(file);
        } else if let Some(rest) = s.strip_prefix("env ") {
            let (name, value) = rest.split_once(": ")?;
            env.insert(name.trim().to_string(), md::quoted_after(value.trim(), "")?);
        } else if let Some(text) = md::quoted_after(s, "stdin:") {
            stdin = text;
        }
    }
    Some(Run::Process {
        stdin,
        stdin_pieces: Vec::new(),
        args,
        fixture: std::collections::BTreeMap::new(),
        times: 1,
        files,
        env,
    })
}

fn asserts_of(expected: &str, earlier: &[TestCase]) -> Vec<Assertion> {
    // Exit 0 and empty stderr are what every run is checked for anyway.
    let expected = expected
        .trim()
        .strip_suffix("; exit 0; empty stderr")
        .unwrap_or(expected);
    if let Some(text) = md::quoted_after(expected, "stdout:") {
        return vec![Assertion::StdoutEq { text }];
    }
    // `stdout: "…"; stderr contains "…"; exit code N`: an expected failure.
    if let Some(asserts) = failure_asserts(expected) {
        return asserts;
    }
    let e = expected.trim();
    if let Some(rest) = e.strip_prefix("stdout line ")
        && let Some((n, tail)) = rest.split_once(" is ")
        && let (Ok(line), Some(text)) = (n.trim().parse(), md::single_quoted(tail.trim()))
    {
        return vec![Assertion::LineEq { line, text }];
    }
    if e == "stderr is empty; exit code 0" {
        return vec![Assertion::StderrEmpty, Assertion::ExitCode { code: 0 }];
    }
    if let Some(n) = e
        .strip_prefix("the same stdout as test ")
        .and_then(|s| s.trim().parse::<u32>().ok())
        && let Some(t) = earlier.iter().find(|t| t.n == n)
    {
        return t.asserts.clone();
    }
    vec![Assertion::Todo {
        note: expected.to_string(),
    }]
}

fn failure_asserts(expected: &str) -> Option<Vec<Assertion>> {
    let mut asserts = Vec::new();
    // The whole cell may be one code span: `` `stderr: "…"; exit code 2` ``.
    let t = expected.trim();
    let unwrapped = if t.starts_with('`') && t.ends_with('`') && md::code_spans(t).len() == 1 {
        md::unbacktick(t)
    } else {
        t.to_string()
    };
    for part in unwrapped.split("; ") {
        let part = part.trim();
        if let Some(text) = md::quoted_after(part, "stdout:") {
            asserts.push(Assertion::StdoutEq { text });
        } else if let Some(text) = md::quoted_after(part, "stderr contains") {
            asserts.push(Assertion::StderrContains { text });
        } else if let Some(text) = md::quoted_after(part, "stderr starts with") {
            asserts.push(Assertion::StderrStartsWith { text });
        } else if let Some(text) = md::quoted_after(part, "stderr:") {
            asserts.push(Assertion::StderrEq { text });
        } else {
            let code = part.strip_prefix("exit code ")?.trim().parse().ok()?;
            asserts.push(Assertion::ExitCode { code });
        }
    }
    asserts
        .iter()
        .any(|a| matches!(a, Assertion::ExitCode { .. }))
        .then_some(asserts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_line_rows() {
        let run = run_of(r#"`args: ["-n", "2", "poem.txt"]` `file poem.txt: "a\nb\n"` `file bad: bytes [255]` `env IGNORE_CASE: "1"`"#);
        let Run::Process { args, files, env, .. } = run else {
            panic!("expected a process run, got {run:?}");
        };
        assert_eq!(args, ["-n", "2", "poem.txt"]);
        assert_eq!(files[0].text, "a\nb\n");
        assert_eq!(files[1].bytes, Some(vec![255]));
        assert_eq!(env["IGNORE_CASE"], "1");
        let a = asserts_of(r#"stderr: "head: missing file path\n"; exit code 2"#, &[]);
        assert!(matches!(&a[0], Assertion::StderrEq { text } if text == "head: missing file path\n"));
        let a = asserts_of(r#"stderr starts with "head: absent: "; exit code 1"#, &[]);
        assert!(matches!(&a[0], Assertion::StderrStartsWith { text } if text == "head: absent: "));
    }

    #[test]
    fn function_rows_become_calls() {
        let sigs = signatures(
            "pub fn swap_pair(number: i32, marker: char) -> (char, i32) {\npub fn find_cell(grid: [[u8; 3]; 3], target: u8) -> (i32, i32) {",
        );
        assert_eq!(
            sigs[0].params[1],
            ("marker".to_string(), "char".to_string())
        );
        assert_eq!(sigs[1].params[0].0, "grid");
        let one = &sigs[..1];
        assert_eq!(
            call_of("`marker = ',', number = -2`", one)
                .map(|r| r.call)
                .as_deref(),
            Some("swap_pair(-2, ',')")
        );
        assert_eq!(
            call_of(
                "`grid = [[1, 2, 3], [4, 5, 6], [7, 8, 9]], target = 5`",
                &sigs[1..]
            )
            .map(|r| r.call)
            .as_deref(),
            Some("find_cell([[1, 2, 3], [4, 5, 6], [7, 8, 9]], 5)")
        );
        assert_eq!(
            call_of("Part 2: `find_cell([[0; 3]; 3], 1)`", &sigs)
                .map(|r| r.call)
                .as_deref(),
            Some("find_cell([[0; 3]; 3], 1)")
        );
        assert!(call_of("`stdin: \"3\\n\"`", one).is_none());
    }

    #[test]
    fn typed_arguments_and_expected_values() {
        let sigs = signatures(
            "pub fn decorate(text: &mut String) -> usize {\npub fn first_word(text: &str) -> &str {\npub fn adjust(scores: &mut [u32], bonus: u32) -> () {",
        );
        let row = call_of("`text = \"one two\"`", &sigs[..1]).unwrap();
        assert_eq!(
            row.call,
            "{ let mut a0 = String::from(\"one two\"); let r = decorate(&mut a0); (a0, r) }"
        );
        let row = call_of("`text = \"x\" repeated 1,000 times`", &sigs[1..2]).unwrap();
        assert_eq!(row.call, "first_word(&String::from(\"x\".repeat(1000)))");
        let row = call_of("`scores = [1, 2], bonus = 3`", &sigs[2..]).unwrap();
        assert_eq!(
            row.call,
            "{ let mut a0: [u32; 2] = [1, 2]; let _ = adjust(&mut a0, 3); a0 }"
        );
        assert_eq!(
            expected_expr("`Caller-owned value after call: \"Ana!\"; return ()`").as_deref(),
            Some("\"Ana!\"")
        );
        assert_eq!(
            expected_expr("`\"[\" repeated 2 times + \"x\"`").as_deref(),
            Some("format!(\"{}{}\", \"[\".repeat(2), \"x\")")
        );
        assert_eq!(
            expected_expr("`the exact result constructed by the contract`"),
            None
        );
        assert_eq!(plain_number("4_294_967_295"), "4294967295");
    }

    #[test]
    fn panic_and_failure_rows() {
        let (run, asserts) = rust_row(
            "`crate::positive(0)`",
            "panics with \"count must be positive\"",
            &[],
        )
        .unwrap();
        assert!(matches!(run, Run::Call { expect: None, .. }));
        assert!(matches!(&asserts[..], [
            Assertion::StderrContains { text },
            Assertion::ExitCode { code: 101 }
        ] if text == "count must be positive"));
        let asserts = asserts_of(
            "`stdout: \"\"`; stderr contains `\"Error: Empty\"`; exit code 1",
            &[],
        );
        assert!(matches!(&asserts[..], [
            Assertion::StdoutEq { text },
            Assertion::StderrContains { .. },
            Assertion::ExitCode { code: 1 }
        ] if text.is_empty()));
    }

    #[test]
    fn rust_rows_are_used_as_written() {
        let names = vec![(
            "text_2".to_string(),
            fixture_text("Prefix \"!\"; ASCII `a` repeated exactly 3 times").unwrap(),
        )];
        assert_eq!(names[0].1, "[\"!\", \"a\".repeat(3).as_str()].concat()");
        let (run, asserts) = rust_row(
            "`let mut p=make(1); bump(&mut p); (p.x,p.y)`",
            "`(2,4294967295)`",
            &names,
        )
        .unwrap();
        assert_eq!(
            run,
            Run::Call {
                call: "{ let mut p=make(1); bump(&mut p); (p.x,p.y) }".into(),
                expect: Some("(2,4294967295)".into()),
                typed: true,
            }
        );
        assert!(matches!(
            asserts[0],
            Assertion::Returns {
                value: None,
                tol: None
            }
        ));
        let (run, _) = rust_row("`text_2.len()`", "`4`", &names).unwrap();
        let Run::Call { call, .. } = run else {
            panic!()
        };
        assert!(call.starts_with("{ let text_2: &'static str = Box::leak([\"!\""));
        assert!(call.ends_with("text_2.len() }"), "{call}");
        let (_, asserts) =
            rust_row("`f(1.0).0`", "`33.8` (absolute tolerance ±1e-9)", &names).unwrap();
        assert!(matches!(asserts[0], Assertion::Returns { tol: Some(t), .. } if t == 1e-9));
        let (run, asserts) = rust_row(
            "Compiler probe: `let a=P{x:1};let b=a;a.x;`",
            "`compiler rejects caller with E0382`",
            &names,
        )
        .unwrap();
        assert!(matches!(run, Run::Fixture { .. }));
        assert!(matches!(&asserts[0], Assertion::CompileError { code, .. } if code == "E0382"));
        assert!(rust_row("Parsed-source rule: `x`", "`y`", &names).is_none());
    }

    #[test]
    fn expands_output_names() {
        let names = output_names("Print the binding. DIE: stdout is one of `\"1\"`, `\"2\"`.");
        assert_eq!(
            names,
            vec![(
                "DIE".to_string(),
                "stdout is one of `\"1\"`, `\"2\"`.".to_string()
            )]
        );
        assert_eq!(
            expand_names("DIE", &names),
            "Stdout is one of `\"1\"`, `\"2\"`."
        );
        assert_eq!(
            expand_names("Every run satisfies DIE; no coverage", &names),
            "Every run satisfies DIE; no coverage. DIE: stdout is one of `\"1\"`, `\"2\"`."
        );
        assert_eq!(expand_names("DIEHARD", &names), "DIEHARD");
    }
}
