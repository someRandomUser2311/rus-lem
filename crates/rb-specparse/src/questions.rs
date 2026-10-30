//! Questions (quick checks, follow-up sets, practice sets).

use crate::Report;
use crate::md::{self, Section};
use rb_content::*;

/// Parses one question section (heading `ID · Type · Difficulty d`).
pub fn question(sec: &Section, report: &mut Report) -> Option<Question> {
    let parts = md::dot_parts(&sec.heading);
    if parts.len() < 3 {
        report.error(format!(
            "question heading `{}` is not `ID · Type · Difficulty d`",
            sec.heading
        ));
        return None;
    }
    let id = parts[0].clone();
    let kind = match parts[1].as_str() {
        "Single choice" => QuestionKind::Single,
        "Multiple choice" => QuestionKind::Multi,
        "True / false" => QuestionKind::TrueFalse,
        "Does this compile?" => QuestionKind::Compiles,
        "Predict the output" => QuestionKind::PredictOutput,
        "Fill in the blank" => QuestionKind::FillBlank,
        "Arrange the lines" => QuestionKind::Arrange,
        "Spot the bug" => QuestionKind::SpotBug,
        "Match pairs" => QuestionKind::MatchPairs,
        other => {
            report.error(format!("{id}: unknown question type `{other}`"));
            return None;
        }
    };
    let difficulty = parts[2]
        .trim_start_matches("Difficulty")
        .trim()
        .parse()
        .unwrap_or(1);
    let lines = &sec.lines;
    let concepts = lines
        .iter()
        .find(|l| l.starts_with("Concepts:"))
        .map(|l| md::concept_ids(l))
        .unwrap_or_default();

    // Prompt: everything under **Question** up to the next bold label.
    let prompt_lines = md::block_after(lines, "**Question**").unwrap_or_default();
    let (prompt_md, code) = split_prompt(&prompt_lines);

    let options = md::block_after(lines, "**Options**")
        .map(|b| lettered(&b))
        .unwrap_or_default();
    let right = md::block_after(lines, "**Right**")
        .map(|b| lettered(&b))
        .unwrap_or_default();
    let left = md::block_after(lines, "**Left**")
        .map(|b| numbered(&b))
        .unwrap_or_default();
    let options = if kind == QuestionKind::MatchPairs {
        right
    } else {
        options
    };

    let answer_line = lines
        .iter()
        .find(|l| l.starts_with("**Answer:**"))
        .map(|l| l["**Answer:**".len()..].trim().to_string());
    let accepted_line = lines
        .iter()
        .find(|l| l.starts_with("**Accepted answers:**"))
        .map(|l| l["**Accepted answers:**".len()..].trim().to_string());

    let answer = match kind {
        QuestionKind::Single
        | QuestionKind::Multi
        | QuestionKind::TrueFalse
        | QuestionKind::Compiles => {
            let keys: Vec<String> = answer_line
                .clone()
                .unwrap_or_default()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            Answer::Keys { keys }
        }
        QuestionKind::PredictOutput => {
            let block = md::block_after(lines, "**Answer:**").unwrap_or_default();
            let text = md::fences(&block)
                .into_iter()
                .next()
                .map(|f| f.text)
                .unwrap_or_else(|| md::unbacktick(&answer_line.clone().unwrap_or_default()));
            Answer::Text {
                accepted: vec![text.clone()],
                text,
            }
        }
        QuestionKind::FillBlank => Answer::Blanks {
            blanks: blanks(
                answer_line.as_deref().unwrap_or(""),
                accepted_line.as_deref(),
            ),
        },
        QuestionKind::Arrange => {
            let orders = answer_line
                .clone()
                .unwrap_or_default()
                .split(" or ")
                .map(|o| {
                    o.split(',')
                        .map(|k| k.trim().to_string())
                        .filter(|k| !k.is_empty())
                        .collect()
                })
                .collect();
            Answer::Orders { orders }
        }
        QuestionKind::SpotBug => {
            let a = answer_line.clone().unwrap_or_default();
            let a = a.trim_start_matches("lines").trim_start_matches("line");
            Answer::Lines {
                lines: a.split(',').filter_map(|n| n.trim().parse().ok()).collect(),
            }
        }
        QuestionKind::MatchPairs => {
            let pairs = answer_line
                .clone()
                .unwrap_or_default()
                .split(',')
                .filter_map(|p| {
                    p.trim()
                        .split_once('-')
                        .map(|(l, r)| (l.trim().to_string(), r.trim().to_string()))
                })
                .collect();
            Answer::Pairs { pairs }
        }
    };
    if answer_line.is_none() {
        report.error(format!("{id}: no **Answer:** line"));
    }

    let verify = match kind {
        QuestionKind::Compiles => Some(compile_facts(lines, &answer)),
        QuestionKind::PredictOutput => match &answer {
            Answer::Text { text, .. } => Some(Verify {
                compiles: true,
                error_code: None,
                error_line: None,
                error_message: None,
                stdout: Some(text.clone()),
            }),
            _ => None,
        },
        _ => None,
    };

    let expl = md::block_after(lines, "**Explanations**").unwrap_or_default();
    let mut explanations = Vec::new();
    let mut general = Vec::new();
    let mut wrong_answers = Vec::new();
    let mut fix_md = None;
    let mut in_wrong = false;
    for l in &expl {
        let t = l.trim_end();
        if t.trim().is_empty() {
            continue;
        }
        let indented = t.starts_with("  ");
        let item = t.trim_start().strip_prefix("- ").map(str::trim);
        let Some(item) = item else {
            // continuation of the previous bullet
            if let Some(last) = wrong_answers.last_mut().filter(|_| in_wrong) {
                let w: &mut WrongAnswer = last;
                w.why_md.push(' ');
                w.why_md.push_str(t.trim());
            } else if let Some(g) = general.last_mut() {
                let g: &mut String = g;
                g.push(' ');
                g.push_str(t.trim());
            }
            continue;
        };
        if indented && in_wrong {
            wrong_answers.push(split_wrong(item));
            continue;
        }
        in_wrong = false;
        if item.starts_with("Common wrong answers") {
            in_wrong = true;
            continue;
        }
        if let Some(fix) = item.strip_prefix("**Fix:**") {
            fix_md = Some(fix.trim().to_string());
            continue;
        }
        if let Some((key, rest)) = option_key(item) {
            let rest = rest.trim();
            let correct = rest.starts_with("Correct");
            explanations.push(Explanation {
                key,
                correct,
                text_md: rest.to_string(),
            });
            continue;
        }
        if let Some((pair, rest)) = pair_key(item) {
            explanations.push(Explanation {
                key: pair,
                correct: true,
                text_md: rest.trim().to_string(),
            });
            continue;
        }
        general.push(item.to_string());
    }

    let q = Question {
        uid: id.clone(),
        id,
        kind,
        difficulty,
        concepts,
        prompt_md,
        code,
        options,
        left,
        answer,
        explanations,
        general_md: if general.is_empty() {
            None
        } else {
            Some(general.join("\n\n"))
        },
        wrong_answers,
        fix_md,
        verify,
    };
    check(&q, report);
    Some(q)
}

