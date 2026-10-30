//! The app's commands as plain functions over [`AppState`]. `tauri_cmds` exposes them to
//! the window; `bin/web.rs` exposes the same functions over HTTP for browser previews.

use crate::AppState;
use crate::views::{self, ProblemView};
use rb_content::{Chapter, Concept, Lesson, Level, QuestionSet, SourceFile};
use rb_grader::{GradeEvent, Grader, Mode, TestStatus, Verdict};
use rb_progress::badges::{self, BadgeInfo};
use rb_progress::rules::{self, ChapterState};
use rb_progress::{Award, Profile, Stats};
use rb_quiz::{Checked, Response};
use rb_runner::Toolchain;
use serde::Serialize;
use std::collections::BTreeMap;
use std::sync::atomic::Ordering;
use ts_rs::TS;

type R<T> = Result<T, String>;

fn e(err: impl std::fmt::Display) -> String {
    err.to_string()
}

fn course_id(st: &AppState) -> &str {
    &st.course.id
}

// ------------------------------------------------------------------ app & profile

#[derive(Serialize, TS)]
#[ts(export)]
pub struct ToolchainStatus {
    pub found: bool,
    pub rustc_version: Option<String>,
    pub min_rustc: String,
    pub target_rustc: String,
    pub new_enough: bool,
    pub rust_analyzer: bool,
    pub error: Option<String>,
}

#[derive(Serialize, TS)]
#[ts(export)]
pub struct AppInfo {
    pub course_id: String,
    pub course_title: String,
    pub concepts: Vec<Concept>,
    pub profile: Profile,
    pub settings: BTreeMap<String, serde_json::Value>,
    pub stats: Stats,
    pub toolchain: ToolchainStatus,
}

fn toolchain_status(st: &AppState) -> ToolchainStatus {
    let tc = st.toolchain.lock().unwrap();
    let min = st.course.toolchain.min_rustc.clone();
    match tc.as_ref() {
        Some(t) => ToolchainStatus {
            found: true,
            rustc_version: Some(t.rustc_version.clone()),
            new_enough: t.at_least(&min),
            min_rustc: min,
            target_rustc: st.course.toolchain.target_rustc.clone(),
            rust_analyzer: t.rust_analyzer.is_some(),
            error: None,
        },
        None => ToolchainStatus {
            found: false,
            rustc_version: None,
            new_enough: false,
            min_rustc: min,
            target_rustc: st.course.toolchain.target_rustc.clone(),
            rust_analyzer: false,
            error: Toolchain::detect().err().map(|e| e.to_string()),
        },
    }
}

pub fn app_info(st: &AppState) -> R<AppInfo> {
    let store = st.store.lock().unwrap();
    Ok(AppInfo {
        course_id: st.course.id.clone(),
        course_title: st.course.title.clone(),
        concepts: st.course.concepts.clone(),
        profile: store.profile().map_err(e)?,
        settings: store.settings().map_err(e)?,
        stats: store.stats().map_err(e)?,
        toolchain: toolchain_status(st),
    })
}

/// Looks for the toolchain again (after the learner installed Rust or rust-analyzer).
pub fn check_toolchain(st: &AppState) -> ToolchainStatus {
    *st.toolchain.lock().unwrap() = Toolchain::detect().ok();
    toolchain_status(st)
}

/// First launch: build a tiny Cargo project (with the vendored crates, when the
/// course has them) so Cargo's caches and the shared target dir are warm.
pub fn warm_up(st: &AppState) -> R<bool> {
    let tc = st
        .toolchain
        .lock()
        .unwrap()
        .clone()
        .ok_or("Rust isn't installed")?;
    let dir = st.data_dir.join("warmup");
    std::fs::create_dir_all(dir.join("src")).map_err(e)?;
    let deps = if st.vendor_dir.is_some() {
        "rand = \"0.10.1\"\n"
    } else {
        ""
    };
    std::fs::write(
        dir.join("Cargo.toml"),
        format!(
            "[package]\nname = \"warmup\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\n{deps}"
        ),
    )
    .map_err(e)?;
    std::fs::write(
        dir.join("src/main.rs"),
        "fn main() {\n    println!(\"ready\");\n}\n",
    )
    .map_err(e)?;
    let target = st.data_dir.join("target");
    let c = tc
        .cargo(&dir, "build", Some(&target), st.vendor_dir.as_deref(), None)
        .map_err(e)?;
    Ok(c.ok)
}

