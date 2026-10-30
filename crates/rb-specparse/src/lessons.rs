//! Lessons (lessons.md).

use crate::md::{self, Section};
use crate::{Report, questions};
use rb_content::*;
use std::collections::BTreeMap;

pub fn lessons(text: &str, report: &mut Report) -> Vec<Lesson> {
    let lines: Vec<&str> = text.lines().collect();
    md::sections(&lines, 2)
        .into_iter()
        .filter(|s| s.heading.starts_with('L'))
        .filter_map(|s| lesson(&s, report))
        .collect()
}

fn lesson(sec: &Section, report: &mut Report) -> Option<Lesson> {
    let parts = md::dot_parts(&sec.heading);
    let id = parts[0].clone();
    let title = parts.get(1).cloned().unwrap_or_default();
    let fields = md::fields(&sec.lines);
    let get = |k: &str| md::field(&fields, k).unwrap_or("").to_string();
    let minutes = get("Screens")
        .split("~")
        .nth(1)
        .and_then(|s| s.split_whitespace().next())
        .and_then(|n| n.parse().ok())
        .unwrap_or(7);
    let objective_md = sec
        .lines
        .iter()
        .find(|l| l.starts_with("**Objective:**"))
        .map(|l| l["**Objective:**".len()..].trim().to_string())
        .unwrap_or_default();

    let mut screens: Vec<Screen> = Vec::new();
    let mut summary = Vec::new();
    let mut completion = Completion {
        xp: 20,
        button: String::new(),
        first_try: String::new(),
        many_tries: String::new(),
    };
    for s in md::sections(&sec.lines, 3) {
        let h = md::dot_parts(&s.heading);
        if h[0].starts_with("Screen") {
            if let Some(mut screen) = screen(&id, &h, &s.lines, &screens, report) {
                screen.key = format!("s{}", h[0].trim_start_matches("Screen").trim());
                screens.push(screen);
            }
        } else if h[0].starts_with("Quick check") {
            for q in md::sections(&s.lines, 5) {
                if let Some(q) = questions::question(&q, report) {
                    let k = screens
                        .iter()
                        .filter(|s| s.kind == ScreenKind::Check)
                        .count()
                        + 1;
                    screens.push(Screen {
                        key: format!("qc{k}"),
                        kind: ScreenKind::Check,
                        heading: String::new(),
                        text_md: String::new(),
                        code: None,
                        files: Vec::new(),
                        coming_from: BTreeMap::new(),
                        panels: Vec::new(),
                        steps: Vec::new(),
                        diagnostic: None,
                        annotations: Vec::new(),
                        compare: None,
                        question: Some(q),
                        visual_md: None,
                    });
                }
            }
        } else if h[0] == "Summary" {
            summary = s
                .lines
                .iter()
                .filter_map(|l| l.trim().strip_prefix("- "))
                .map(str::to_string)
                .collect();
        } else if h[0] == "Completion" {
            for l in &s.lines {
                let Some(item) = l.trim().strip_prefix("- ") else {
                    continue;
                };
                let quoted = || {
                    item.split_once(':')
                        .map(|(_, v)| v.trim().trim_matches('"').to_string())
                        .unwrap_or_default()
                };
                if item.starts_with("XP:") {
                    completion.xp = quoted().parse().unwrap_or(20);
                } else if item.starts_with("Primary button") {
                    completion.button = quoted();
                } else if item.starts_with("Message on first-try")
                    || item.starts_with("First-try message")
                {
                    completion.first_try = quoted();
                } else if item.starts_with("Message after several")
                    || item.starts_with("Retry message")
                {
                    completion.many_tries = quoted();
                }
            }
        }
    }
    if screens
        .iter()
        .filter(|s| s.kind != ScreenKind::Check)
        .count()
        < 4
    {
        report.error(format!("{id}: fewer than 4 teaching screens"));
    }
    if summary.is_empty() {
        report.error(format!("{id}: no summary"));
    }
    if !screens.iter().any(|s| s.coming_from.len() == 5) {
        report.error(format!(
            "{id}: no Coming-from card with all five backgrounds"
        ));
    }
    Some(Lesson {
        uid: id.clone(),
        id,
        title,
        book_section: get("Book section"),
        foundation: get("Foundation lesson").starts_with("Yes"),
        concepts_introduced: md::concept_ids(&get("Concepts introduced")),
        concepts_reused: md::concept_ids(&get("Concepts reused")),
        minutes,
        objective_md,
        screens,
        summary,
        completion,
        read_more: Vec::new(),
    })
}

