//! Tauri command wrappers. Blocking work (compiling, running tests, Cargo) runs on
//! a blocking thread so the window stays responsive.

use crate::AppState;
use crate::commands::{self as c, *};
use crate::views::ProblemView;
use rb_content::SourceFile;
use rb_grader::GradeEvent;
use rb_progress::rules::ChapterState;
use rb_progress::{Award, Profile};
use rb_quiz::{Checked, Response};
use std::sync::Arc;
use tauri::State;
use tauri::ipc::Channel;

type St<'a> = State<'a, Arc<AppState>>;
type R<T> = Result<T, String>;

async fn blocking<T: Send + 'static>(
    st: &St<'_>,
    f: impl FnOnce(&AppState) -> R<T> + Send + 'static,
) -> R<T> {
    let st: Arc<AppState> = Arc::clone(st);
    tauri::async_runtime::spawn_blocking(move || f(&st))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub fn app_info(st: St) -> R<AppInfo> {
    c::app_info(&st)
}
#[tauri::command]
pub fn check_toolchain(st: St) -> ToolchainStatus {
    c::check_toolchain(&st)
}
#[tauri::command]
pub async fn warm_up(st: St<'_>) -> R<bool> {
    blocking(&st, c::warm_up).await
}
#[tauri::command]
pub fn save_profile(st: St, profile: Profile) -> R<()> {
    c::save_profile(&st, profile)
}
#[tauri::command]
pub fn set_setting(st: St, key: String, value: serde_json::Value) -> R<()> {
    c::set_setting(&st, key, value)
}
#[tauri::command]
pub fn reset_progress(st: St) -> R<()> {
    c::reset_progress(&st)
}
#[tauri::command]
pub fn get_path(st: St) -> R<Vec<ChapterState>> {
    c::get_path(&st)
}
#[tauri::command]
pub fn get_chapter(st: St, id: String) -> R<ChapterView> {
    c::get_chapter(&st, id)
}
#[tauri::command]
pub fn complete_lesson(st: St, lesson: String, tries: u32) -> R<Award> {
    c::complete_lesson(&st, lesson, tries)
}
#[tauri::command]
pub fn check_answer(st: St, question: String, response: Response, record: bool) -> R<Checked> {
    c::check_answer(&st, question, response, record)
}
#[tauri::command]
pub fn complete_set(st: St, set: String, score: f64) -> R<Award> {
    c::complete_set(&st, set, score)
}
#[tauri::command]
pub fn get_problem(st: St, id: String) -> R<ProblemView> {
    c::get_problem(&st, id)
}
#[tauri::command]
pub fn save_draft(st: St, id: String, files: Vec<SourceFile>) -> R<()> {
    c::save_draft(&st, id, files)
}
#[tauri::command]
pub fn open_hint(st: St, id: String, n: u32) -> R<ProblemView> {
    c::open_hint(&st, id, n)
}
#[tauri::command]
pub async fn grade(
    st: St<'_>,
    id: String,
    files: Vec<SourceFile>,
    submit: bool,
    review_chapter: Option<u32>,
    part: Option<u32>,
    on_event: Channel<GradeEvent>,
) -> R<GradeOutcome> {
    blocking(&st, move |s| {
        c::grade(s, id, files, submit, review_chapter, part, |ev| {
            let _ = on_event.send(ev);
        })
    })
    .await
}
#[tauri::command]
pub fn cancel_grade(st: St) {
    c::cancel_grade(&st)
}
#[tauri::command]
pub fn get_profile(st: St) -> R<ProfilePage> {
    c::get_profile(&st)
}
#[tauri::command]
pub fn get_review(st: St, chapter: u32) -> R<Vec<ReviewPick>> {
    c::get_review(&st, chapter)
}
#[tauri::command]
pub fn read_book(st: St, file: String) -> R<String> {
    c::read_book(&st, file)
}
#[tauri::command]
pub fn lsp_start(
    st: St,
    id: String,
    files: Vec<SourceFile>,
    on_message: Channel<String>,
) -> R<LspStarted> {
    c::lsp_start(&st, id, files, move |m| {
        let _ = on_message.send(m);
    })
}
#[tauri::command]
pub fn lsp_send(st: St, message: String) -> R<()> {
    c::lsp_send(&st, message)
}
#[tauri::command]
pub fn lsp_stop(st: St) {
    c::lsp_stop(&st)
}
