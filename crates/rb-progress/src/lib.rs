//! The learner's progress: a local SQLite store and the rules that read it.

pub mod badges;
pub mod rules;

use anyhow::Result;
use rb_content::{Chapter, Level, Problem, Question};
use rules::{SetScore, Snapshot};
use rusqlite::{Connection, OptionalExtension, params};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use ts_rs::TS;

const SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS meta (key TEXT PRIMARY KEY, value TEXT NOT NULL);
CREATE TABLE IF NOT EXISTS profile (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    experience TEXT NOT NULL DEFAULT 'beginner',
    languages TEXT NOT NULL DEFAULT '[]',
    coming_from TEXT NOT NULL DEFAULT 'python',
    daily_goal INTEGER NOT NULL DEFAULT 30,
    onboarded INTEGER NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS settings (key TEXT PRIMARY KEY, value TEXT NOT NULL);
CREATE TABLE IF NOT EXISTS lessons (
    course TEXT NOT NULL, lesson TEXT NOT NULL,
    tries INTEGER NOT NULL, first_try INTEGER NOT NULL, completed_at INTEGER NOT NULL,
    PRIMARY KEY (course, lesson)
);
CREATE TABLE IF NOT EXISTS question_sets (
    course TEXT NOT NULL, set_id TEXT NOT NULL,
    best REAL NOT NULL, first REAL NOT NULL, attempts INTEGER NOT NULL, completed_at INTEGER NOT NULL,
    PRIMARY KEY (course, set_id)
);
CREATE TABLE IF NOT EXISTS answers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    course TEXT NOT NULL, question TEXT NOT NULL, correct INTEGER NOT NULL, at INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS problems (
    course TEXT NOT NULL, problem TEXT NOT NULL,
    submits INTEGER NOT NULL DEFAULT 0, failed_submits INTEGER NOT NULL DEFAULT 0,
    runs INTEGER NOT NULL DEFAULT 0,
    accepted_at INTEGER,
    PRIMARY KEY (course, problem)
);
CREATE TABLE IF NOT EXISTS submissions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    course TEXT NOT NULL, problem TEXT NOT NULL, mode TEXT NOT NULL, verdict TEXT NOT NULL,
    files TEXT NOT NULL, at INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS test_failures (
    course TEXT NOT NULL, problem TEXT NOT NULL, test TEXT NOT NULL, fails INTEGER NOT NULL,
    PRIMARY KEY (course, problem, test)
);
CREATE TABLE IF NOT EXISTS hints (
    course TEXT NOT NULL, problem TEXT NOT NULL, n INTEGER NOT NULL, opened_at INTEGER NOT NULL, submits_then INTEGER NOT NULL,
    PRIMARY KEY (course, problem, n)
);
CREATE TABLE IF NOT EXISTS drafts (
    course TEXT NOT NULL, problem TEXT NOT NULL, files TEXT NOT NULL, updated_at INTEGER NOT NULL,
    PRIMARY KEY (course, problem)
);
CREATE TABLE IF NOT EXISTS xp_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    course TEXT NOT NULL, item TEXT NOT NULL, reason TEXT NOT NULL, xp INTEGER NOT NULL,
    at INTEGER NOT NULL, day TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS mastery (
    course TEXT NOT NULL, concept TEXT NOT NULL, score REAL NOT NULL, updated_at INTEGER NOT NULL,
    PRIMARY KEY (course, concept)
);
CREATE TABLE IF NOT EXISTS badges (id TEXT PRIMARY KEY, earned_at INTEGER NOT NULL);
CREATE TABLE IF NOT EXISTS checkpoints (
    course TEXT NOT NULL, problem TEXT NOT NULL, part INTEGER NOT NULL, at INTEGER NOT NULL,
    PRIMARY KEY (course, problem, part)
);
CREATE TABLE IF NOT EXISTS test_outs (course TEXT NOT NULL, chapter INTEGER NOT NULL, at INTEGER NOT NULL, PRIMARY KEY (course, chapter));
CREATE TABLE IF NOT EXISTS reviews (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    course TEXT NOT NULL, chapter INTEGER NOT NULL, problem TEXT NOT NULL, at INTEGER NOT NULL
);
"#;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Profile {
    /// beginner | experienced
    pub experience: String,
    pub languages: Vec<String>,
    /// python | js | java | c | go
    pub coming_from: String,
    pub daily_goal: i64,
    pub onboarded: bool,
}

