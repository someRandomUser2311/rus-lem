//! Progress rules as pure functions over a [`Snapshot`] of the learner's progress.
//! Source: rust-basics-course-plan/00-foundations/progress-and-unlocking.md.

use rb_content::{Chapter, ChapterRef, Course, Level, PathKind, Problem};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use ts_rs::TS;

pub const FOLLOW_UP_TEST_OUT: f64 = 0.8;
pub const HIDDEN_REVEAL_AFTER: u32 = 2;
pub const SOLUTION_AFTER_FAILS: u32 = 5;
pub const HINT_WAIT_SECS: i64 = 30;
pub const STREAK_DAY_XP: i64 = 20;
pub const MASTERY_PER_PROBLEM: f64 = 25.0;
pub const MASTERY_PER_QUESTION: f64 = 5.0;
pub const MASTERY_DECAY_PER_WEEK: f64 = 0.10;

/// Everything the rules need to know about one learner in one course.
#[derive(Debug, Clone, Default, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Snapshot {
    pub onboarded: bool,
    pub lessons_done: BTreeSet<String>,
    /// Question set id → best score (0–1) and whether the first attempt scored ≥ 80%.
    pub sets_done: BTreeMap<String, SetScore>,
    pub accepted: BTreeSet<String>,
    /// Chapters unlocked early by passing their test-out.
    pub tested_out: BTreeSet<u32>,
    /// Experienced programmers may test out: practice sets and Hard problems are open early.
    #[serde(default)]
    pub experienced: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct SetScore {
    pub best: f64,
    pub first: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(tag = "state", rename_all = "snake_case")]