fn split_prompt(lines: &[&str]) -> (String, Option<CodeBlock>) {
    let fences = md::fences(lines);
    if fences.len() == 1 {
        let mut text = Vec::new();
        let mut in_fence = false;
        for &l in lines {
            if md::is_fence(l) {
                in_fence = !in_fence;
                continue;
            }
            if !in_fence {
                text.push(l);
            }
        }
        let f = &fences[0];
        (
            md::text(&text),
            Some(CodeBlock {
                file: None,
                lang: if f.info.is_empty() {
                    "text".into()
                } else {
                    f.info.clone()
                },
                text: f.text.clone(),
            }),
        )
    } else {
        (md::text(lines), None)
    }
}

/// `- A. text` items.
pub fn lettered(lines: &[&str]) -> Vec<Choice> {
    let mut out: Vec<Choice> = Vec::new();
    for l in lines {
        let t = l.trim();
        // `- A. text`, or `A. text` without the bullet.
        if let Some((key, rest)) = option_key(t.strip_prefix("- ").unwrap_or(t)) {
            out.push(Choice {
                key,
                text_md: rest.trim().to_string(),
            });
            continue;
        }
        if !t.is_empty()
            && let Some(last) = out.last_mut()
        {
            last.text_md.push(' ');
            last.text_md.push_str(t);
        }
    }
    out
}