/// What the learner earned from an action.
#[derive(Debug, Clone, Default, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Award {
    pub xp: i64,
    pub first_time: bool,
    pub badges: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Stats {
    pub xp: i64,
    pub level: u32,
    pub level_floor: i64,
    pub level_next: i64,
    pub today_xp: i64,
    pub daily_goal: i64,
    pub streak: u32,
    pub best_streak: u32,
    /// Last 70 days, oldest first: (YYYY-MM-DD, xp).
    pub calendar: Vec<(String, i64)>,
    pub lessons_done: u32,
    pub problems_accepted: u32,
    pub questions_correct: u32,
    pub submissions: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ProblemState {
    pub accepted: bool,
    pub submits: u32,
    pub failed_submits: u32,
    /// Hint number → seconds since it was opened.
    pub hints_opened: Vec<u32>,
    /// Seconds until the next hint can be opened (0 = now; null = all open or needs a submit).
    pub next_hint_in: Option<i64>,
    pub next_hint: Option<u32>,
    pub solution_available: bool,
    /// Hidden tests whose details are revealed (failed twice).
    pub revealed: Vec<String>,
    pub fails: BTreeMap<String, u32>,
    /// Checkpoint problems: the parts whose Submit was Accepted.
    pub checkpoints: Vec<u32>,
}

pub struct Store {
    conn: Connection,
    /// Overrides "now" in tests.
    pub clock: Option<i64>,
}

pub fn now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

impl Store {
    pub fn open(path: &Path) -> Result<Store> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        Self::init(Connection::open(path)?)
    }

    pub fn in_memory() -> Result<Store> {
        Self::init(Connection::open_in_memory()?)
    }

    fn init(conn: Connection) -> Result<Store> {
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        conn.execute_batch(SCHEMA)?;
        conn.execute(
            "INSERT OR IGNORE INTO profile (id, created_at) VALUES (1, ?1)",
            params![now()],
        )?;
        conn.execute(
            "INSERT OR IGNORE INTO meta (key, value) VALUES ('schema', '1')",
            [],
        )?;
        Ok(Store { conn, clock: None })
    }

    fn now(&self) -> i64 {
        self.clock.unwrap_or_else(now)
    }

    fn day(&self, at: i64) -> String {
        local_day(at)
    }

    // ------------------------------------------------------------ profile & settings

    pub fn profile(&self) -> Result<Profile> {
        Ok(self.conn.query_row("SELECT experience, languages, coming_from, daily_goal, onboarded FROM profile WHERE id = 1", [], |r| {
            Ok(Profile {
                experience: r.get(0)?,
                languages: serde_json::from_str(&r.get::<_, String>(1)?).unwrap_or_default(),
                coming_from: r.get(2)?,
                daily_goal: r.get(3)?,
                onboarded: r.get::<_, i64>(4)? != 0,
            })
        })?)
    }

    pub fn set_profile(&self, p: &Profile) -> Result<()> {
        self.conn.execute(
            "UPDATE profile SET experience = ?1, languages = ?2, coming_from = ?3, daily_goal = ?4, onboarded = ?5 WHERE id = 1",
            params![p.experience, serde_json::to_string(&p.languages)?, p.coming_from, p.daily_goal, p.onboarded as i64],
        )?;
        Ok(())
    }

    pub fn setting(&self, key: &str) -> Result<Option<serde_json::Value>> {
        let v: Option<String> = self
            .conn
            .query_row("SELECT value FROM settings WHERE key = ?1", [key], |r| {
                r.get(0)
            })
            .optional()?;
        Ok(v.and_then(|v| serde_json::from_str(&v).ok()))
    }

    pub fn settings(&self) -> Result<BTreeMap<String, serde_json::Value>> {
        let mut st = self.conn.prepare("SELECT key, value FROM settings")?;
        let rows = st.query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))?;
        let mut out = BTreeMap::new();
        for row in rows {
            let (k, v) = row?;
            if let Ok(v) = serde_json::from_str(&v) {
                out.insert(k, v);
            }
        }
        Ok(out)
    }

    pub fn set_setting(&self, key: &str, value: &serde_json::Value) -> Result<()> {
        self.conn.execute("INSERT INTO settings (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value = excluded.value", params![key, value.to_string()])?;
        Ok(())
    }

    /// Deletes all progress but keeps settings.
    pub fn reset_progress(&self) -> Result<()> {
        self.conn.execute_batch(
            "DELETE FROM lessons; DELETE FROM question_sets; DELETE FROM answers; DELETE FROM problems; DELETE FROM submissions;
             DELETE FROM test_failures; DELETE FROM hints; DELETE FROM drafts; DELETE FROM xp_events; DELETE FROM mastery;
             DELETE FROM badges; DELETE FROM test_outs; DELETE FROM reviews; DELETE FROM checkpoints;
             UPDATE profile SET onboarded = 0 WHERE id = 1;",
        )?;
        Ok(())
    }

    // ------------------------------------------------------------ snapshot

    pub fn snapshot(&self, course: &str) -> Result<Snapshot> {
        let mut s = Snapshot {
            onboarded: self.profile()?.onboarded,
            ..Default::default()
        };
        let mut st = self
            .conn
            .prepare("SELECT lesson FROM lessons WHERE course = ?1")?;
        s.lessons_done = st
            .query_map([course], |r| r.get(0))?
            .collect::<Result<BTreeSet<String>, _>>()?;
        let mut st = self
            .conn
            .prepare("SELECT set_id, best, first FROM question_sets WHERE course = ?1")?;
        for row in st.query_map([course], |r| {
            Ok((r.get::<_, String>(0)?, r.get(1)?, r.get(2)?))
        })? {
            let (id, best, first) = row?;
            s.sets_done.insert(id, SetScore { best, first });
        }
        let mut st = self.conn.prepare(
            "SELECT problem FROM problems WHERE course = ?1 AND accepted_at IS NOT NULL",
        )?;
        s.accepted = st
            .query_map([course], |r| r.get(0))?
            .collect::<Result<BTreeSet<String>, _>>()?;
        let mut st = self
            .conn
            .prepare("SELECT chapter FROM test_outs WHERE course = ?1")?;
        s.tested_out = st
            .query_map([course], |r| r.get::<_, u32>(0))?
            .collect::<Result<BTreeSet<u32>, _>>()?;
        Ok(s)
    }

    // ------------------------------------------------------------ XP

    fn add_xp(&self, course: &str, item: &str, reason: &str, xp: i64) -> Result<()> {
        if xp == 0 {
            return Ok(());
        }
        let at = self.now();
        self.conn.execute("INSERT INTO xp_events (course, item, reason, xp, at, day) VALUES (?1, ?2, ?3, ?4, ?5, ?6)", params![course, item, reason, xp, at, self.day(at)])?;
        Ok(())
    }

    fn earned(&self, course: &str, item: &str, reason: &str) -> Result<bool> {
        Ok(self.conn.query_row(
            "SELECT COUNT(*) FROM xp_events WHERE course = ?1 AND item = ?2 AND reason = ?3",
            params![course, item, reason],
            |r| r.get::<_, i64>(0),
        )? > 0)
    }

    pub fn total_xp(&self) -> Result<i64> {
        Ok(self
            .conn
            .query_row("SELECT COALESCE(SUM(xp), 0) FROM xp_events", [], |r| {
                r.get(0)
            })?)
    }

    pub fn stats(&self) -> Result<Stats> {
        let xp = self.total_xp()?;
        let (level, level_floor, level_next) = rules::level_for(xp);
        let today = self.day(self.now());
        let mut st = self
            .conn
            .prepare("SELECT day, SUM(xp) FROM xp_events GROUP BY day")?;
        let days: BTreeMap<String, i64> = st
            .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?
            .collect::<Result<_, _>>()?;
        let today_xp = days.get(&today).copied().unwrap_or(0);
        let (streak, best_streak) = streaks(&days, &today);
        let calendar = (0..70)
            .rev()
            .map(|i| {
                let d = self.day(self.now() - i * 86_400);
                let x = days.get(&d).copied().unwrap_or(0);
                (d, x)
            })
            .collect();
        let count = |sql: &str| -> Result<u32> {
            Ok(self.conn.query_row(sql, [], |r| r.get::<_, i64>(0))? as u32)
        };
        Ok(Stats {
            xp,
            level,
            level_floor,
            level_next,
            today_xp,
            daily_goal: self.profile()?.daily_goal,
            streak,
            best_streak,
            calendar,
            lessons_done: count("SELECT COUNT(*) FROM lessons")?,
            problems_accepted: count(
                "SELECT COUNT(*) FROM problems WHERE accepted_at IS NOT NULL",
            )?,
            questions_correct: count(
                "SELECT COUNT(DISTINCT question) FROM answers WHERE correct = 1",
            )?,
            submissions: count("SELECT COUNT(*) FROM submissions")?,
        })
    }

    // ------------------------------------------------------------ lessons & questions

    pub fn complete_lesson(
        &self,
        course: &str,
        lesson: &rb_content::Lesson,
        tries: u32,
    ) -> Result<Award> {
        let first = !self
            .conn
            .query_row(
                "SELECT COUNT(*) FROM lessons WHERE course = ?1 AND lesson = ?2",
                params![course, lesson.id],
                |r| r.get::<_, i64>(0),
            )
            .map(|n| n > 0)?;
        if first {
            self.conn.execute(
                "INSERT INTO lessons (course, lesson, tries, first_try, completed_at) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![course, lesson.id, tries, (tries == 0) as i64, self.now()],
            )?;
            self.add_xp(course, &lesson.id, "lesson", lesson.completion.xp as i64)?;
        }
        let badges = self.award_badges()?;
        Ok(Award {
            xp: if first {
                lesson.completion.xp as i64
            } else {
                0
            },
            first_time: first,
            badges,
        })
    }

    pub fn record_answer(&self, course: &str, q: &Question, correct: bool) -> Result<()> {
        let at = self.now();
        let before: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM answers WHERE course = ?1 AND question = ?2 AND correct = 1",
            params![course, q.id],
            |r| r.get(0),
        )?;
        self.conn.execute(
            "INSERT INTO answers (course, question, correct, at) VALUES (?1, ?2, ?3, ?4)",
            params![course, q.id, correct as i64, at],
        )?;
        if correct && before == 0 {
            for c in &q.concepts {
                self.bump_mastery(course, c, rules::MASTERY_PER_QUESTION)?;
            }
        }
        Ok(())
    }

    /// Finishes a question set with `score` (0–1, first-attempt correctness).
    /// A foundation lesson's set at ≥ 80% on the first attempt also completes the lesson (test-out).
    pub fn complete_set(
        &self,
        course: &str,
        ch: &Chapter,
        set_id: &str,
        score: f64,
    ) -> Result<Award> {
        let Some(set) = ch.question_set(set_id) else {
            anyhow::bail!("no question set {set_id}")
        };
        let existing: Option<(f64, i64)> = self
            .conn
            .query_row(
                "SELECT best, attempts FROM question_sets WHERE course = ?1 AND set_id = ?2",
                params![course, set_id],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .optional()?;
        let first_time = existing.is_none();
        match existing {
            None => {
                self.conn.execute("INSERT INTO question_sets (course, set_id, best, first, attempts, completed_at) VALUES (?1, ?2, ?3, ?3, 1, ?4)", params![course, set_id, score, self.now()])?;
                self.add_xp(course, set_id, "questions", set.xp as i64)?;
            }
            Some((best, attempts)) => {
                self.conn.execute("UPDATE question_sets SET best = ?3, attempts = ?4 WHERE course = ?1 AND set_id = ?2", params![course, set_id, best.max(score), attempts + 1])?;
            }
        }
        let mut award = Award {
            xp: if first_time { set.xp as i64 } else { 0 },
            first_time,
            badges: Vec::new(),
        };
        if first_time
            && score >= rules::FOLLOW_UP_TEST_OUT
            && let Some(lesson) = set.lesson.as_ref().and_then(|l| ch.lesson(l))
            && lesson.foundation
        {
            let a = self.complete_lesson(course, lesson, 0)?;
            award.xp += a.xp;
            if a.first_time {
                self.grant_badge("tested-out")?;
            }
        }
        award.badges = self.award_badges()?;
        Ok(award)
    }

    // ------------------------------------------------------------ problems

    fn ensure_problem(&self, course: &str, problem: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO problems (course, problem) VALUES (?1, ?2)",
            params![course, problem],
        )?;
        Ok(())
    }

    /// Records a Run or Submit and its per-test outcome. Returns the award (XP on first Accepted).
    #[allow(clippy::too_many_arguments)]
    pub fn record_grade(
        &self,
        course: &str,
        chapter: &Chapter,
        p: &Problem,
        submit: bool,
        verdict: &str,
        accepted: bool,
        failed_tests: &[String],
        files: &serde_json::Value,
    ) -> Result<Award> {
        self.ensure_problem(course, &p.id)?;
        let at = self.now();
        self.conn.execute("INSERT INTO submissions (course, problem, mode, verdict, files, at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)", params![course, p.id, if submit { "submit" } else { "run" }, verdict, files.to_string(), at])?;
        if !submit {
            self.conn.execute(
                "UPDATE problems SET runs = runs + 1 WHERE course = ?1 AND problem = ?2",
                params![course, p.id],
            )?;
            return Ok(Award::default());
        }
        self.conn.execute("UPDATE problems SET submits = submits + 1, failed_submits = failed_submits + ?3 WHERE course = ?1 AND problem = ?2", params![course, p.id, (!accepted) as i64])?;
        for t in failed_tests {
            self.conn.execute(
                "INSERT INTO test_failures (course, problem, test, fails) VALUES (?1, ?2, ?3, 1) ON CONFLICT(course, problem, test) DO UPDATE SET fails = fails + 1",
                params![course, p.id, t],
            )?;
        }
        let mut award = Award::default();
        if accepted {
            let was: Option<i64> = self.conn.query_row(
                "SELECT accepted_at FROM problems WHERE course = ?1 AND problem = ?2",
                params![course, p.id],
                |r| r.get(0),
            )?;
            if was.is_none() {
                self.conn.execute(
                    "UPDATE problems SET accepted_at = ?3 WHERE course = ?1 AND problem = ?2",
                    params![course, p.id, at],
                )?;
                self.add_xp(course, &p.id, "problem", p.xp as i64)?;
                award.xp = p.xp as i64;
                award.first_time = true;
                for c in &p.concepts {
                    self.bump_mastery(course, c, rules::MASTERY_PER_PROBLEM * p.level.weight())?;
                }
                if p.level == Level::Hard {
                    self.grant_badge("first-hard")?;
                }
            }
        }
        let _ = chapter;
        award.badges = self.award_badges()?;
        Ok(award)
    }

    /// A checkpoint part's Submit was Accepted.
    pub fn record_checkpoint(&self, course: &str, problem: &str, part: u32) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO checkpoints (course, problem, part, at) VALUES (?1, ?2, ?3, ?4)",
            params![course, problem, part, self.now()],
        )?;
        Ok(())
    }

    /// XP for an Accepted review problem: its level's XP, once per review visit.
    pub fn record_review(&self, course: &str, chapter: u32, p: &Problem) -> Result<Award> {
        let at = self.now();
        let day = self.day(at);
        let item = format!("review:{chapter}:{}:{day}", p.id);
        if self.earned(course, &item, "review")? {
            return Ok(Award::default());
        }
        self.conn.execute(
            "INSERT INTO reviews (course, chapter, problem, at) VALUES (?1, ?2, ?3, ?4)",
            params![course, chapter, p.id, at],
        )?;
        self.add_xp(course, &item, "review", p.xp as i64)?;
        for c in &p.concepts {
            self.bump_mastery(course, c, rules::MASTERY_PER_PROBLEM * p.level.weight())?;
        }
        Ok(Award {
            xp: p.xp as i64,
            first_time: true,
            badges: self.award_badges()?,
        })
    }

    pub fn last_reviewed(&self, course: &str) -> Result<Option<String>> {
        Ok(self
            .conn
            .query_row(
                "SELECT problem FROM reviews WHERE course = ?1 ORDER BY at DESC LIMIT 1",
                [course],
                |r| r.get(0),
            )
            .optional()?)
    }

    pub fn record_test_out(&self, course: &str, chapter: u32) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO test_outs (course, chapter, at) VALUES (?1, ?2, ?3)",
            params![course, chapter, self.now()],
        )?;
        self.grant_badge("tested-out")?;
        Ok(())
    }

    pub fn open_hint(&self, course: &str, p: &Problem, n: u32) -> Result<bool> {
        let st = self.problem_state(course, p)?;
        if st.hints_opened.contains(&n) {
            return Ok(true);
        }
        if st.next_hint != Some(n) || st.next_hint_in != Some(0) {
            return Ok(false);
        }
        self.conn.execute("INSERT OR IGNORE INTO hints (course, problem, n, opened_at, submits_then) VALUES (?1, ?2, ?3, ?4, ?5)", params![course, p.id, n, self.now(), st.submits])?;
        Ok(true)
    }

    pub fn problem_state(&self, course: &str, p: &Problem) -> Result<ProblemState> {
        let row: Option<(i64, i64, Option<i64>)> = self
            .conn
            .query_row("SELECT submits, failed_submits, accepted_at FROM problems WHERE course = ?1 AND problem = ?2", params![course, p.id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))
            .optional()?;
        let (submits, failed, accepted_at) = row.unwrap_or((0, 0, None));
        let mut st = self.conn.prepare(
            "SELECT n, opened_at, submits_then FROM hints WHERE course = ?1 AND problem = ?2",
        )?;
        let opened: BTreeMap<u32, (i64, u32)> = st
            .query_map(params![course, p.id], |r| {
                Ok((
                    r.get::<_, u32>(0)?,
                    (r.get::<_, i64>(1)?, r.get::<_, u32>(2)?),
                ))
            })?
            .collect::<Result<_, _>>()?;
        let mut st = self
            .conn
            .prepare("SELECT test, fails FROM test_failures WHERE course = ?1 AND problem = ?2")?;
        let fails: BTreeMap<String, u32> = st
            .query_map(params![course, p.id], |r| Ok((r.get(0)?, r.get(1)?)))?
            .collect::<Result<_, _>>()?;
        let now = self.now();
        let hint_count = p.hints.len() as u32;
        let next_hint = (1..=hint_count).find(|n| !opened.contains_key(n));
        let next_hint_in = next_hint.and_then(|n| {
            if rules::hint_available(n, &opened, now, submits as u32) {
                Some(0)
            } else {
                opened
                    .get(&(n - 1))
                    .map(|(at, _)| (at + rules::HINT_WAIT_SECS - now).max(1))
            }
        });
        let revealed = p
            .tests
            .iter()
            .filter(|t| {
                !t.visible && fails.get(&t.key).copied().unwrap_or(0) >= rules::HIDDEN_REVEAL_AFTER
            })
            .map(|t| t.key.clone())
            .collect();
        let accepted = accepted_at.is_some();
        let mut st = self.conn.prepare(
            "SELECT part FROM checkpoints WHERE course = ?1 AND problem = ?2 ORDER BY part",
        )?;
        let checkpoints: Vec<u32> = st
            .query_map(params![course, p.id], |r| r.get(0))?
            .collect::<Result<_, _>>()?;
        Ok(ProblemState {
            checkpoints,
            accepted,
            submits: submits as u32,
            failed_submits: failed as u32,
            hints_opened: opened.keys().copied().collect(),
            next_hint_in,
            next_hint,
            solution_available: rules::solution_available(
                accepted,
                failed as u32,
                opened.len(),
                p.hints.len(),
            ),
            revealed,
            fails,
        })
    }

    pub fn save_draft(&self, course: &str, problem: &str, files: &serde_json::Value) -> Result<()> {
        self.conn.execute(
            "INSERT INTO drafts (course, problem, files, updated_at) VALUES (?1, ?2, ?3, ?4) ON CONFLICT(course, problem) DO UPDATE SET files = excluded.files, updated_at = excluded.updated_at",
            params![course, problem, files.to_string(), self.now()],
        )?;
        Ok(())
    }

    pub fn draft(&self, course: &str, problem: &str) -> Result<Option<serde_json::Value>> {
        let v: Option<String> = self
            .conn
            .query_row(
                "SELECT files FROM drafts WHERE course = ?1 AND problem = ?2",
                params![course, problem],
                |r| r.get(0),
            )
            .optional()?;
        Ok(v.and_then(|v| serde_json::from_str(&v).ok()))
    }

    // ------------------------------------------------------------ mastery

    fn bump_mastery(&self, course: &str, concept: &str, by: f64) -> Result<()> {
        let now = self.now();
        let current = self.mastery_of(course, concept)?;
        let score = (current + by).min(100.0);
        self.conn.execute(
            "INSERT INTO mastery (course, concept, score, updated_at) VALUES (?1, ?2, ?3, ?4) ON CONFLICT(course, concept) DO UPDATE SET score = excluded.score, updated_at = excluded.updated_at",
            params![course, concept, score, now],
        )?;
        Ok(())
    }

    fn mastery_of(&self, course: &str, concept: &str) -> Result<f64> {
        let row: Option<(f64, i64)> = self
            .conn
            .query_row(
                "SELECT score, updated_at FROM mastery WHERE course = ?1 AND concept = ?2",
                params![course, concept],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .optional()?;
        Ok(row
            .map(|(s, at)| rules::decayed(s, (self.now() - at) as f64 / (7.0 * 86_400.0)))
            .unwrap_or(0.0))
    }

    /// Every concept's current (decayed) mastery.
    pub fn mastery(&self, course: &str) -> Result<BTreeMap<String, f64>> {
        let mut st = self
            .conn
            .prepare("SELECT concept, score, updated_at FROM mastery WHERE course = ?1")?;
        let now = self.now();
        let rows = st.query_map([course], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, f64>(1)?,
                r.get::<_, i64>(2)?,
            ))
        })?;
        let mut out = BTreeMap::new();
        for row in rows {
            let (c, s, at) = row?;
            out.insert(c, rules::decayed(s, (now - at) as f64 / (7.0 * 86_400.0)));
        }
        Ok(out)
    }

    // ------------------------------------------------------------ badges

    pub fn badges(&self) -> Result<Vec<(String, i64)>> {
        let mut st = self
            .conn
            .prepare("SELECT id, earned_at FROM badges ORDER BY earned_at")?;
        Ok(st
            .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?
            .collect::<Result<_, _>>()?)
    }

    pub fn grant_badge(&self, id: &str) -> Result<bool> {
        Ok(self.conn.execute(
            "INSERT OR IGNORE INTO badges (id, earned_at) VALUES (?1, ?2)",
            params![id, self.now()],
        )? > 0)
    }

    /// Grants every badge whose condition now holds; returns the new ones.
    pub fn award_badges(&self) -> Result<Vec<String>> {
        let stats = self.stats()?;
        let mut new = Vec::new();
        for b in badges::BADGES {
            if (b.earned)(&stats) && self.grant_badge(b.id)? {
                new.push(b.id.to_string());
            }
        }
        Ok(new)
    }

    /// Chapter-complete badges, given the computed path.
    pub fn award_chapter_badges(&self, path: &[rules::ChapterState]) -> Result<Vec<String>> {
        let mut new = Vec::new();
        for c in path {
            if c.passed {
                let id = format!("chapter-{}", c.num);
                if self.grant_badge(&id)? {
                    new.push(id);
                }
            }
        }
        Ok(new)
    }
}

