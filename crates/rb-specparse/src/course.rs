//! Course-wide files (course-overview.md, concept-map.md) and whole chapters.

use crate::{Report, lessons, md, problems, questions};
use anyhow::{Context, Result};
use rb_content::*;
use std::path::{Path, PathBuf};

/// A chapter row of course-overview §8.
#[derive(Debug, Clone)]
pub struct ChapterInfo {
    pub num: u32,
    pub folder: String,
    pub title: String,
    pub part: String,
}

pub struct Plan {
    pub dir: PathBuf,
    pub chapters: Vec<ChapterInfo>,
    pub parts: Vec<Part>,
    pub concepts: Vec<Concept>,
}

fn read(path: &Path) -> Result<String> {
    std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))
}

impl Plan {
    pub fn open(dir: &Path) -> Result<Plan> {
        let overview = read(&dir.join("00-foundations/course-overview.md"))?;
        let lines: Vec<&str> = overview.lines().collect();

        let mut parts = Vec::new();
        for row in md::table_with_header(&lines, "Part") {
            let (id, title) = row[0].split_once(". ").unwrap_or((&row[0], ""));
            let range: Vec<u32> = row[1]
                .split(['–', '-'])
                .filter_map(|n| n.trim().parse().ok())
                .collect();
            let chapters = match range.as_slice() {
                [a, b] => (*a..=*b).collect(),
                [a] => vec![*a],
                _ => Vec::new(),
            };
            parts.push(Part {
                id: id.to_string(),
                title: title.to_string(),
                chapters,
            });
        }

        let mut chapters = Vec::new();
        for row in md::tables_with_header(&lines, "Ch").into_iter().flatten() {
            let Ok(num) = row[0].parse::<u32>() else {
                continue;
            };
            if row.len() < 3 || !row[1].starts_with('`') {
                continue;
            }
            let folder = md::unbacktick(&row[1]);
            let part = parts
                .iter()
                .find(|p| p.chapters.contains(&num))
                .map(|p| p.id.clone())
                .unwrap_or_default();
            chapters.push(ChapterInfo {
                num,
                folder,
                title: String::new(),
                part,
            });
        }

        let concept_text = read(&dir.join("00-foundations/concept-map.md"))?;
        let concept_lines: Vec<&str> = concept_text.lines().collect();
        let mut concepts = Vec::new();
        for sec in md::sections(&concept_lines, 2) {
            let Some(rest) = sec.heading.strip_prefix("Chapter ") else {
                continue;
            };
            let (num, title) = rest.split_once(':').unwrap_or((rest, ""));
            let Ok(num) = num.trim().parse::<u32>() else {
                continue;
            };
            if let Some(c) = chapters.iter_mut().find(|c| c.num == num) {
                c.title = title.trim().to_string();
            }
            for row in md::table_with_header(&sec.lines, "ID") {
                if row.len() < 5 {
                    continue;
                }
                concepts.push(Concept {
                    id: md::unbacktick(&row[0]),
                    name: row[1].clone(),
                    chapter: num,
                    lesson: row[2].clone(),
                    definition: row[3].clone(),
                    tested_by: row[4].trim().to_string(),
                });
            }
        }
        Ok(Plan {
            dir: dir.to_path_buf(),
            chapters,
            parts,
            concepts,
        })
    }

    pub fn info(&self, num: u32) -> Option<&ChapterInfo> {
        self.chapters.iter().find(|c| c.num == num)
    }