fn screen(
    lesson: &str,
    h: &[String],
    lines: &[&str],
    earlier: &[Screen],
    report: &mut Report,
) -> Option<Screen> {
    let kind = match h.get(1).map(|s| s.as_str()) {
        Some("concept") => ScreenKind::Concept,
        Some("stepper") => ScreenKind::Stepper,
        Some("error") => ScreenKind::Error,
        Some("compare") => ScreenKind::Compare,
        other => {
            report.error(format!("{lesson} {}: unknown screen type {other:?}", h[0]));
            return None;
        }
    };
    let heading = h.get(2).cloned().unwrap_or_default();
    let text_md = md::block_after(lines, "**Text:**")
        .map(|b| md::text(&b))
        .unwrap_or_default();

    let (code, files) = code_blocks(lines, earlier);
    let coming_from = coming_from(lines);
    let visual_md = md::block_after(lines, "**Visual (static):**").map(|b| md::text(&b));

    let mut steps = Vec::new();
    if kind == ScreenKind::Stepper {
        for row in md::table_with_header(lines, "Step") {
            if row.len() < 4 {
                continue;
            }
            let (file, hl) = highlighted(&row[1]);
            steps.push(Step {
                n: steps.len() as u32 + 1,
                file,
                hl,
                err: Vec::new(),
                caption_md: row[3].clone(),
                visual_md: row[2].clone(),
                ops: Vec::new(),
                notes: Vec::new(),
                links: Vec::new(),
            });
        }
        if steps.is_empty() {
            report.error(format!("{lesson} {}: stepper without steps", h[0]));
        }
        if let Some(v) = md::block_after(lines, "**Visualizer:**") {
            let v = md::text(&v);
            let v = v.split("\n|").next().unwrap_or("").trim().to_string();
            // Keep the prose as the screen's visual description for authors.
            return Some(Screen {
                key: String::new(),
                kind,
                heading,
                text_md,
                code,
                files,
                coming_from,
                panels: Vec::new(),
                steps,
                diagnostic: None,
                annotations: Vec::new(),
                compare: None,
                question: None,
                visual_md: Some(v),
            });
        }
    }

    let mut diagnostic = None;
    let mut annotations = Vec::new();
    if kind == ScreenKind::Error {
        let out = lines
            .iter()
            .position(|l| l.starts_with("**Compiler output") || l.starts_with("**Runtime output"));
        if let Some(i) = out {
            diagnostic = md::fences(&lines[i..])
                .into_iter()
                .next()
                .map(|f| f.text.trim_matches('\n').to_string());
        }
        if diagnostic.is_none() {
            report.error(format!(
                "{lesson} {}: error screen without compiler or runtime output",
                h[0]
            ));
        }
        if let Some(b) = md::block_after(lines, "**Annotations:**") {
            for l in b {
                let t = l.trim();
                let Some((n, rest)) = t.split_once(". ") else {
                    continue;
                };
                if n.parse::<u32>().is_err() {
                    continue;
                }
                let (mark, note) = rest.split_once(" → ").unwrap_or((rest, ""));
                let mark = md::code_spans(mark)
                    .into_iter()
                    .next()
                    .unwrap_or_else(|| mark.to_string());
                annotations.push(Annotation {
                    mark,
                    note_md: note.trim().to_string(),
                    line: None,
                });
            }
        }
    }

    let mut compare = None;
    if kind == ScreenKind::Compare {
        let panel = |label_line: &str| -> Option<ComparePanel> {
            let i = lines.iter().position(|l| l.starts_with(label_line))?;
            // `**Left panel** (label: "…")`, `**Left** (label: "…")` or `· label "…" (side style: …)`.
            let label = lines[i]
                .split("label: \"")
                .nth(1)
                .or_else(|| lines[i].split("label \"").nth(1))?
                .split('"')
                .next()?
                .to_string();
            let rest = &lines[i + 1..];
            let f = md::fences(rest).into_iter().next()?;
            let caption = rest.iter().find_map(|l| {
                l.strip_prefix("One-line explanation:")
                    .or_else(|| l.strip_prefix("Explanation:"))
            })?;
            let caption = caption.trim().to_string();
            Some(ComparePanel {
                label,
                code: CodeBlock {
                    file: None,
                    lang: lang(&f.info),
                    text: f.text,
                },
                caption_md: caption,
            })
        };
        match (panel("**Left"), panel("**Right")) {
            (Some(left), Some(right)) => compare = Some(Compare { left, right }),
            _ => report.error(format!(
                "{lesson} {}: compare screen needs both panels",
                h[0]
            )),
        }
    }

    Some(Screen {
        key: String::new(),
        kind,
        heading,
        text_md,
        code,
        files,
        coming_from,
        panels: Vec::new(),
        steps,
        diagnostic,
        annotations,
        compare,
        question: None,
        visual_md,
    })
}

