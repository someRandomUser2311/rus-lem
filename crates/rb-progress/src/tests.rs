//! Rules from progress-and-unlocking.md, checked against the bundled Chapter 1.

use super::*;
use rb_content::{Bundle, Course};
use rules::{ItemState, path};

fn bundle() -> (Course, Vec<Chapter>) {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../app/src-tauri/resources/content/rust-basics");
    let course = Bundle::course(&dir).unwrap();
    let chapters = course
        .chapters
        .iter()
        .filter(|c| c.available)
        .map(|c| Bundle::chapter(&dir, &c.id).unwrap())
        .collect();
    (course, chapters)
}

fn state<'a>(p: &'a [rules::ChapterState], id: &str) -> &'a ItemState {
    &p[0]
        .nodes
        .iter()
        .find(|n| n.id == id)
        .unwrap_or_else(|| panic!("no node {id}"))
        .state
}

const C: &str = "rust-basics";

#[test]
fn chapter_one_unlocking() {
    let (course, chapters) = bundle();
    let ch = &chapters[0];
    let store = Store::in_memory().unwrap();

    // Before onboarding everything is locked.
    let p = path(&course, &chapters, &store.snapshot(C).unwrap());
    assert!(matches!(p[0].state, ItemState::Locked { .. }));
    assert!(matches!(p[1].state, ItemState::Locked { .. }));

    let mut prof = store.profile().unwrap();
    prof.onboarded = true;
    store.set_profile(&prof).unwrap();
    let p = path(&course, &chapters, &store.snapshot(C).unwrap());
    assert_eq!(p[0].state, ItemState::Open);
    // Chapter 2 waits for Chapter 1 (or is "Coming soon" while unbundled).
    assert!(matches!(p[1].state, ItemState::Locked { ref reason }
        if reason.starts_with("Finish Chapter 1") || reason == "Coming soon"));
    assert_eq!(*state(&p, "L1.1"), ItemState::Open);
    assert!(matches!(state(&p, "L1.2"), ItemState::Locked { .. }));
    // L1.1 is a foundation lesson: its questions are open as a test-out.
    assert_eq!(*state(&p, "Q1.1"), ItemState::Open);
    assert!(matches!(state(&p, "Q1.2"), ItemState::Locked { .. }));
    assert!(matches!(state(&p, "P01-01"), ItemState::Locked { .. }));
    assert!(matches!(state(&p, "Q1.P"), ItemState::Locked { .. }));
    assert!(matches!(state(&p, "LM1"), ItemState::Locked { .. }));

    // Test out of L1.1 with 80% on the first try.
    let a = store.complete_set(C, ch, "Q1.1", 0.8).unwrap();
    assert_eq!(a.xp, 10 + 20);
    let p = path(&course, &chapters, &store.snapshot(C).unwrap());
    assert_eq!(*state(&p, "L1.1"), ItemState::Done);
    assert_eq!(*state(&p, "L1.2"), ItemState::Open);

    store
        .complete_lesson(C, ch.lesson("L1.2").unwrap(), 0)
        .unwrap();
    let p = path(&course, &chapters, &store.snapshot(C).unwrap());
    assert_eq!(*state(&p, "P01-01"), ItemState::Open);
    assert_eq!(*state(&p, "Q1.2"), ItemState::Open);
    assert!(matches!(state(&p, "P01-05"), ItemState::Locked { .. }));

    store
        .complete_lesson(C, ch.lesson("L1.3").unwrap(), 2)
        .unwrap();
    let p = path(&course, &chapters, &store.snapshot(C).unwrap());
    assert_eq!(*state(&p, "Q1.P"), ItemState::Open);
    // LM1 needs a Medium (P01-06) Accepted.
    assert!(matches!(state(&p, "LM1"), ItemState::Locked { .. }));

    let files = serde_json::json!([]);
    let p6 = ch.problem("P01-06").unwrap();
    let a = store
        .record_grade(C, ch, p6, true, "accepted", true, &[], &files)
        .unwrap();
    assert_eq!(a.xp, 20);
    assert!(a.badges.contains(&"first-accepted".to_string()));
    // XP is given once.
    assert_eq!(
        store
            .record_grade(C, ch, p6, true, "accepted", true, &[], &files)
            .unwrap()
            .xp,
        0
    );
    let p = path(&course, &chapters, &store.snapshot(C).unwrap());
    assert_eq!(*state(&p, "LM1"), ItemState::Open);

    // 60% of 6 problems = 4 (rounded up); Chapter 1 has no Hard.
    assert_eq!((p[0].needed, p[0].accepted, p[0].needs_hard), (4, 1, false));
    for id in ["P01-01", "P01-02", "P01-03"] {
        store
            .record_grade(
                C,
                ch,
                ch.problem(id).unwrap(),
                true,
                "accepted",
                true,
                &[],
                &files,
            )
            .unwrap();
    }
    let p = path(&course, &chapters, &store.snapshot(C).unwrap());
    assert!(p[0].passed);
    assert_eq!(
        store.award_chapter_badges(&p).unwrap(),
        vec!["chapter-1".to_string()]
    );
}