    /// Parses one chapter folder into the model (generated layer only).
    pub fn chapter(&self, num: u32, report: &mut Report) -> Result<Chapter> {
        let info = self
            .info(num)
            .with_context(|| format!("chapter {num} is not in course-overview §8"))?;
        let dir = self.dir.join(&info.folder);
        let readme = read(&dir.join("README.md"))?;
        let lessons = lessons::lessons(&read(&dir.join("lessons.md"))?, report);
        let question_sets =
            questions::question_sets(&read(&dir.join("questions.md"))?, num, report);
        let problems_md = read(&dir.join("problems.md"))?;
        let rust_rows = problems::rust_rows(&problems_md);
        let mut problems = problems::problems(&problems_md, rust_rows.as_ref(), report);
        if let Ok(p) = read(&dir.join("project.md")) {
            problems.extend(problems::projects(&p, num, rust_rows.as_ref(), report));
        }
        let review = match read(&dir.join("review.md")) {
            Ok(r) => review_picks(&r),
            Err(_) => Vec::new(),
        };

        let rl: Vec<&str> = readme.lines().collect();
        let starter_stage = rl
            .iter()
            .find_map(|l| l.split("Starter stage: ").nth(1))
            .map(|s| s.trim().to_string())
            .unwrap_or_default();
        let sections = md::sections(&rl, 2);
        let bullets = |prefix: &str| -> Vec<String> {
            sections
                .iter()
                .find(|s| s.heading.starts_with(prefix))
                .map(|s| {
                    s.lines
                        .iter()
                        .filter_map(|l| l.trim().strip_prefix("- "))
                        .map(str::to_string)
                        .collect()
                })
                .unwrap_or_default()
        };
        let mut book_sections = bullets("Book sections covered");
        if book_sections.is_empty() {
            // Some READMEs use a table instead.
            if let Some(s) = sections
                .iter()
                .find(|s| s.heading.starts_with("Book sections"))
            {
                book_sections = md::table(&s.lines)
                    .into_iter()
                    .map(|r| r[0].clone())
                    .collect();
            }
        }
        let objectives = bullets("Learning objectives");

        let sequence = sections
            .iter()
            .find(|s| s.heading.starts_with("Course sequence"))
            .map(|s| sequence(&s.lines, &lessons, &question_sets, &problems, report))
            .unwrap_or_default();

        Ok(Chapter {
            num,
            id: format!("ch{num:02}"),
            title: info.title.clone(),
            part: info.part.clone(),
            starter_stage,
            book_sections,
            objectives,
            unlock: UnlockRule {
                problem_share: 0.6,
                require_hard: problems.iter().any(|p| p.level == Level::Hard),
            },
            sequence,
            lessons,
            question_sets,
            problems,
            review,
        })
    }
}

fn review_picks(text: &str) -> Vec<String> {
    let lines: Vec<&str> = text.lines().collect();
    md::table_with_header(&lines, "#")
        .into_iter()
        .filter_map(|r| {
            let cell = r.get(1)?;
            let cell = cell.trim_start_matches('[');
            Some(cell.split(" · ").next()?.trim().to_string())
        })
        .collect()
}

fn sequence(
    lines: &[&str],
    lessons: &[Lesson],
    sets: &[QuestionSet],
    problems: &[Problem],
    report: &mut Report,
) -> Vec<PathItem> {
    let mut out = Vec::new();
    for row in md::table_with_header(lines, "#") {
        if row.len() < 3 {
            continue;
        }
        // `[P04-16](problems.md#…)` → `P04-16`
        let cell = row[1].trim();
        let item = match cell.strip_prefix('[').and_then(|r| r.split_once("](")) {
            Some((text, _)) => text,
            None => cell,
        };
        let kind = row[2].to_lowercase();
        let (k, id) = if lessons.iter().any(|l| l.id == item) {
            (PathKind::Lesson, item.to_string())
        } else if let Some(s) = sets.iter().find(|s| s.id == item) {
            (
                if s.practice {
                    PathKind::Practice
                } else {
                    PathKind::Questions
                },
                item.to_string(),
            )
        } else if problems.iter().any(|p| p.id == item) {
            let p = problems.iter().find(|p| p.id == item).unwrap();
            (
                if p.level == Level::Milestone {
                    PathKind::Milestone
                } else {
                    PathKind::Problem
                },
                item.to_string(),
            )
        } else if item == "Practice" {
            match sets.iter().find(|s| s.practice) {
                Some(s) => (PathKind::Practice, s.id.clone()),
                None => continue,
            }
        } else if item == "Review" || kind.contains("review") {
            (PathKind::Review, "review".to_string())
        } else if item == "Project" || kind.contains("chapter project") || kind.trim() == "project"
        {
            match problems.iter().find(|p| p.level == Level::Project) {
                Some(p) => (PathKind::Project, p.id.clone()),
                None => continue,
            }
        } else {
            report.error(format!("course sequence: unknown item `{item}`"));
            continue;
        };
        out.push(PathItem { kind: k, id });
    }
    if out.is_empty() {
        out = short_form(lines, lessons, sets, problems, report);
    } else {
        tail(lines, sets, problems, &mut out);
    }
    out
}