/// Current and best streaks from per-day XP. A day counts with ≥ 20 XP; today
/// doesn't break the streak until it's over.
fn streaks(days: &BTreeMap<String, i64>, today: &str) -> (u32, u32) {
    let good: BTreeSet<&str> = days
        .iter()
        .filter(|(_, x)| **x >= rules::STREAK_DAY_XP)
        .map(|(d, _)| d.as_str())
        .collect();
    let mut best = 0;
    let mut run = 0;
    let mut prev: Option<i64> = None;
    for d in &good {
        let n = day_number(d);
        run = if prev == Some(n - 1) { run + 1 } else { 1 };
        best = best.max(run);
        prev = Some(n);
    }
    let t = day_number(today);
    let mut current = 0;
    let mut d = if good.contains(today) { t } else { t - 1 };
    while good.contains(day_string(d).as_str()) {
        current += 1;
        d -= 1;
    }
    (current, best)
}

/// Local calendar day "YYYY-MM-DD" for a unix time, using the system's UTC offset.
pub fn local_day(at: i64) -> String {
    day_string((at + utc_offset_secs()).div_euclid(86_400))
}

fn utc_offset_secs() -> i64 {
    #[cfg(unix)]
    unsafe {
        let t: libc::time_t = now() as libc::time_t;
        let mut tm: libc::tm = std::mem::zeroed();
        if libc::localtime_r(&t, &mut tm).is_null() {
            return 0;
        }
        tm.tm_gmtoff
    }
    #[cfg(not(unix))]
    {
        0
    }
}

/// Days since 1970-01-01 → "YYYY-MM-DD" (proleptic Gregorian).
fn day_string(days: i64) -> String {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    format!("{:04}-{:02}-{:02}", if m <= 2 { y + 1 } else { y }, m, d)
}

fn day_number(s: &str) -> i64 {
    let mut it = s.split('-').map(|p| p.parse::<i64>().unwrap_or(0));
    let (y, m, d) = (
        it.next().unwrap_or(1970),
        it.next().unwrap_or(1),
        it.next().unwrap_or(1),
    );
    let y = if m <= 2 { y - 1 } else { y };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let mp = if m > 2 { m - 3 } else { m + 9 };
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

#[cfg(test)]
mod tests;