/// `1. text` items.
fn numbered(lines: &[&str]) -> Vec<Choice> {
    lines
        .iter()
        .filter_map(|l| {
            let t = l.trim();
            let (n, rest) = t.split_once(". ")?;
            n.parse::<u32>().ok()?;
            Some(Choice {
                key: n.to_string(),
                text_md: rest.trim().to_string(),
            })
        })
        .collect()
}

/// "A. rest" → ("A", "rest").
fn option_key(item: &str) -> Option<(String, String)> {
    let (k, rest) = item.split_once(". ")?;
    (k.len() == 1 && k.chars().all(|c| c.is_ascii_uppercase()))
        .then(|| (k.to_string(), rest.to_string()))
}

/// "1-C: rest" → ("1-C", "rest").
fn pair_key(item: &str) -> Option<(String, String)> {
    let (k, rest) = item.split_once(':')?;
    let (l, r) = k.split_once('-')?;
    (l.trim().parse::<u32>().is_ok() && r.trim().len() == 1)
        .then(|| (k.trim().to_string(), rest.to_string()))
}

fn split_wrong(item: &str) -> WrongAnswer {
    // "`answer`: why" or "answer: why" — split at the first ": " outside code spans.
    let chars: Vec<char> = item.chars().collect();
    let mut ticks = false;
    for i in 0..chars.len() {
        if chars[i] == '`' {
            ticks = !ticks;
        }
        if !ticks && chars[i] == ':' && chars.get(i + 1) == Some(&' ') {
            let a: String = chars[..i].iter().collect();
            let b: String = chars[i + 1..].iter().collect();
            return WrongAnswer {
                answer_md: a.trim().to_string(),
                why_md: b.trim().to_string(),
            };
        }
    }
    WrongAnswer {
        answer_md: item.to_string(),
        why_md: String::new(),
    }
}

/// "(1) mut  (2) &mut" plus "(1) mut / (2) &mut, & mut".
fn blanks(answer: &str, accepted: Option<&str>) -> Vec<Vec<String>> {
    let split = |s: &str| -> Vec<(usize, String)> {
        let mut out = Vec::new();
        let mut rest = s;
        while let Some(start) = rest.find('(') {
            let after = &rest[start + 1..];
            let Some(close) = after.find(')') else { break };
            let Ok(n) = after[..close].trim().parse::<usize>() else {
                rest = &after[close + 1..];
                continue;
            };
            let body = &after[close + 1..];
            let end = find_next_blank(body);
            out.push((
                n,
                body[..end].trim().trim_end_matches('/').trim().to_string(),
            ));
            rest = &body[end..];
        }
        out
    };
    let mut out: Vec<Vec<String>> = Vec::new();
    for (n, v) in split(answer) {
        if out.len() < n {
            out.resize(n, Vec::new());
        }
        out[n - 1].push(md::unbacktick(&v));
    }
    if let Some(acc) = accepted {
        for (n, v) in split(acc) {
            if out.len() < n {
                out.resize(n, Vec::new());
            }
            for alt in v.split(',') {
                let a = md::unbacktick(alt);
                if !a.is_empty() && !out[n - 1].contains(&a) {
                    out[n - 1].push(a);
                }
            }
        }
    }
    out
}

fn find_next_blank(s: &str) -> usize {
    let b = s.as_bytes();
    for i in 0..b.len() {
        if b[i] == b'(' {
            let rest = &s[i + 1..];
            if let Some(c) = rest.find(')')
                && rest[..c].trim().parse::<usize>().is_ok()
            {
                return i;
            }
        }
    }
    s.len()
}

