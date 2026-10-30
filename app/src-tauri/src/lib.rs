//! Rusticoder's backend: a thin layer of Tauri commands over the engine crates.

pub mod commands;
mod tauri_cmds;
pub mod views;

use rb_content::{Bundle, Chapter, Course};
use rb_progress::Store;
use rb_runner::Toolchain;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use tauri::Manager;

pub struct AppState {
    pub course: Course,
    pub chapters: Vec<Chapter>,
    /// The bundled Rust Book chapters (for "Read more").
    pub book_dir: PathBuf,
    /// Vendored crates for Cargo problems (offline `rand`), if the course has any.
    pub vendor_dir: Option<PathBuf>,
    pub data_dir: PathBuf,
    pub store: Mutex<Store>,
    pub toolchain: Mutex<Option<Toolchain>>,
    pub cancel: Arc<AtomicBool>,
    pub lsp: Mutex<Option<rb_lsp::Session>>,
}

impl AppState {
    /// Loads the bundled course from `resources` and opens the learner's data in `data_dir`.
    pub fn load(resources: &std::path::Path, data_dir: PathBuf) -> anyhow::Result<AppState> {
        let content = resources.join("content/rust-basics");
        let course = Bundle::course(&content)?;
        let chapters = course
            .chapters
            .iter()
            .filter(|c| c.available)
            .map(|c| Bundle::chapter(&content, &c.id))
            .collect::<anyhow::Result<Vec<_>>>()?;
        std::fs::create_dir_all(&data_dir)?;
        let store = Store::open(&data_dir.join("progress.db"))?;
        Ok(AppState {
            course,
            chapters,
            book_dir: resources.join("book"),
            vendor_dir: Some(content.join("vendor")).filter(|v| v.is_dir()),
            data_dir,
            store: Mutex::new(store),
            toolchain: Mutex::new(Toolchain::detect().ok()),
            cancel: Arc::new(AtomicBool::new(false)),
            lsp: Mutex::new(None),
        })
    }

    pub fn chapter(&self, id: &str) -> Result<&Chapter, String> {
        self.chapters
            .iter()
            .find(|c| c.id == id)
            .ok_or_else(|| format!("no chapter {id}"))
    }

    /// The chapter that contains a problem, lesson or question set.
    pub fn chapter_of(&self, item: &str) -> Result<&Chapter, String> {
        self.chapters
            .iter()
            .find(|c| {
                c.problem(item).is_some()
                    || c.lesson(item).is_some()
                    || c.question_set(item).is_some()
            })
            .ok_or_else(|| format!("no item {item}"))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            let resources = app.path().resource_dir()?.join("resources");
            let state = AppState::load(&resources, app.path().app_data_dir()?)?;
            app.manage(Arc::new(state));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            tauri_cmds::app_info,
            tauri_cmds::check_toolchain,
            tauri_cmds::warm_up,
            tauri_cmds::save_profile,
            tauri_cmds::set_setting,
            tauri_cmds::reset_progress,
            tauri_cmds::get_path,
            tauri_cmds::get_chapter,
            tauri_cmds::complete_lesson,
            tauri_cmds::check_answer,
            tauri_cmds::complete_set,
            tauri_cmds::get_problem,
            tauri_cmds::save_draft,
            tauri_cmds::open_hint,
            tauri_cmds::grade,
            tauri_cmds::cancel_grade,
            tauri_cmds::get_profile,
            tauri_cmds::get_review,
            tauri_cmds::read_book,
            tauri_cmds::lsp_start,
            tauri_cmds::lsp_send,
            tauri_cmds::lsp_stop,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Rusticoder");
}