pub fn save_profile(st: &AppState, profile: Profile) -> R<()> {
    st.store.lock().unwrap().set_profile(&profile).map_err(e)
}

pub fn set_setting(st: &AppState, key: String, value: serde_json::Value) -> R<()> {
    st.store
        .lock()
        .unwrap()
        .set_setting(&key, &value)
        .map_err(e)
}

pub fn reset_progress(st: &AppState) -> R<()> {
    st.store.lock().unwrap().reset_progress().map_err(e)
}

// ------------------------------------------------------------------ path & chapters

fn snapshot(st: &AppState) -> R<rules::Snapshot> {
    let store = st.store.lock().unwrap();
    let mut s = store.snapshot(course_id(st)).map_err(e)?;
    s.experienced = store.profile().map_err(e)?.experience == "experienced";
    Ok(s)
}

pub fn get_path(st: &AppState) -> R<Vec<ChapterState>> {
    let s = snapshot(st)?;
    let path = rules::path(&st.course, &st.chapters, &s);
    st.store
        .lock()
        .unwrap()
        .award_chapter_badges(&path)
        .map_err(e)?;
    Ok(path)
}

#[derive(Serialize, TS)]
#[ts(export)]
pub struct ProblemSummary {
    pub id: String,
    pub title: String,
    pub level: Level,
    pub xp: u32,
    pub comes_after: String,
    pub concepts: Vec<String>,
}

#[derive(Serialize, TS)]
#[ts(export)]
pub struct ChapterView {
    pub num: u32,
    pub id: String,
    pub title: String,
    pub part: String,
    pub starter_stage: String,
    pub book_sections: Vec<String>,
    pub objectives: Vec<String>,
    pub lessons: Vec<Lesson>,
    pub question_sets: Vec<QuestionSet>,
    pub problems: Vec<ProblemSummary>,
}

pub fn get_chapter(st: &AppState, id: String) -> R<ChapterView> {
    let ch = st.chapter(&id)?;
    Ok(ChapterView {
        num: ch.num,
        id: ch.id.clone(),
        title: ch.title.clone(),
        part: ch.part.clone(),
        starter_stage: ch.starter_stage.clone(),
        book_sections: ch.book_sections.clone(),
        objectives: ch.objectives.clone(),
        lessons: ch.lessons.clone(),
        question_sets: ch.question_sets.clone(),
        problems: ch
            .problems
            .iter()
            .map(|p| ProblemSummary {
                id: p.id.clone(),
                title: p.title.clone(),
                level: p.level,
                xp: p.xp,
                comes_after: p.comes_after.clone(),
                concepts: p.concepts.clone(),
            })
            .collect(),
    })
}

// ------------------------------------------------------------------ lessons & questions

pub fn complete_lesson(st: &AppState, lesson: String, tries: u32) -> R<Award> {
    let ch = st.chapter_of(&lesson)?;
    let l = ch.lesson(&lesson).ok_or("no lesson")?;
    st.store
        .lock()
        .unwrap()
        .complete_lesson(course_id(st), l, tries)
        .map_err(e)
}

fn find_question<'a>(ch: &'a Chapter, id: &str) -> Option<&'a rb_content::Question> {
    ch.question_sets
        .iter()
        .flat_map(|s| s.questions.iter())
        .chain(
            ch.lessons
                .iter()
                .flat_map(|l| l.screens.iter().filter_map(|s| s.question.as_ref())),
        )
        .find(|q| q.id == id)
}