#[test]
fn hidden_tests_reveal_and_solution() {
    let (_, chapters) = bundle();
    let ch = &chapters[0];
    let p = ch.problem("P01-03").unwrap();
    let store = Store::in_memory().unwrap();
    let files = serde_json::json!([]);
    store
        .record_grade(
            C,
            ch,
            p,
            true,
            "wrong_answer",
            false,
            &["t09".into()],
            &files,
        )
        .unwrap();
    assert!(store.problem_state(C, p).unwrap().revealed.is_empty());
    store
        .record_grade(
            C,
            ch,
            p,
            true,
            "wrong_answer",
            false,
            &["t09".into()],
            &files,
        )
        .unwrap();
    assert_eq!(
        store.problem_state(C, p).unwrap().revealed,
        vec!["t09".to_string()]
    );

    // Solution after 5 failed submits and all 3 hints.
    for _ in 0..3 {
        store
            .record_grade(C, ch, p, true, "wrong_answer", false, &[], &files)
            .unwrap();
    }
    let mut s = Store {
        conn: store.conn,
        clock: Some(now()),
    };
    assert!(!s.problem_state(C, p).unwrap().solution_available);
    assert!(s.open_hint(C, p, 1).unwrap());
    assert!(!s.open_hint(C, p, 2).unwrap(), "hint 2 waits 30 s");
    s.clock = Some(now() + 31);
    assert!(s.open_hint(C, p, 2).unwrap());
    s.clock = Some(now() + 62);
    assert!(s.open_hint(C, p, 3).unwrap());
    assert!(s.problem_state(C, p).unwrap().solution_available);
}

#[test]
fn streak_counting() {
    let mut days = BTreeMap::new();
    for d in ["2026-10-20", "2026-10-21", "2026-10-22", "2026-10-24"] {
        days.insert(d.to_string(), 25);
    }
    days.insert("2026-10-23".into(), 5); // under 20 XP: doesn't count
    assert_eq!(streaks(&days, "2026-10-24"), (1, 3));
    assert_eq!(streaks(&days, "2026-10-25"), (1, 3));
    assert_eq!(streaks(&days, "2026-10-26"), (0, 3));
    assert_eq!(day_string(day_number("2024-02-29")), "2024-02-29");
    assert_eq!(day_number("1970-01-02"), 1);
}

#[test]
fn mastery_grows_and_caps() {
    let (_, chapters) = bundle();
    let ch = &chapters[0];
    let store = Store::in_memory().unwrap();
    let files = serde_json::json!([]);
    store
        .record_grade(
            C,
            ch,
            ch.problem("P01-06").unwrap(),
            true,
            "accepted",
            true,
            &[],
            &files,
        )
        .unwrap();
    let m = store.mastery(C).unwrap();
    assert!((m["basic.println"] - 20.0).abs() < 0.01, "{m:?}"); // 25 × 0.8
}