fn lang(info: &str) -> String {
    if info.is_empty() {
        "text".into()
    } else {
        info.to_string()
    }
}

/// "3", "2–7", "1, 4" or, in a multi-file screen, "src/main.rs:2" / "src/lib.rs:1–3":
/// the file (if named) and the lines.
fn highlighted(cell: &str) -> (Option<String>, Vec<u32>) {
    let cell = cell.trim();
    let (file, lines) = match cell.rsplit_once(':') {
        Some((f, l)) if f.contains('.') || f.contains('/') => (Some(f.trim().to_string()), l),
        _ => (None, cell),
    };
    let mut hl = Vec::new();
    for part in lines.split(',') {
        let part = part.trim();
        match part.split_once(['–', '-']) {
            Some((a, b)) => {
                if let (Ok(a), Ok(b)) = (a.trim().parse::<u32>(), b.trim().parse::<u32>()) {
                    hl.extend(a..=b);
                }
            }
            None => hl.extend(part.parse::<u32>().ok()),
        }
    }
    (file, hl)
}

/// The screen's code: one `**Code**` block, or several (a package's files), each
/// labelled with its path.
fn code_blocks(lines: &[&str], earlier: &[Screen]) -> (Option<CodeBlock>, Vec<CodeBlock>) {
    let labels: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter(|(_, l)| l.starts_with("**Code**"))
        .map(|(i, _)| i)
        .collect();
    if labels.len() < 2 {
        return (code_block(lines, earlier), Vec::new());
    }
    let files: Vec<CodeBlock> = labels
        .iter()
        .filter_map(|&i| code_block(&lines[i..], earlier))
        .collect();
    // A package names a different file in every block; otherwise the first block is the
    // screen's code (later ones are variations of it).
    let mut paths: Vec<&str> = files.iter().filter_map(|f| f.file.as_deref()).collect();
    paths.sort();
    paths.dedup();
    if paths.len() != files.len() {
        return (code_block(lines, earlier), Vec::new());
    }
    (None, files)
}

fn code_block(lines: &[&str], earlier: &[Screen]) -> Option<CodeBlock> {
    let i = lines.iter().position(|l| l.starts_with("**Code**"))?;
    let label = &lines[i];
    let file = md::code_spans(label).into_iter().next();
    if let Some(n) = label.split("same program as screen ").nth(1) {
        let n: usize = n.trim_end_matches('.').trim().parse().ok()?;
        let teaching: Vec<&Screen> = earlier
            .iter()
            .filter(|s| s.kind != ScreenKind::Check)
            .collect();
        return teaching.get(n - 1).and_then(|s| s.code.clone());
    }
    // the first fence after the label, before the next bold label
    let mut j = i + 1;
    while j < lines.len() && !md::is_fence(lines[j]) {
        if md::is_bold_label(lines[j]) {
            return None;
        }
        j += 1;
    }
    let f = md::fences(&lines[j..]).into_iter().next()?;
    Some(CodeBlock {
        file,
        lang: lang(&f.info),
        text: f.text,
    })
}

fn coming_from(lines: &[&str]) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    let Some(i) = lines.iter().position(|l| l.starts_with("**Coming from**")) else {
        return out;
    };
    for row in md::table_with_header(&lines[i..], "Background") {
        if row.len() < 2 {
            continue;
        }
        let key = match row[0].as_str() {
            "Python" => "python",
            "JavaScript/TypeScript" => "js",
            "Java/C#" => "java",
            "C/C++" => "c",
            "Go" => "go",
            _ => continue,
        };
        out.insert(key.to_string(), row[1].clone());
    }
    out
}