/// Checks an answer. Follow-up and practice answers are recorded (mastery); quick
/// checks aren't.
pub fn check_answer(
    st: &AppState,
    question: String,
    response: Response,
    record: bool,
) -> R<Checked> {
    let ch = st
        .chapters
        .iter()
        .find(|c| find_question(c, &question).is_some())
        .ok_or_else(|| format!("no question {question}"))?;
    let q = find_question(ch, &question).ok_or("no question")?;
    let checked = rb_quiz::check(q, &response);
    if record {
        st.store
            .lock()
            .unwrap()
            .record_answer(course_id(st), q, checked.correct)
            .map_err(e)?;
    }
    Ok(checked)
}

pub fn complete_set(st: &AppState, set: String, score: f64) -> R<Award> {
    let ch = st.chapter_of(&set)?;
    let store = st.store.lock().unwrap();
    let mut award = store
        .complete_set(course_id(st), ch, &set, score)
        .map_err(e)?;
    if ch.question_set(&set).is_some_and(|s| s.practice) {
        award.badges.extend(check_test_out(st, &store, ch)?);
    }
    Ok(award)
}

/// Chapter test-out: practice set at 80% on the first try plus 2 Hard problems Accepted.
fn check_test_out(st: &AppState, store: &rb_progress::Store, ch: &Chapter) -> R<Vec<String>> {
    let snap = store.snapshot(course_id(st)).map_err(e)?;
    let practice = ch
        .question_sets
        .iter()
        .find(|s| s.practice)
        .map(|s| s.id.clone())
        .unwrap_or_default();
    let first = snap
        .sets_done
        .get(&practice)
        .map(|s| s.first)
        .unwrap_or(0.0);
    let hards = rules::counted(ch)
        .filter(|p| p.level == Level::Hard && snap.accepted.contains(&p.id))
        .count();
    if first >= rules::FOLLOW_UP_TEST_OUT && hards >= 2 && !snap.tested_out.contains(&ch.num) {
        store.record_test_out(course_id(st), ch.num).map_err(e)?;
        return Ok(vec!["tested-out".into()]);
    }
    Ok(Vec::new())
}

// ------------------------------------------------------------------ problems

fn draft(store: &rb_progress::Store, course: &str, id: &str) -> Option<Vec<SourceFile>> {
    store
        .draft(course, id)
        .ok()
        .flatten()
        .and_then(|v| serde_json::from_value(v).ok())
}

pub fn get_problem(st: &AppState, id: String) -> R<ProblemView> {
    let ch = st.chapter_of(&id)?;
    let p = ch.problem(&id).ok_or("no problem")?;
    let store = st.store.lock().unwrap();
    let state = store.problem_state(course_id(st), p).map_err(e)?;
    Ok(views::problem_view(
        &ch.id,
        p,
        state,
        draft(&store, course_id(st), &id),
    ))
}

pub fn save_draft(st: &AppState, id: String, files: Vec<SourceFile>) -> R<()> {
    st.store
        .lock()
        .unwrap()
        .save_draft(course_id(st), &id, &serde_json::to_value(files).map_err(e)?)
        .map_err(e)
}

pub fn open_hint(st: &AppState, id: String, n: u32) -> R<ProblemView> {
    let ch = st.chapter_of(&id)?;
    let p = ch.problem(&id).ok_or("no problem")?;
    let store = st.store.lock().unwrap();
    store.open_hint(course_id(st), p, n).map_err(e)?;
    let state = store.problem_state(course_id(st), p).map_err(e)?;
    Ok(views::problem_view(
        &ch.id,
        p,
        state,
        draft(&store, course_id(st), &id),
    ))
}

#[derive(Serialize, TS)]
#[ts(export)]
pub struct GradeOutcome {
    pub result: rb_grader::GradeResult,
    pub award: Award,
    pub problem: ProblemView,
    /// Set when this was a review attempt that earned XP.
    pub review_xp: i64,
}