/// The items after the table, from a line like "Each lesson → … → [mixed practice](…) →
/// [review](…) → [chapter project and LM6](…)."
fn tail(lines: &[&str], sets: &[QuestionSet], problems: &[Problem], out: &mut Vec<PathItem>) {
    let Some(line) = lines.iter().find(|l| l.starts_with("Each lesson →")) else {
        return;
    };
    let mut add = |kind: PathKind, id: String| {
        if !out.iter().any(|i| i.id == id) {
            out.push(PathItem { kind, id });
        }
    };
    for step in line.split('→').map(|s| s.to_lowercase()) {
        if step.contains("practice")
            && let Some(s) = sets.iter().find(|s| s.practice)
        {
            add(PathKind::Practice, s.id.clone());
        }
        if step.contains("[review]") {
            add(PathKind::Review, "review".to_string());
        }
        if step.contains("project") {
            for (level, kind) in [
                (Level::Project, PathKind::Project),
                (Level::Milestone, PathKind::Milestone),
            ] {
                if let Some(p) = problems.iter().find(|p| p.level == level) {
                    add(kind, p.id.clone());
                }
            }
        }
    }
}

/// The sequence from a "Short form: `L3.1 → Q3.1 → P03-01–02 → … → Practice → Review →
/// Number utilities → LM3`" line.
fn short_form(
    lines: &[&str],
    lessons: &[Lesson],
    sets: &[QuestionSet],
    problems: &[Problem],
    report: &mut Report,
) -> Vec<PathItem> {
    let mut out = Vec::new();
    let Some(line) = lines.iter().find(|l| l.starts_with("Short form:")) else {
        return out;
    };
    let Some(seq) = md::code_spans(line).into_iter().next() else {
        return out;
    };
    for item in seq.split('→').map(str::trim).filter(|s| !s.is_empty()) {
        let found = |id: &str| -> Option<PathItem> {
            if lessons.iter().any(|l| l.id == id) {
                return Some(PathItem {
                    kind: PathKind::Lesson,
                    id: id.into(),
                });
            }
            if sets.iter().any(|s| s.id == id && !s.practice) {
                return Some(PathItem {
                    kind: PathKind::Questions,
                    id: id.into(),
                });
            }
            let p = problems.iter().find(|p| p.id == id)?;
            let kind = match p.level {
                Level::Milestone => PathKind::Milestone,
                Level::Project => PathKind::Project,
                _ => PathKind::Problem,
            };
            Some(PathItem {
                kind,
                id: id.into(),
            })
        };
        // A range of problems: P03-01–02 or P03-01..02.
        if let Some((first, last)) = item.split_once('–').or_else(|| item.split_once("..")) {
            let (prefix, from) = first.rsplit_once('-').unwrap_or(("", first));
            let (Ok(a), Ok(b)) = (from.parse::<u32>(), last.trim().parse::<u32>()) else {
                report.error(format!("course sequence: can't read range `{item}`"));
                continue;
            };
            for n in a..=b {
                match found(&format!("{prefix}-{n:02}")) {
                    Some(i) => out.push(i),
                    None => report.error(format!("course sequence: unknown item {prefix}-{n:02}")),
                }
            }
            continue;
        }
        if let Some(i) = found(item) {
            out.push(i);
        } else if item == "Practice" {
            match sets.iter().find(|s| s.practice) {
                Some(s) => out.push(PathItem {
                    kind: PathKind::Practice,
                    id: s.id.clone(),
                }),
                None => report.error("course sequence: no practice set".to_string()),
            }
        } else if item == "Review" {
            out.push(PathItem {
                kind: PathKind::Review,
                id: "review".into(),
            });
        } else if let Some(p) = problems
            .iter()
            .find(|p| p.level == Level::Project && p.title == item)
        {
            out.push(PathItem {
                kind: PathKind::Project,
                id: p.id.clone(),
            });
        } else {
            report.error(format!("course sequence: unknown item `{item}`"));
        }
    }
    out
}