#[ts(export)]
pub enum ItemState {
    Locked { reason: String },
    Open,
    Done,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct PathNode {
    pub kind: PathKind,
    pub id: String,
    pub title: String,
    /// Problem level, for problems.
    pub level: Option<Level>,
    pub xp: u32,
    pub state: ItemState,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ChapterState {
    pub num: u32,
    pub id: String,
    pub title: String,
    pub available: bool,
    pub state: ItemState,
    pub nodes: Vec<PathNode>,
    /// Problems needed to unlock the next chapter, and how many are Accepted.
    pub needed: u32,
    pub accepted: u32,
    pub needs_hard: bool,
    pub has_hard: bool,
    /// The chapter's own rule is met (the next chapter opens).
    pub passed: bool,
}

/// Problems that count for the chapter unlock rule (not projects or milestones).
pub fn counted(ch: &Chapter) -> impl Iterator<Item = &Problem> {
    ch.problems
        .iter()
        .filter(|p| !matches!(p.level, Level::Project | Level::Milestone))
}

pub fn lessons_complete(ch: &Chapter, s: &Snapshot) -> bool {
    ch.lessons.iter().all(|l| s.lessons_done.contains(&l.id))
}

/// Is the chapter's own rule met: lessons done and 60% (rounded up) of problems, with a Hard if it has any?
pub fn chapter_passed(ch: &Chapter, s: &Snapshot) -> (bool, u32, u32) {
    let total = counted(ch).count() as f64;
    let needed = (total * ch.unlock.problem_share).ceil() as u32;
    let accepted = counted(ch).filter(|p| s.accepted.contains(&p.id)).count() as u32;
    let hard_ok = !ch.unlock.require_hard
        || counted(ch).any(|p| p.level == Level::Hard && s.accepted.contains(&p.id));
    (
        lessons_complete(ch, s) && accepted >= needed && hard_ok,
        needed,
        accepted,
    )
}

/// The full path with every item's state.
pub fn path(course: &Course, chapters: &[Chapter], s: &Snapshot) -> Vec<ChapterState> {
    let mut out = Vec::new();
    let mut prev_passed = s.onboarded;
    let mut prev_title = String::new();
    for cref in &course.chapters {
        let ch = chapters.iter().find(|c| c.num == cref.num);
        let open = prev_passed || s.tested_out.contains(&cref.num.saturating_sub(1));
        let state = if !cref.available {
            ItemState::Locked {
                reason: "Coming soon".into(),
            }
        } else if !open {
            if cref.num == 1 {
                ItemState::Locked {
                    reason: "Finish onboarding first".into(),
                }
            } else {
                ItemState::Locked {
                    reason: format!("Finish Chapter {}: {}", cref.num - 1, prev_title),
                }
            }
        } else {
            ItemState::Open
        };
        let Some(ch) = ch else {
            out.push(empty(cref, state));
            prev_passed = false;
            prev_title = cref.title.clone();
            continue;
        };
        let (passed, needed, accepted) = chapter_passed(ch, s);
        let nodes = if matches!(state, ItemState::Open) {
            nodes(ch, s)
        } else {
            locked_nodes(ch, &state)
        };
        let has_hard = counted(ch).any(|p| p.level == Level::Hard);
        let state = if matches!(state, ItemState::Open) && passed && done_everything(&nodes) {
            ItemState::Done
        } else {
            state
        };
        out.push(ChapterState {
            num: ch.num,
            id: ch.id.clone(),
            title: ch.title.clone(),
            available: true,
            state,
            nodes,
            needed,
            accepted,
            needs_hard: ch.unlock.require_hard,
            has_hard,
            passed,
        });
        prev_passed = passed;
        prev_title = ch.title.clone();
    }
    out
}

fn done_everything(nodes: &[PathNode]) -> bool {
    nodes.iter().all(|n| n.state == ItemState::Done)
}

fn empty(c: &ChapterRef, state: ItemState) -> ChapterState {
    ChapterState {
        num: c.num,
        id: c.id.clone(),
        title: c.title.clone(),
        available: c.available,
        state,
        nodes: Vec::new(),
        needed: 0,
        accepted: 0,
        needs_hard: false,
        has_hard: false,
        passed: false,
    }
}

fn title_of(ch: &Chapter, kind: PathKind, id: &str) -> (String, Option<Level>, u32) {
    match kind {
        PathKind::Lesson => ch
            .lesson(id)
            .map(|l| (l.title.clone(), None, l.completion.xp))
            .unwrap_or_default(),
        PathKind::Questions | PathKind::Practice => ch
            .question_set(id)
            .map(|q| {
                (
                    if q.practice {
                        format!("Practice: Chapter {}", ch.num)
                    } else {
                        format!("Questions: {}", q.title)
                    },
                    None,
                    q.xp,
                )
            })
            .unwrap_or_default(),
        PathKind::Problem | PathKind::Project | PathKind::Milestone => ch
            .problem(id)
            .map(|p| (p.title.clone(), Some(p.level), p.xp))
            .unwrap_or_default(),
        PathKind::Review => ("Review".into(), None, 0),
    }
}

fn locked_nodes(ch: &Chapter, state: &ItemState) -> Vec<PathNode> {
    ch.sequence
        .iter()
        .map(|i| {
            let (title, level, xp) = title_of(ch, i.kind, &i.id);
            PathNode {
                kind: i.kind,
                id: i.id.clone(),
                title,
                level,
                xp,
                state: state.clone(),
            }
        })
        .collect()
}

fn nodes(ch: &Chapter, s: &Snapshot) -> Vec<PathNode> {
    let all_lessons = lessons_complete(ch, s);
    let medium_solved = counted(ch).any(|p| p.level >= Level::Medium && s.accepted.contains(&p.id));
    let project_open = all_lessons && medium_solved;
    ch.sequence
        .iter()
        .map(|i| {
            let (title, level, xp) = title_of(ch, i.kind, &i.id);
            let state = match i.kind {
                PathKind::Lesson => {
                    let idx = ch.lessons.iter().position(|l| l.id == i.id).unwrap_or(0);
                    if s.lessons_done.contains(&i.id) {
                        ItemState::Done
                    } else if idx == 0 || s.lessons_done.contains(&ch.lessons[idx - 1].id) {
                        ItemState::Open
                    } else {
                        ItemState::Locked {
                            reason: format!("Finish {} first", ch.lessons[idx - 1].title),
                        }
                    }
                }
                PathKind::Questions => {
                    let set = ch.question_set(&i.id);
                    let lesson = set.and_then(|q| q.lesson.clone()).unwrap_or_default();
                    let foundation = ch.lesson(&lesson).is_some_and(|l| l.foundation);
                    if s.sets_done.contains_key(&i.id) {
                        ItemState::Done
                    } else if s.lessons_done.contains(&lesson) || foundation {
                        // A foundation lesson's questions double as its test-out.
                        ItemState::Open
                    } else {
                        ItemState::Locked {
                            reason: format!(
                                "Finish {} first",
                                ch.lesson(&lesson)
                                    .map(|l| l.title.as_str())
                                    .unwrap_or("the lesson")
                            ),
                        }
                    }
                }
                PathKind::Practice => {
                    if s.sets_done.contains_key(&i.id) {
                        ItemState::Done
                    } else if all_lessons || s.experienced {
                        ItemState::Open
                    } else {
                        ItemState::Locked {
                            reason: "Finish every lesson in this chapter first".into(),
                        }
                    }
                }
                PathKind::Review => {
                    if all_lessons {
                        ItemState::Open
                    } else {
                        ItemState::Locked {
                            reason: "Finish every lesson in this chapter first".into(),
                        }
                    }
                }
                PathKind::Problem => {
                    let p = ch.problem(&i.id);
                    let after = p.map(|p| p.comes_after.clone()).unwrap_or_default();
                    let test_out = s.experienced && p.is_some_and(|p| p.level == Level::Hard);
                    if s.accepted.contains(&i.id) {
                        ItemState::Done
                    } else if s.lessons_done.contains(&after)
                        || ch.lesson(&after).is_none()
                        || test_out
                    {
                        ItemState::Open
                    } else {
                        ItemState::Locked {
                            reason: format!(
                                "Finish {} first",
                                ch.lesson(&after)
                                    .map(|l| l.title.as_str())
                                    .unwrap_or(&after)
                            ),
                        }
                    }
                }
                PathKind::Project => {
                    if s.accepted.contains(&i.id) {
                        ItemState::Done
                    } else if project_open {
                        ItemState::Open
                    } else {
                        ItemState::Locked {
                            reason: "Finish every lesson and solve a Medium problem first".into(),
                        }
                    }
                }
                PathKind::Milestone => {
                    let prev = previous_milestone(&i.id);
                    let prev_ok = prev.as_ref().is_none_or(|p| s.accepted.contains(p));
                    let open = project_open;
                    if s.accepted.contains(&i.id) {
                        ItemState::Done
                    } else if open && prev_ok {
                        ItemState::Open
                    } else if !prev_ok {
                        ItemState::Locked {
                            reason: format!("Finish milestone {} first", prev.unwrap_or_default()),
                        }
                    } else {
                        ItemState::Locked {
                            reason: "Finish every lesson and solve a Medium problem first".into(),
                        }
                    }
                }
            };
            PathNode {
                kind: i.kind,
                id: i.id.clone(),
                title,
                level,
                xp,
                state,
            }
        })
        .collect()
}

/// "LM3" → "LM2"; "LM1" → None.
pub fn previous_milestone(id: &str) -> Option<String> {
    let n: u32 = id.strip_prefix("LM")?.parse().ok()?;
    (n > 1).then(|| format!("LM{}", n - 1))
}

/// Can hint `n` (1-based) be opened? Hint 1 always; later hints 30 s after the
/// previous one was opened, or after one more submit.
pub fn hint_available(n: u32, opened: &BTreeMap<u32, (i64, u32)>, now: i64, submits: u32) -> bool {
    if n <= 1 {
        return true;
    }
    match opened.get(&(n - 1)) {
        Some((at, submits_then)) => now - at >= HINT_WAIT_SECS || submits > *submits_then,
        None => false,
    }
}

pub fn solution_available(
    accepted: bool,
    failed_submits: u32,
    hints_opened: usize,
    hint_count: usize,
) -> bool {
    accepted || (failed_submits >= SOLUTION_AFTER_FAILS && hints_opened >= hint_count)
}

/// Mastery after decay: 10% of its value per week without practice.
pub fn decayed(score: f64, weeks: f64) -> f64 {
    score * (1.0 - MASTERY_DECAY_PER_WEEK).powf(weeks.max(0.0))
}

/// Level from total XP: level n needs 25·n² XP.
pub fn level_for(xp: i64) -> (u32, i64, i64) {
    let mut lvl = 1u32;
    while 25 * (lvl as i64) * (lvl as i64) <= xp {
        lvl += 1;
    }
    let floor = 25 * ((lvl - 1) as i64).pow(2);
    let next = 25 * (lvl as i64).pow(2);
    (lvl, floor, next)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hints_wait_or_submit() {
        let mut opened = BTreeMap::new();
        assert!(hint_available(1, &opened, 0, 0));
        assert!(!hint_available(2, &opened, 0, 0));
        opened.insert(1, (100, 2));
        assert!(!hint_available(2, &opened, 110, 2));
        assert!(hint_available(2, &opened, 130, 2));
        assert!(hint_available(2, &opened, 110, 3));
        assert!(!hint_available(3, &opened, 1000, 9));
    }

    #[test]
    fn solution_rule() {
        assert!(solution_available(true, 0, 0, 3));
        assert!(!solution_available(false, 5, 2, 3));
        assert!(!solution_available(false, 4, 3, 3));
        assert!(solution_available(false, 5, 3, 3));
    }

    #[test]
    fn decay_and_levels() {
        assert!((decayed(100.0, 1.0) - 90.0).abs() < 1e-9);
        assert!((decayed(100.0, 2.0) - 81.0).abs() < 1e-9);
        assert_eq!(level_for(0).0, 1);
        assert_eq!(level_for(25).0, 2);
        assert_eq!(level_for(99).0, 2);
        assert_eq!(level_for(100).0, 3);
    }

    #[test]
    fn milestones_chain() {
        assert_eq!(previous_milestone("LM1"), None);
        assert_eq!(previous_milestone("LM2").as_deref(), Some("LM1"));
    }
}