pub fn grade(
    st: &AppState,
    id: String,
    files: Vec<SourceFile>,
    submit: bool,
    review_chapter: Option<u32>,
    part: Option<u32>,
    on_event: impl Fn(GradeEvent),
) -> R<GradeOutcome> {
    let ch = st.chapter_of(&id)?;
    let p = ch.problem(&id).ok_or("no problem")?.clone();
    // Only the problem's own files can be submitted.
    let files: Vec<SourceFile> = files
        .into_iter()
        .filter(|f| p.starter.iter().any(|s| s.path == f.path))
        .collect();
    let tc = st
        .toolchain
        .lock()
        .unwrap()
        .clone()
        .ok_or("Rust isn't installed. Open Settings → Rust toolchain.")?;
    st.cancel.store(false, Ordering::Relaxed);
    {
        let store = st.store.lock().unwrap();
        store
            .save_draft(
                course_id(st),
                &id,
                &serde_json::to_value(&files).map_err(e)?,
            )
            .map_err(e)?;
    }
    let edition = st.course.toolchain.edition.clone();
    let work = st.data_dir.join("grade");
    let target = st.data_dir.join("target");
    let cancel = st.cancel.clone();
    let mode = if submit { Mode::Submit } else { Mode::Run };
    let grader = Grader {
        toolchain: &tc,
        edition: &edition,
        work_root: work,
        shared_target: Some(target),
        vendor: st.vendor_dir.clone(),
        cancel: Some(&cancel),
    };
    let result = grader.grade(&p, &files, mode, part, on_event).map_err(e)?;

    let store = st.store.lock().unwrap();
    // A checkpoint part's Accepted counts for the problem only when it's the last part.
    let part_accepted = result.verdict == Verdict::Accepted;
    if let (true, Some(n)) = (part_accepted, result.part) {
        store
            .record_checkpoint(course_id(st), &p.id, n)
            .map_err(e)?;
    }
    let accepted = part_accepted && result.part.is_none_or(|n| n as usize == p.parts.len());
    let failed: Vec<String> = result
        .tests
        .iter()
        .filter(|t| !matches!(t.status, TestStatus::Passed | TestStatus::NotRun))
        .map(|t| t.key.clone())
        .collect();
    let verdict = serde_json::to_value(result.verdict)
        .map_err(e)?
        .as_str()
        .unwrap_or("")
        .to_string();
    let mut award = if result.verdict == Verdict::Cancelled {
        Award::default()
    } else {
        store
            .record_grade(
                course_id(st),
                ch,
                &p,
                submit,
                &verdict,
                accepted,
                &failed,
                &serde_json::to_value(&files).map_err(e)?,
            )
            .map_err(e)?
    };
    let mut review_xp = 0;
    if let (true, Some(chn)) = (accepted, review_chapter) {
        let a = store.record_review(course_id(st), chn, &p).map_err(e)?;
        review_xp = a.xp;
        award.badges.extend(a.badges);
    }
    if accepted && p.level == Level::Hard {
        award.badges.extend(check_test_out(st, &store, ch)?);
    }
    let state = store.problem_state(course_id(st), &p).map_err(e)?;
    let result = views::redact(result, &p, &state.revealed);
    let problem = views::problem_view(&ch.id, &p, state, draft(&store, course_id(st), &id));
    Ok(GradeOutcome {
        result,
        award,
        problem,
        review_xp,
    })
}

pub fn cancel_grade(st: &AppState) {
    st.cancel.store(true, Ordering::Relaxed);
}

// ------------------------------------------------------------------ profile, review, book

#[derive(Serialize, TS)]
#[ts(export)]
pub struct EarnedBadge {
    pub badge: BadgeInfo,
    pub earned_at: Option<i64>,
}

