//! Static checks (content rules) and dynamic checks (compile and run everything).

use crate::source::Source;
use anyhow::Result;
use rb_content::*;
use std::collections::HashSet;

pub fn is_todo(t: &TestCase) -> bool {
    matches!(t.run, Run::Todo)
        || t.asserts
            .iter()
            .any(|a| matches!(a, Assertion::Todo { .. }))
}

/// Rule violations that block bundling.
pub fn static_errors(course: &Course, ch: &Chapter) -> Vec<String> {
    let mut e = Vec::new();
    let concept = |id: &str| course.concepts.iter().find(|c| c.id == id);
    let check_concepts = |owner: &str, ids: &[String], e: &mut Vec<String>| {
        for id in ids {
            let id = id.split_whitespace().next().unwrap_or(id);
            match concept(id) {
                None => e.push(format!("{owner}: unknown concept `{id}`")),
                Some(c) if c.chapter > ch.num => e.push(format!(
                    "{owner}: `{id}` is from chapter {} (forward reference)",
                    c.chapter
                )),
                _ => {}
            }
        }
    };

    let mut ids = HashSet::new();
    for l in &ch.lessons {
        check_concepts(&l.id, &l.concepts_introduced, &mut e);
        check_concepts(&l.id, &l.concepts_reused, &mut e);
        for s in &l.screens {
            let panels: Vec<&str> = s
                .panels
                .iter()
                .map(|p| match p {
                    Panel::Terminal { id, .. } | Panel::Scene { id, .. } => id.as_str(),
                })
                .collect();
            if s.kind == ScreenKind::Stepper && s.panels.is_empty() {
                e.push(format!(
                    "{} {}: stepper has no panels (author them in the overlay)",
                    l.id, s.key
                ));
            }
            for st in &s.steps {
                for op in &st.ops {
                    if !panels.contains(&op.panel.as_str()) {
                        e.push(format!(
                            "{} {} step {}: op on unknown panel `{}`",
                            l.id, s.key, st.n, op.panel
                        ));
                    }
                }
                let lines = s
                    .code
                    .as_ref()
                    .map(|c| c.text.lines().count() as u32)
                    .unwrap_or(0);
                for h in st.hl.iter().chain(&st.err) {
                    if *h == 0 || *h > lines {
                        e.push(format!(
                            "{} {} step {}: line {h} is outside the code",
                            l.id, s.key, st.n
                        ));
                    }
                }
            }
            if let Some(q) = &s.question {
                check_concepts(&q.id, &q.concepts, &mut e);
                if !ids.insert(q.id.clone()) {
                    e.push(format!("{}: duplicate id", q.id));
                }
            }
        }
    }
    for set in &ch.question_sets {
        for q in &set.questions {
            check_concepts(&q.id, &q.concepts, &mut e);
            if !ids.insert(q.id.clone()) {
                e.push(format!("{}: duplicate id", q.id));
            }
        }
    }
    for p in &ch.problems {
        check_concepts(&p.id, &p.concepts, &mut e);
        check_concepts(&p.id, &p.reuses, &mut e);
        if p.solution.is_empty() {
            e.push(format!("{}: no reference solution", p.id));
        }
        let visible = p.tests.iter().filter(|t| t.visible).count();
        let max_visible = if matches!(p.level, Level::Project | Level::Milestone) {
            6
        } else {
            4
        };
        if !(2..=max_visible).contains(&visible) {
            e.push(format!(
                "{}: {visible} visible tests (2–{max_visible})",
                p.id
            ));
        }
        let mut keys = HashSet::new();
        for t in &p.tests {
            if !keys.insert(&t.key) {
                e.push(format!("{} {}: duplicate test key", p.id, t.key));
            }
            if is_todo(t) {
                e.push(format!(
                    "{} #{}: not converted yet: {}",
                    p.id, t.n, t.expected_md
                ));
            }
            if t.asserts.is_empty() {
                e.push(format!("{} #{}: no assertions", p.id, t.n));
            }
            if t.visible && t.category == "constraint" {
                e.push(format!("{} #{}: constraint rows must be hidden", p.id, t.n));
            }
        }
        for w in &p.wrong {
            if w.fails.is_empty() {
                e.push(format!(
                    "{} wrong `{}`: names no test it fails",
                    p.id, w.name
                ));
            }
            for f in w.fails.iter().chain(&w.may_fail) {
                if !p.tests.iter().any(|t| &t.key == f) {
                    e.push(format!("{} wrong `{}`: unknown test {f}", p.id, w.name));
                }
            }
        }
    }
    let known: HashSet<&str> = ch
        .lessons
        .iter()
        .map(|l| l.id.as_str())
        .chain(ch.question_sets.iter().map(|s| s.id.as_str()))
        .chain(ch.problems.iter().map(|p| p.id.as_str()))
        .chain(["review"])
        .collect();
    for item in &ch.sequence {
        if !known.contains(item.id.as_str()) {
            e.push(format!("sequence: unknown item {}", item.id));
        }
    }
    for p in &ch.problems {
        if !ch.sequence.iter().any(|i| i.id == p.id) {
            e.push(format!("{}: missing from the course sequence", p.id));
        }
    }
    e
}

pub fn run(src: &Source, chapters: &[u32], dynamic: bool, only: &[String]) -> Result<bool> {
    let course = src.course()?;
    let mut ok = true;
    for &n in chapters {
        let mut ch = src.chapter(n)?;
        let mut errors = static_errors(&course, &ch);
        if !only.is_empty() {
            let mine = |s: &str| {
                only.iter()
                    .any(|id| s.starts_with(&format!("{id} ")) || s.starts_with(&format!("{id}:")))
            };
            errors.retain(|e| mine(e));
            ch.problems.retain(|p| only.contains(&p.id));
            ch.question_sets.clear();
            ch.lessons.clear();
        }
        println!("== chapter {n}: {} static errors", errors.len());
        for e in &errors {
            println!("  ERROR {e}");
        }
        ok &= errors.is_empty();
        if dynamic {
            ok &= crate::dynamic::run(&ch, src.vendor_dir().as_deref())?;
        }
    }
    Ok(ok)
}