fn compile_facts(lines: &[&str], answer: &Answer) -> Verify {
    let compiles =
        matches!(answer, Answer::Keys { keys } if keys.first().map(|k| k == "A").unwrap_or(false));
    let mut v = Verify {
        compiles,
        error_code: None,
        error_line: None,
        error_message: None,
        stdout: None,
    };
    if let Some(l) = lines.iter().find(|l| l.starts_with("**Compiler error:**")) {
        let rest = l["**Compiler error:**".len()..].trim();
        let (code, line) = rest.split_once(" at line ").unwrap_or((rest, ""));
        v.error_code = Some(md::unbacktick(code)).filter(|c| !c.is_empty() && c != "none");
        v.error_line = line.trim().trim_end_matches('.').parse().ok();
    }
    if let Some(l) = lines.iter().find(|l| l.starts_with("**Error message:**")) {
        let same_line = md::unbacktick(l["**Error message:**".len()..].trim());
        // Either on the same line or the first line of the code block after it.
        v.error_message = if same_line.is_empty() {
            md::block_after(lines, "**Error message:**")
                .and_then(|b| md::fences(&b).first().map(|f| f.text.clone()))
                .and_then(|t| t.lines().next().map(str::to_string))
        } else {
            Some(same_line)
        };
    }
    if let Some(block) = md::block_after(lines, "**Output:**") {
        let fences = md::fences(&block);
        v.stdout = match fences.first() {
            Some(f) => Some(f.text.clone()),
            None => {
                let t = md::text(&block);
                let t = md::unbacktick(&t);
                Some(
                    md::single_quoted(&t)
                        .map(|s| s.trim_end_matches('\n').to_string())
                        .unwrap_or(t),
                )
            }
        };
    }
    v
}

fn check(q: &Question, report: &mut Report) {
    let id = &q.id;
    if q.prompt_md.is_empty() {
        report.error(format!("{id}: empty question text"));
    }
    if q.concepts.is_empty() {
        report.error(format!("{id}: no concepts"));
    }
    match q.kind {
        QuestionKind::Single
        | QuestionKind::Multi
        | QuestionKind::TrueFalse
        | QuestionKind::Compiles => {
            if q.options.len() < 2 {
                report.error(format!("{id}: fewer than 2 options"));
            }
            for o in &q.options {
                if !q.explanations.iter().any(|e| e.key == o.key) {
                    report.error(format!("{id}: option {} has no explanation", o.key));
                }
            }
            if let Answer::Keys { keys } = &q.answer {
                for k in keys {
                    if !q.options.iter().any(|o| &o.key == k) {
                        report.error(format!("{id}: answer {k} is not an option"));
                    }
                }
                if q.kind == QuestionKind::Multi && keys.len() < 2 {
                    report.error(format!("{id}: multiple choice needs 2+ correct options"));
                }
            }
        }
        QuestionKind::PredictOutput | QuestionKind::FillBlank => {
            if q.wrong_answers.len() < 2 {
                report.warn(format!("{id}: fewer than 2 common wrong answers"));
            }
        }
        QuestionKind::Arrange => {
            if let Answer::Orders { orders } = &q.answer {
                for o in orders {
                    if o.len() != q.options.len() {
                        report.error(format!("{id}: order {o:?} doesn't use every line"));
                    }
                }
            }
        }
        QuestionKind::SpotBug => {
            if q.code.is_none() {
                report.error(format!("{id}: spot the bug needs code"));
            }
        }
        QuestionKind::MatchPairs => {
            if q.left.len() != q.options.len() {
                report.error(format!("{id}: left and right columns differ in length"));
            }
        }
    }
}

/// Parses a questions.md file into its sets.
pub fn question_sets(text: &str, chapter: u32, report: &mut Report) -> Vec<QuestionSet> {
    let lines: Vec<&str> = text.lines().collect();
    let mut out = Vec::new();
    for sec in md::sections(&lines, 2) {
        let parts = md::dot_parts(&sec.heading);
        let id = parts[0].clone();
        if !id.starts_with('Q') {
            continue;
        }
        let practice = id.ends_with(".P");
        let title = parts.get(1).cloned().unwrap_or_default();
        let lesson = (!practice).then(|| format!("L{}", &id[1..]));
        let mut qs = Vec::new();
        for qsec in md::sections(&sec.lines, 4) {
            if let Some(q) = question(&qsec, report) {
                qs.push(q);
            }
        }
        let n = qs.len();
        if practice && !(10..=15).contains(&n) {
            report.warn(format!("{id}: practice set has {n} questions (10–15)"));
        }
        if !practice && !(8..=12).contains(&n) {
            report.warn(format!("{id}: follow-up set has {n} questions (8–12)"));
        }
        let _ = chapter;
        out.push(QuestionSet {
            uid: id.clone(),
            id,
            title,
            practice,
            lesson,
            xp: if practice { 15 } else { 10 },
            questions: qs,
        });
    }
    out
}