#[derive(Serialize, TS)]
#[ts(export)]
pub struct ProfilePage {
    pub stats: Stats,
    pub badges: Vec<EarnedBadge>,
    pub mastery: BTreeMap<String, f64>,
    pub concepts: Vec<Concept>,
    pub chapters: Vec<ChapterState>,
}

pub fn get_profile(st: &AppState) -> R<ProfilePage> {
    let s = snapshot(st)?;
    let chapters = rules::path(&st.course, &st.chapters, &s);
    let store = st.store.lock().unwrap();
    let earned: BTreeMap<String, i64> = store.badges().map_err(e)?.into_iter().collect();
    let nums: Vec<u32> = st.course.chapters.iter().map(|c| c.num).collect();
    let parts: Vec<String> = st.course.parts.iter().map(|p| p.id.clone()).collect();
    let mut all = badges::all(&nums, &parts);
    // Badges earned but not in the list (future ids) still show.
    for id in earned.keys() {
        if !all.iter().any(|b| &b.id == id) {
            all.push(badges::info(id));
        }
    }
    Ok(ProfilePage {
        badges: all
            .into_iter()
            .map(|b| EarnedBadge {
                earned_at: earned.get(&b.id).copied(),
                badge: b,
            })
            .collect(),
        stats: store.stats().map_err(e)?,
        mastery: store.mastery(course_id(st)).map_err(e)?,
        concepts: st.course.concepts.clone(),
        chapters,
    })
}

#[derive(Serialize, TS)]
#[ts(export)]
pub struct ReviewPick {
    pub problem: ProblemSummary,
    pub chapter: u32,
    /// The weak concept this pick revisits.
    pub concept: String,
    pub mastery: f64,
}

/// 3–5 problems from earlier chapters for the learner's weakest concepts; the chapter's
/// default picks when there's no history.
pub fn get_review(st: &AppState, chapter: u32) -> R<Vec<ReviewPick>> {
    let store = st.store.lock().unwrap();
    let mastery = store.mastery(course_id(st)).map_err(e)?;
    let last = store.last_reviewed(course_id(st)).map_err(e)?;
    let earlier: Vec<&Chapter> = st.chapters.iter().filter(|c| c.num < chapter).collect();
    let summary = |c: &Chapter, id: &str| -> Option<ProblemSummary> {
        c.problem(id).map(|p| ProblemSummary {
            id: p.id.clone(),
            title: p.title.clone(),
            level: p.level,
            xp: p.xp,
            comes_after: p.comes_after.clone(),
            concepts: p.concepts.clone(),
        })
    };
    let mut picks: Vec<ReviewPick> = Vec::new();
    if !mastery.is_empty() {
        let mut concepts: Vec<(&Concept, f64)> = st
            .course
            .concepts
            .iter()
            .filter(|c| c.chapter < chapter && c.tested_by == "code")
            .map(|c| (c, mastery.get(&c.id).copied().unwrap_or(0.0)))
            .collect();
        concepts.sort_by(|a, b| a.1.total_cmp(&b.1));
        for (c, m) in concepts {
            if picks.len() >= 5 {
                break;
            }
            let found = earlier
                .iter()
                .flat_map(|ch| rules::counted(ch).map(move |p| (ch, p)))
                .find(|(_, p)| {
                    p.concepts.first() == Some(&c.id)
                        && Some(&p.id) != last.as_ref()
                        && !picks.iter().any(|x| x.problem.id == p.id)
                });
            if let Some((ch, p)) = found
                && let Some(s) = summary(ch, &p.id)
            {
                picks.push(ReviewPick {
                    problem: s,
                    chapter: ch.num,
                    concept: c.id.clone(),
                    mastery: m,
                });
            }
        }
    }
    if picks.len() < 3
        && let Ok(ch) = st.chapter(&format!("ch{chapter:02}"))
    {
        for id in &ch.review {
            if picks.len() >= 5 {
                break;
            }
            if let Some(c) = earlier.iter().find(|c| c.problem(id).is_some())
                && !picks.iter().any(|x| &x.problem.id == id)
            {
                let s = summary(c, id).unwrap();
                let concept = s.concepts.first().cloned().unwrap_or_default();
                picks.push(ReviewPick {
                    mastery: mastery.get(&concept).copied().unwrap_or(0.0),
                    concept,
                    chapter: c.num,
                    problem: s,
                });
            }
        }
    }
    Ok(picks)
}

