//! Rusticoder course content: the model, bundle loading and overlay merging.

pub mod merge;
pub mod model;

pub use model::*;

use anyhow::{Context, Result};
use std::path::Path;

/// A bundled course on disk: `course.json` plus one `chNN.json` per available chapter.
pub struct Bundle;

impl Bundle {
    pub fn course(dir: &Path) -> Result<Course> {
        let path = dir.join("course.json");
        let text = std::fs::read_to_string(&path)
            .with_context(|| format!("reading {}", path.display()))?;
        serde_json::from_str(&text).with_context(|| format!("parsing {}", path.display()))
    }

    pub fn chapter(dir: &Path, id: &str) -> Result<Chapter> {
        let path = dir.join(format!("{id}.json"));
        let text = std::fs::read_to_string(&path)
            .with_context(|| format!("reading {}", path.display()))?;
        serde_json::from_str(&text).with_context(|| format!("parsing {}", path.display()))
    }
}

impl Chapter {
    pub fn lesson(&self, id: &str) -> Option<&Lesson> {
        self.lessons.iter().find(|l| l.id == id)
    }

    pub fn question_set(&self, id: &str) -> Option<&QuestionSet> {
        self.question_sets.iter().find(|q| q.id == id)
    }

    pub fn problem(&self, id: &str) -> Option<&Problem> {
        self.problems.iter().find(|p| p.id == id)
    }
}
