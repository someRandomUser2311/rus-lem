//! What the UI receives. Problems never carry their solution, wrong variants or
//! hidden-test details unless the learner has unlocked them.

use rb_content::{Level, Problem, SourceFile};
use rb_grader::{GradeResult, TestStatus};
use rb_progress::ProblemState;
use serde::Serialize;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct VisibleTest {
    pub key: String,
    pub n: u32,
    pub category: String,
    pub input: String,
    pub expected: String,
    pub checks: String,
    pub part: Option<u32>,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct ProblemView {
    pub chapter: String,
    pub id: String,
    pub title: String,
    pub level: Level,
    pub xp: u32,
    pub kind: String,
    pub concepts: Vec<String>,
    pub reuses: Vec<String>,
    pub starter_stage: String,
    pub time_limit_ms: u64,
    pub cargo: bool,
    pub description_md: String,
    pub examples_md: String,
    pub constraints_md: String,
    pub parts: Vec<String>,
    /// Parts are graded one at a time on their own submissions.
    pub checkpoints: bool,
    /// Checkpoint problems: hidden tests per part (index = part − 1).
    pub hidden_per_part: Vec<usize>,
    pub hint_count: usize,
    /// Text of the hints the learner has opened (index = hint number − 1).
    pub hints: Vec<Option<String>>,
    pub starter: Vec<SourceFile>,
    /// The learner's saved files, if any.
    pub draft: Option<Vec<SourceFile>>,
    pub visible_tests: Vec<VisibleTest>,
    pub hidden_count: usize,
    pub state: ProblemState,
    pub solution: Option<Vec<SourceFile>>,
    pub solution_approach_md: Option<String>,
    pub idiomatic_md: Option<String>,
}

pub fn problem_view(
    chapter: &str,
    p: &Problem,
    state: ProblemState,
    draft: Option<Vec<SourceFile>>,
) -> ProblemView {
    let hints = (1..=p.hints.len() as u32)
        .map(|n| {
            state
                .hints_opened
                .contains(&n)
                .then(|| p.hints[n as usize - 1].clone())
        })
        .collect();
    let visible_tests = p
        .tests
        .iter()
        .filter(|t| t.visible)
        .map(|t| VisibleTest {
            key: t.key.clone(),
            n: t.n,
            category: t.category.clone(),
            input: t.input_md.clone(),
            expected: t.expected_md.clone(),
            checks: t.checks_md.clone(),
            part: t.part,
        })
        .collect();
    let open = state.solution_available;
    ProblemView {
        chapter: chapter.to_string(),
        id: p.id.clone(),
        title: p.title.clone(),
        level: p.level,
        xp: p.xp,
        kind: p.kind.clone(),
        concepts: p.concepts.clone(),
        reuses: p.reuses.clone(),
        starter_stage: p.starter_stage.clone(),
        time_limit_ms: p.time_limit_ms,
        cargo: p.mode == rb_content::BuildMode::Cargo,
        description_md: p.description_md.clone(),
        examples_md: p.examples_md.clone(),
        constraints_md: p.constraints_md.clone(),
        parts: p.parts.clone(),
        checkpoints: p.checkpoints,
        hidden_per_part: (1..=p.parts.len() as u32)
            .map(|n| {
                p.tests
                    .iter()
                    .filter(|t| !t.visible && t.part == Some(n))
                    .count()
            })
            .collect(),
        hint_count: p.hints.len(),
        hints,
        starter: p.starter.clone(),
        draft,
        hidden_count: p.tests.iter().filter(|t| !t.visible).count(),
        visible_tests,
        solution: open.then(|| p.solution.clone()),
        solution_approach_md: open.then(|| p.solution_approach_md.clone()),
        idiomatic_md: if open { p.idiomatic_md.clone() } else { None },
        state,
    }
}

/// Hides hidden tests' details unless revealed (failed twice). Rule tests always
/// say which rule they check.
pub fn redact(mut r: GradeResult, p: &Problem, revealed: &[String]) -> GradeResult {
    for t in &mut r.tests {
        if t.visible || revealed.contains(&t.key) {
            if let Some(tc) = p.tests.iter().find(|x| x.key == t.key)
                && t.expected.is_none()
                && t.status != TestStatus::Passed
            {
                t.expected = Some(tc.expected_md.clone());
            }
            continue;
        }
        t.input = String::new();
        if t.category == "constraint" {
            continue;
        }
        t.expected = None;
        t.actual = None;
        if t.status != TestStatus::Passed && t.status != TestStatus::NotRun {
            let what = match t.status {
                TestStatus::TimeLimit => "ran out of time",
                TestStatus::RuntimeError => "crashed",
                TestStatus::OutputLimit => "printed too much",
                _ => "failed",
            };
            t.message = Some(format!(
                "Hidden test {} {what}. Its input is shown after it fails twice.",
                t.n
            ));
        }
    }
    r
}
