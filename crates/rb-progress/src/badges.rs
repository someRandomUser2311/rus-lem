//! Badge definitions. Chapter badges (`chapter-N`) and part badges (`part-I`) are
//! granted by id; the rest are checked against the learner's stats.

use crate::Stats;
use serde::Serialize;
use ts_rs::TS;

pub struct Badge {
    pub id: &'static str,
    pub earned: fn(&Stats) -> bool,
}

pub const BADGES: &[Badge] = &[
    Badge {
        id: "first-lesson",
        earned: |s| s.lessons_done >= 1,
    },
    Badge {
        id: "first-accepted",
        earned: |s| s.problems_accepted >= 1,
    },
    Badge {
        id: "ten-accepted",
        earned: |s| s.problems_accepted >= 10,
    },
    Badge {
        id: "fifty-accepted",
        earned: |s| s.problems_accepted >= 50,
    },
    Badge {
        id: "hundred-questions",
        earned: |s| s.questions_correct >= 100,
    },
    Badge {
        id: "streak-3",
        earned: |s| s.best_streak >= 3,
    },
    Badge {
        id: "streak-7",
        earned: |s| s.best_streak >= 7,
    },
    Badge {
        id: "streak-30",
        earned: |s| s.best_streak >= 30,
    },
    Badge {
        id: "xp-100",
        earned: |s| s.xp >= 100,
    },
    Badge {
        id: "xp-1000",
        earned: |s| s.xp >= 1000,
    },
    Badge {
        id: "xp-5000",
        earned: |s| s.xp >= 5000,
    },
    Badge {
        id: "persistent",
        earned: |s| s.submissions >= 25,
    },
];

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct BadgeInfo {
    pub id: String,
    pub title: String,
    pub description: String,
    pub icon: String,
}

/// Title, description and icon of a badge id (including `chapter-N` and `part-X`).
pub fn info(id: &str) -> BadgeInfo {
    let (title, description, icon) = match id {
        "first-lesson" => (
            "First steps".to_string(),
            "Finished your first lesson.".to_string(),
            "footprints",
        ),
        "first-accepted" => (
            "Accepted!".to_string(),
            "Solved your first coding problem.".to_string(),
            "check",
        ),
        "ten-accepted" => (
            "Ten down".to_string(),
            "Solved 10 coding problems.".to_string(),
            "stack",
        ),
        "fifty-accepted" => (
            "Half a hundred".to_string(),
            "Solved 50 coding problems.".to_string(),
            "mountain",
        ),
        "hundred-questions" => (
            "Quiz whiz".to_string(),
            "Answered 100 different questions correctly.".to_string(),
            "spark",
        ),
        "streak-3" => (
            "On a roll".to_string(),
            "Kept a 3-day streak.".to_string(),
            "flame",
        ),
        "streak-7" => (
            "Week of Rust".to_string(),
            "Kept a 7-day streak.".to_string(),
            "flame",
        ),
        "streak-30" => (
            "Month of Rust".to_string(),
            "Kept a 30-day streak.".to_string(),
            "flame",
        ),
        "xp-100" => ("100 XP".to_string(), "Earned 100 XP.".to_string(), "star"),
        "xp-1000" => (
            "1,000 XP".to_string(),
            "Earned 1,000 XP.".to_string(),
            "star",
        ),
        "xp-5000" => (
            "5,000 XP".to_string(),
            "Earned 5,000 XP.".to_string(),
            "star",
        ),
        "persistent" => (
            "Persistent".to_string(),
            "Submitted code 25 times. Every failure taught you something.".to_string(),
            "hammer",
        ),
        "first-hard" => (
            "Hard mode".to_string(),
            "Solved your first Hard problem.".to_string(),
            "diamond",
        ),
        "tested-out" => (
            "Already knew it".to_string(),
            "Tested out of a lesson or chapter.".to_string(),
            "rocket",
        ),
        other => {
            if let Some(n) = other.strip_prefix("chapter-") {
                (
                    format!("Chapter {n} complete"),
                    format!("Met the goal for Chapter {n} and opened the next one."),
                    "flag",
                )
            } else if let Some(p) = other.strip_prefix("part-") {
                (
                    format!("Part {p} passed"),
                    format!("Passed the checkpoint exam for Part {p}."),
                    "trophy",
                )
            } else {
                (other.to_string(), String::new(), "star")
            }
        }
    };
    BadgeInfo {
        id: id.to_string(),
        title,
        description,
        icon: icon.to_string(),
    }
}

/// Every badge that can be earned in a course with `chapters` chapters and the given parts.
pub fn all(chapters: &[u32], parts: &[String]) -> Vec<BadgeInfo> {
    let mut ids: Vec<String> = BADGES.iter().map(|b| b.id.to_string()).collect();
    ids.extend(["first-hard", "tested-out"].map(String::from));
    ids.extend(chapters.iter().map(|n| format!("chapter-{n}")));
    ids.extend(parts.iter().map(|p| format!("part-{p}")));
    ids.iter().map(|i| info(i)).collect()
}