/// A bundled Rust Book chapter as Markdown ("Read more").
pub fn read_book(st: &AppState, file: String) -> R<String> {
    if file.contains("..") || file.contains('/') || file.contains('\\') {
        return Err("bad file name".into());
    }
    std::fs::read_to_string(st.book_dir.join(&file))
        .map_err(|_| format!("The book section {file} isn't bundled."))
}

// ------------------------------------------------------------------ rust-analyzer

#[derive(Serialize, TS)]
#[ts(export)]
pub struct LspStarted {
    pub root_uri: String,
    /// Editor file path → document URI.
    pub uris: BTreeMap<String, String>,
}

fn file_uri(p: &std::path::Path) -> String {
    let s = p.to_string_lossy().replace('\\', "/");
    let s: String = s
        .split('/')
        .map(|seg| {
            seg.replace('%', "%25")
                .replace(' ', "%20")
                .replace('#', "%23")
        })
        .collect::<Vec<_>>()
        .join("/");
    if s.starts_with('/') {
        format!("file://{s}")
    } else {
        format!("file:///{s}")
    }
}

pub fn lsp_start(
    st: &AppState,
    id: String,
    files: Vec<SourceFile>,
    on_message: impl Fn(String) + Send + 'static,
) -> R<LspStarted> {
    let tc = st
        .toolchain
        .lock()
        .unwrap()
        .clone()
        .ok_or("Rust isn't installed")?;
    let ra = tc
        .rust_analyzer
        .clone()
        .ok_or("rust-analyzer isn't installed. Run `rustup component add rust-analyzer`.")?;
    let ch = st.chapter_of(&id)?;
    let p = ch.problem(&id).ok_or("no problem")?;
    let root = st.data_dir.join("workspaces").join(&st.course.id).join(&id);
    let cargo = p.mode == rb_content::BuildMode::Cargo;
    let pairs: Vec<(String, String)> = files
        .iter()
        .map(|f| (f.path.clone(), f.text.clone()))
        .collect();
    let paths = rb_lsp::prepare_workspace(&root, &id, &pairs, cargo).map_err(e)?;
    // rust-analyzer runs Cargo itself; point it at the vendored crates so `rand` resolves offline.
    if let Some(v) = &st.vendor_dir {
        std::fs::create_dir_all(root.join(".cargo")).map_err(e)?;
        std::fs::write(
            root.join(".cargo/config.toml"),
            rb_runner::vendor_config_toml(v),
        )
        .map_err(e)?;
    }
    let mut uris = BTreeMap::new();
    for (f, p) in files.iter().zip(paths) {
        uris.insert(f.path.clone(), file_uri(&p));
    }
    let mut lsp = st.lsp.lock().unwrap();
    if let Some(mut old) = lsp.take() {
        old.stop();
    }
    let session =
        rb_lsp::Session::start(&ra, &root, tc.cargo.parent(), on_message, || {}).map_err(e)?;
    *lsp = Some(session);
    Ok(LspStarted {
        root_uri: file_uri(&root),
        uris,
    })
}

pub fn lsp_send(st: &AppState, message: String) -> R<()> {
    match st.lsp.lock().unwrap().as_ref() {
        Some(s) => s.send(&message).map_err(e),
        None => Err("rust-analyzer isn't running".into()),
    }
}

pub fn lsp_stop(st: &AppState) {
    if let Some(mut s) = st.lsp.lock().unwrap().take() {
        s.stop();
    }
}
