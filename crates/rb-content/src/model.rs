//! The course content model. Authored as TOML under `content/`, bundled as JSON,
//! loaded by the app. Every learner-facing string comes from the course plan.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use ts_rs::TS;

// ---------------------------------------------------------------- course

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Course {
    pub id: String,
    pub title: String,
    pub version: String,
    pub toolchain: Toolchain,
    /// Every chapter in course order, bundled or not.
    pub chapters: Vec<ChapterRef>,
    pub concepts: Vec<Concept>,
    /// Course parts (I–V), used by exams and badges.
    #[serde(default)]
    pub parts: Vec<Part>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Toolchain {
    pub edition: String,
    /// Oldest rustc the content is verified against, e.g. "1.85.0".
    pub min_rustc: String,
    /// The version the course text is written for, e.g. "1.98".
    pub target_rustc: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Part {
    pub id: String,
    pub title: String,
    pub chapters: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ChapterRef {
    pub num: u32,
    pub id: String,
    pub title: String,
    pub part: String,
    /// False while the chapter's content isn't bundled yet ("Coming soon").
    pub available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Concept {
    pub id: String,
    pub name: String,
    pub chapter: u32,
    pub lesson: String,
    pub definition: String,
    /// "code" (tested by problems) or "quiz" (tested by questions only).
    pub tested_by: String,
}

// ---------------------------------------------------------------- chapter

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Chapter {
    pub num: u32,
    pub id: String,
    pub title: String,
    pub part: String,
    pub starter_stage: String,
    pub book_sections: Vec<String>,
    pub objectives: Vec<String>,
    pub unlock: UnlockRule,
    /// The learner's path through the chapter, in order.
    pub sequence: Vec<PathItem>,
    pub lessons: Vec<Lesson>,
    pub question_sets: Vec<QuestionSet>,
    pub problems: Vec<Problem>,
    /// Default review picks (problem ids from earlier chapters).
    #[serde(default)]
    pub review: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct UnlockRule {
    /// Share of the chapter's (non-project) problems that must be Accepted.
    pub problem_share: f64,
    pub require_hard: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct PathItem {
    pub kind: PathKind,
    /// Lesson id, question-set id, problem id, or "review".
    pub id: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum PathKind {
    Lesson,
    Questions,
    Problem,
    Practice,
    Review,
    Project,
    Milestone,
}

// ---------------------------------------------------------------- lessons

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Lesson {
    pub id: String,
    pub uid: String,
    pub title: String,
    pub book_section: String,
    pub foundation: bool,
    pub concepts_introduced: Vec<String>,
    pub concepts_reused: Vec<String>,
    pub minutes: u32,
    pub objective_md: String,
    pub screens: Vec<Screen>,
    pub summary: Vec<String>,
    pub completion: Completion,
    /// Path of the matching Rust Book chapter file, e.g. "ch01-02-hello-world.md".
    #[serde(default)]
    pub read_more: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Completion {
    pub xp: u32,
    pub button: String,
    pub first_try: String,
    /// Contains `{tries}`.
    pub many_tries: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum ScreenKind {
    Concept,
    Stepper,
    Error,
    Compare,
    Check,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct CodeBlock {
    /// File label shown in the code header ("src/main.rs", "terminal", "Cargo.toml").
    #[serde(default)]
    pub file: Option<String>,
    /// rust | console | toml | text
    pub lang: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Screen {
    /// Stable key within the lesson ("s1", "qc1"); overlays merge on it.
    pub key: String,
    pub kind: ScreenKind,
    #[serde(default)]
    pub heading: String,
    #[serde(default)]
    pub text_md: String,
    #[serde(default)]
    pub code: Option<CodeBlock>,
    /// A multi-file example (a package): every file, in order, each with its path in
    /// `file`. `code` is then empty.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<CodeBlock>,
    /// Keyed by background: python, js, java, c, go.
    #[serde(default)]
    pub coming_from: BTreeMap<String, String>,
    /// Side or static visual drawn by the generic visualizers.
    #[serde(default)]
    pub panels: Vec<Panel>,
    #[serde(default)]
    pub steps: Vec<Step>,
    /// Error screens: verbatim rustc output and annotations.
    #[serde(default)]
    pub diagnostic: Option<String>,
    #[serde(default)]
    pub annotations: Vec<Annotation>,
    #[serde(default)]
    pub compare: Option<Compare>,
    /// Check screens: the quick check.
    #[serde(default)]
    pub question: Option<Question>,
    /// The spec's prose description of the static visual (authoring reference).
    #[serde(default)]
    pub visual_md: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Step {
    pub n: u32,
    /// In a multi-file screen, the file `hl` and `err` refer to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    /// Highlighted code lines (1-based).
    #[serde(default)]
    pub hl: Vec<u32>,
    /// Lines marked as errors.
    #[serde(default)]
    pub err: Vec<u32>,
    pub caption_md: String,
    /// The spec's prose for this step's visual state (authoring reference).
    #[serde(default)]
    pub visual_md: String,
    /// Changes to apply to the panels at this step, on top of the previous step.
    #[serde(default)]
    pub ops: Vec<PanelOp>,
    /// Badges shown next to code lines at this step only.
    #[serde(default)]
    pub notes: Vec<CodeNote>,
    /// Arrows between spans of the code at this step only.
    #[serde(default)]
    pub links: Vec<CodeLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct CodeNote {
    pub line: u32,
    pub text: String,
}

/// Spans are "LINE:COL-COL" (1-based, inclusive columns).
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct CodeLink {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[ts(export)]
pub enum Panel {
    /// Program output, built up by `print` ops.
    Terminal {
        id: String,
        #[serde(default)]
        title: Option<String>,
        /// Show a blinking cursor where the next character will go.
        #[serde(default)]
        cursor: bool,
        /// Shown from the start, before any step prints (a concept screen's test output).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        text: Option<String>,
    },
    /// Generic boxes-and-arrows diagram.
    Scene {
        id: String,
        layout: SceneLayout,
        #[serde(default)]
        nodes: Vec<SceneNode>,
        #[serde(default)]
        edges: Vec<SceneEdge>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum SceneLayout {
    /// Left to right, joined by the edges.
    Row,
    /// Indented tree, by `parent`.
    Tree,
    /// Wrapping grid.
    Grid,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct SceneNode {
    pub id: String,
    pub label: String,
    #[serde(default)]
    pub sub: Option<String>,
    /// box | file | folder | code
    #[serde(default = "default_node_kind")]
    pub kind: String,
    #[serde(default)]
    pub parent: Option<String>,
    #[serde(default)]
    pub state: NodeState,
}

fn default_node_kind() -> String {
    "box".into()
}

/// Visual state of a scene node. Ops replace the fields they set.
#[derive(Debug, Clone, Default, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct NodeState {
    #[serde(default)]
    pub hidden: bool,
    /// plain | rust | blue | green | red
    #[serde(default)]
    pub style: Option<String>,
    #[serde(default)]
    pub lit: bool,
    #[serde(default)]
    pub ghost: bool,
    #[serde(default)]
    pub badge: Option<String>,
    #[serde(default)]
    pub note: Option<String>,
    /// A value shown in code font under the label (`"42\n"`, `Ok(42)`), e.g. a variable's.
    #[serde(default)]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct SceneEdge {
    pub id: String,
    /// A node id, or a span of the screen's code: "code:LINE:COL-COL" (1-based, inclusive).
    pub from: String,
    pub to: String,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub hidden: bool,
    #[serde(default)]
    pub style: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct PanelOp {
    pub panel: String,
    #[serde(flatten)]
    pub op: Op,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(tag = "op", rename_all = "snake_case")]
#[ts(export)]
pub enum Op {
    /// Terminal: append text (may contain "\n"); the new text flashes.
    Print { text: String },
    /// Terminal: show a grey badge under the output.
    Badge { text: String },
    /// Scene: change a node's state (fields given replace the old ones).
    Node {
        id: String,
        #[serde(default)]
        hidden: Option<bool>,
        #[serde(default)]
        style: Option<String>,
        #[serde(default)]
        lit: Option<bool>,
        #[serde(default)]
        ghost: Option<bool>,
        /// An empty string removes the badge.
        #[serde(default)]
        badge: Option<String>,
        #[serde(default)]
        note: Option<String>,
        /// An empty string removes the value.
        #[serde(default)]
        value: Option<String>,
    },
    /// Scene: show or hide an edge.
    Edge { id: String, hidden: bool },
    /// Any panel: clear all badges and lit states from earlier steps.
    Reset,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Annotation {
    /// Text in the diagnostic that the annotation points at.
    pub mark: String,
    pub note_md: String,
    /// Code line the annotation refers to, if any.
    #[serde(default)]
    pub line: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Compare {
    pub left: ComparePanel,
    pub right: ComparePanel,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ComparePanel {
    pub label: String,
    pub code: CodeBlock,
    pub caption_md: String,
}

// ---------------------------------------------------------------- questions

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct QuestionSet {
    /// "Q1.2" or "Q1.P".
    pub id: String,
    pub uid: String,
    pub title: String,
    pub practice: bool,
    /// Lesson the set follows (follow-up sets only).
    #[serde(default)]
    pub lesson: Option<String>,
    pub xp: u32,
    pub questions: Vec<Question>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum QuestionKind {
    Single,
    Multi,
    TrueFalse,
    Compiles,
    PredictOutput,
    FillBlank,
    Arrange,
    SpotBug,
    MatchPairs,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Question {
    pub id: String,
    pub uid: String,
    pub kind: QuestionKind,
    pub difficulty: u8,
    pub concepts: Vec<String>,
    pub prompt_md: String,
    #[serde(default)]
    pub code: Option<CodeBlock>,
    /// A multi-file program (a package): every file with its path. `code` is then empty,
    /// and a compile error's line is in whichever file the compiler reports.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<CodeBlock>,
    /// Choice types, arrange (shuffled lines) and the right column of match pairs.
    #[serde(default)]
    pub options: Vec<Choice>,
    /// Match pairs: the numbered left column.
    #[serde(default)]
    pub left: Vec<Choice>,
    pub answer: Answer,
    /// Explanation per option key (choice types) or per pair ("1-B").
    #[serde(default)]
    pub explanations: Vec<Explanation>,
    /// Typed answers: why the right answer is right.
    #[serde(default)]
    pub general_md: Option<String>,
    #[serde(default)]
    pub wrong_answers: Vec<WrongAnswer>,
    #[serde(default)]
    pub fix_md: Option<String>,
    /// Compile/output facts, checked against real rustc by course-check.
    #[serde(default)]
    pub verify: Option<Verify>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Choice {
    pub key: String,
    pub text_md: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[ts(export)]
pub enum Answer {
    /// One or more option keys (single, multi, true/false, compiles).
    Keys { keys: Vec<String> },
    /// Exact output; `accepted` lists every accepted output.
    Text { text: String, accepted: Vec<String> },
    /// One list of accepted spellings per blank.
    Blanks { blanks: Vec<Vec<String>> },
    /// One or more correct orders of option keys.
    Orders { orders: Vec<Vec<String>> },
    /// Exactly these lines must be selected.
    Lines { lines: Vec<u32> },
    /// Left key → right key.
    Pairs { pairs: Vec<(String, String)> },
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Explanation {
    pub key: String,
    pub correct: bool,
    pub text_md: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct WrongAnswer {
    pub answer_md: String,
    pub why_md: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Verify {
    pub compiles: bool,
    #[serde(default)]
    pub error_code: Option<String>,
    #[serde(default)]
    pub error_line: Option<u32>,
    #[serde(default)]
    pub error_message: Option<String>,
    #[serde(default)]
    pub stdout: Option<String>,
    /// A package's output: the binary the question runs (`cargo run --bin admin`); the
    /// package's own binary when empty.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bin: Option<String>,
}

// ---------------------------------------------------------------- problems

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum Level {
    #[serde(rename = "Warm-up")]
    WarmUp,
    Easy,
    Medium,
    Hard,
    Project,
    Milestone,
}

impl Level {
    pub fn weight(self) -> f64 {
        match self {
            Level::WarmUp => 0.4,
            Level::Easy => 0.6,
            Level::Medium => 0.8,
            Level::Hard | Level::Project | Level::Milestone => 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum BuildMode {
    /// One file compiled with rustc.
    Rustc,
    /// A Cargo package.
    Cargo,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Problem {
    pub id: String,
    pub uid: String,
    pub title: String,
    pub level: Level,
    pub xp: u32,
    pub comes_after: String,
    /// program | function | multi-file | fix-compile | test-writing
    pub kind: String,
    pub concepts: Vec<String>,
    /// Earlier-chapter concepts, "id (ChN)".
    pub reuses: Vec<String>,
    pub starter_stage: String,
    pub time_limit_ms: u64,
    pub mode: BuildMode,
    /// Cargo: build into the project's own target dir (no shared target).
    #[serde(default)]
    pub isolated_target: bool,
    pub description_md: String,
    pub examples_md: String,
    pub constraints_md: String,
    pub hints: Vec<String>,
    pub solution_approach_md: String,
    #[serde(default)]
    pub idiomatic_md: Option<String>,
    pub random_md: String,
    pub source_checks_md: String,
    pub notes_md: String,
    /// Project parts (Part 1, Part 2, …), with their titles.
    #[serde(default)]
    pub parts: Vec<String>,
    /// Parts are checkpoints graded on their own submissions: each test belongs to one
    /// part, and the problem is Accepted when the last part is.
    #[serde(default)]
    pub checkpoints: bool,
    /// `let` bindings whose initialisers fixture runs replace with a test value.
    #[serde(default)]
    pub fixtures: Vec<String>,
    /// Checker programs (`checkers/<name>.rs`) that judge output a fixed expectation can't.
    #[serde(default)]
    pub checkers: Vec<SourceFile>,
    /// Generator programs (`generators/<name>.rs`) for suites; used by course-check only.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub generators: Vec<SourceFile>,
    /// Rust items (`helpers.rs`) added to the call harness, for calls too long to repeat
    /// in every case: a suite case can be `drive("…ops…")`.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub helpers: String,
    /// Files the learner edits, in tab order, with starter contents.
    pub starter: Vec<SourceFile>,
    /// Reference solution files.
    pub solution: Vec<SourceFile>,
    /// Checkpoint problems: reference solutions for the parts before the last.
    #[serde(default)]
    pub part_solutions: BTreeMap<u32, Vec<SourceFile>>,
    /// Deliberately wrong variants used by course-check.
    #[serde(default)]
    pub wrong: Vec<WrongVariant>,
    /// Test-writing problems: the code under test, supplied and read-only. A learner file
    /// with the same path starts with this text; the learner writes only what follows.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subject: Vec<SourceFile>,
    /// Test-writing problems: versions of the subject with one planted bug each, which
    /// the learner's tests must catch.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mutants: Vec<Mutant>,
    pub tests: Vec<TestCase>,
}

/// A buggy version of a test-writing problem's subject.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Mutant {
    /// "M01"
    pub id: String,
    pub title: String,
    /// What changed, as the spec writes it.
    pub change_md: String,
    /// Replacements for subject files (whole files); other subject files stay as they are.
    pub files: Vec<SourceFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct SourceFile {
    /// Path relative to the package root ("src/main.rs", "Cargo.toml"); "main.rs" for rustc problems.
    pub path: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct WrongVariant {
    pub name: String,
    pub files: Vec<SourceFile>,
    /// Test keys this variant must fail; every other test must pass.
    pub fails: Vec<String>,
    /// Checkpoint problems: the part it's graded as (default: the last).
    #[serde(default)]
    pub part: Option<u32>,
    /// Tests it may fail or pass: their outcome depends on random draws.
    #[serde(default)]
    pub may_fail: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TestCase {
    /// Stable key ("t01"); progress and failure counts are keyed by it.
    pub key: String,
    pub n: u32,
    pub visible: bool,
    pub category: String,
    /// The spec's Input, Expected and Checks cells (shown to the learner when revealed).
    pub input_md: String,
    pub expected_md: String,
    pub checks_md: String,
    /// Concept this row checks (first ID in the Checks cell).
    pub concept: String,
    #[serde(default)]
    pub part: Option<u32>,
    pub run: Run,
    pub asserts: Vec<Assertion>,
}

/// How to produce what a test looks at. Tests with equal runs share one execution.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[ts(export)]
pub enum Run {
    /// Run the program (Cargo: `cargo run`-equivalent of the built binary).
    Process {
        #[serde(default)]
        stdin: String,
        /// Appended to `stdin`: large inputs written as repeated pieces.
        #[serde(default)]
        stdin_pieces: Vec<Piece>,
        #[serde(default)]
        args: Vec<String>,
        /// Fixture values by binding name; the program runs with those initialisers replaced.
        #[serde(default)]
        fixture: BTreeMap<String, String>,
        /// Fresh processes to run; every one must pass the assertions.
        #[serde(default = "one")]
        times: u32,
    },
    /// Many generated inputs, each a fresh process. `generator` describes them; `cases` holds
    /// them with the reference solution's output, filled in by course-check.
    Suite {
        #[serde(default)]
        #[ts(type = "unknown")]
        generator: Option<Gen>,
        /// A generator program (`generators/<name>.rs`) instead of `generator`; it prints
        /// the cases (see course-check's suites).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        script: Option<String>,
        #[serde(default)]
        cases: Vec<Case>,
    },
    /// Call the learner's public function with these arguments (a Rust call expression,
    /// `gcd(48, 18)`), in a harness program that prints the returned value's `{:?}`.
    Call {
        call: String,
        /// A Rust expression for the expected value, made by the harness too, so both
        /// print with `{:?}` (used when the expected value is text or built up).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        expect: Option<String>,
        /// The expected value is exact Rust of the call's type: its untyped literals
        /// (`4294967295`) take that type instead of defaulting to `i32`.
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        typed: bool,
    },
    /// A test program: the learner's file as a module, plus this `main`. If it
    /// compiles, it runs with empty stdin. In a Cargo package with a library, `main`
    /// is another crate that uses the library by its crate name (`module_lab::…`), so
    /// privacy applies as it does to any outside caller; `remove` deletes package files
    /// first, and an empty `main` builds only the library.
    Fixture {
        main: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        remove: Vec<String>,
    },
    /// Test-writing problems: `cargo test` with these arguments (`-- --include-ignored`,
    /// `--test contract`, `shelf`) on the subject, or on a mutant of it, with the
    /// learner's tests added. Each test binary runs on its own, with a time limit.
    Tests {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        mutant: Option<String>,
        #[serde(default)]
        command: String,
    },
    /// Run a Cargo command and look at its result and the files it leaves.
    Build { command: String },
    /// Look at the learner's source files only.
    Source,
    /// Look at the learner's Cargo.toml only.
    Manifest,
    /// Not yet converted from the spec; blocks bundling.
    Todo,
}

fn one() -> u32 {
    1
}

impl Run {
    /// One run of the program with this stdin.
    pub fn process(stdin: String) -> Run {
        Run::Process {
            stdin,
            stdin_pieces: Vec::new(),
            args: Vec::new(),
            fixture: BTreeMap::new(),
            times: 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Piece {
    pub text: String,
    #[serde(default = "one")]
    pub times: u32,
    /// A numbered sequence instead of repetition: `text` once for each n from `from` to
    /// `to` (counting down when `from` > `to`), with `{n}` replaced by n (`{n:03}`:
    /// zero-padded to 3 digits).
    #[serde(default)]
    pub from: Option<u32>,
    #[serde(default)]
    pub to: Option<u32>,
}

/// Expands pieces into text.
pub fn expand(pieces: &[Piece]) -> String {
    let mut out = String::new();
    for p in pieces {
        match (p.from, p.to) {
            (Some(a), Some(b)) => {
                let ns: Box<dyn Iterator<Item = u32>> = if a <= b {
                    Box::new(a..=b)
                } else {
                    Box::new((b..=a).rev())
                };
                for n in ns {
                    out.push_str(&numbered(&p.text, n));
                }
            }
            _ => out.push_str(&p.text.repeat(p.times as usize)),
        }
    }
    out
}

/// `text` with `{n}` (or a zero-padded `{n:0W}`) replaced by n.
fn numbered(text: &str, n: u32) -> String {
    let mut out = text.replace("{n}", &n.to_string());
    while let Some(i) = out.find("{n:0") {
        let Some(len) = out[i..].find('}') else { break };
        let width: usize = out[i + 4..i + len].parse().unwrap_or(0);
        out.replace_range(i..=i + len, &format!("{n:0width$}"));
    }
    out
}

/// One generated input, with the reference output when the suite compares against it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Case {
    #[serde(default)]
    pub stdin: String,
    /// Function suites: the call expression, run in the harness instead of the program.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub call: Option<String>,
    #[serde(default)]
    pub fixture: BTreeMap<String, String>,
    #[serde(default)]
    pub stdout: Option<String>,
    /// Program suites: the reference ended with `main` returning `Err`, so the case
    /// expects this exit code and exactly this stderr.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stderr: Option<String>,
}

/// A seeded input generator (the course's LCG: state = 1664525 × state + 1013904223 mod 2³²;
/// pick(n) = next() mod n). Per case: the fixture draw (if any), a line count with
/// pick(`lines`), each line from `items` with pick(len), then pick(2) for a final newline.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Gen {
    pub seed: u32,
    pub cases: u32,
    pub lines: u32,
    pub items: Vec<GenItem>,
    /// Draw a fixture value first: `binding` = `add` + pick(`pick`).
    #[serde(default)]
    pub fixture: Option<GenFixture>,
    /// Compare each case with the reference solution's stdout (else only the assertions).
    #[serde(default)]
    pub reference: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenItem {
    Text(String),
    /// pick(n) as an unsigned decimal.
    Number {
        number: u32,
    },
    /// One of these, chosen with pick(len).
    OneOf {
        one_of: Vec<String>,
    },
    /// The case's fixture value as a decimal.
    Fixture {
        fixture: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GenFixture {
    pub binding: String,
    pub pick: u32,
    #[serde(default)]
    pub add: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(tag = "check", rename_all = "snake_case")]
#[ts(export)]
pub enum Assertion {
    // ---- output
    StdoutEq {
        text: String,
    },
    LineEq {
        line: u32,
        text: String,
    },
    LineCount {
        count: u32,
    },
    EndsWith {
        text: String,
    },
    Contains {
        text: String,
        #[serde(default)]
        line: Option<u32>,
    },
    NotContains {
        text: String,
        #[serde(default)]
        line: Option<u32>,
    },
    NotStartsWith {
        line: u32,
        any: Vec<String>,
    },
    /// Line has exactly `len` characters, each one of `chars` (when given).
    LineChars {
        line: u32,
        len: u32,
        #[serde(default)]
        all: Option<String>,
    },
    NoTrailingWhitespace,
    /// The whole output matches this regular expression (anchor it with `^…$`).
    StdoutMatches {
        pattern: String,
    },
    /// The whole output equals these pieces, expanded.
    StdoutPieces {
        pieces: Vec<Piece>,
    },
    /// Suite cases: the output equals the reference solution's.
    SameAsReference,
    /// A checker program judges the output; it gets the stdin and stdout file paths.
    Checker {
        name: String,
    },
    /// Call runs: the function returned this value (its `{:?}` text) and printed nothing.
    /// Without `value`, a suite case's reference value. With `tol`, numbers with a
    /// fraction may differ by max(tol, 1e-12 × |expected|).
    Returns {
        #[serde(default)]
        value: Option<String>,
        #[serde(default)]
        tol: Option<f64>,
    },
    /// Fixture runs: the program must not compile, failing with this error code (`E0382`).
    CompileError {
        code: String,
        /// Shown when it compiles anyway.
        #[serde(default)]
        message: Option<String>,
    },
    StderrEmpty,
    /// stderr contains this text (panic messages).
    StderrContains {
        text: String,
    },
    ExitCode {
        code: i32,
    },
    // ---- source (parsed with syn; comments ignored)
    /// Count of calls to a macro (`print`, `println`) in the learner's files.
    MacroCount {
        name: String,
        #[serde(default)]
        eq: Option<u32>,
        #[serde(default)]
        ge: Option<u32>,
    },
    /// The exact call `first` is immediately followed by `then` (format strings compared).
    MacroSequence {
        first: MacroCall,
        then: MacroCall,
        #[serde(default)]
        exactly_once: bool,
    },
    /// No string literal passed to these macros contains any of `substrings` (or a digit).
    FmtLiteralForbid {
        macros: Vec<String>,
        #[serde(default)]
        substrings: Vec<String>,
        #[serde(default)]
        digits: bool,
    },
    /// Each name is used as an inline placeholder (`{name}`) in some format string of these macros.
    FmtInlinePlaceholders {
        macros: Vec<String>,
        names: Vec<String>,
    },
    /// These `let` lines are present, unchanged and in this order, never reassigned or shadowed.
    LetsUnchanged {
        lets: Vec<String>,
    },
    BannedKeyword {
        word: String,
    },
    /// The keyword is used (`loop`, `match`).
    RequiredKeyword {
        word: String,
    },
    /// Calls to a method (`read_line`) or path function (`String::new`): exact or minimum count.
    CallCount {
        name: String,
        #[serde(default)]
        eq: Option<u32>,
        #[serde(default)]
        ge: Option<u32>,
    },
    /// A method chain such as `.read_line(…).expect("…")`, with the last call's string argument.
    CallChain {
        calls: Vec<String>,
        #[serde(default)]
        arg: Option<String>,
    },
    /// Exactly one `let name: ty = expr;`, with `expr` one of the accepted forms (tokens
    /// compared; parentheses around it ignored). `in_loop` says where it must be;
    /// `after` names a binding it must come after.
    LetInit {
        name: String,
        #[serde(default)]
        ty: Option<String>,
        exprs: Vec<String>,
        #[serde(default)]
        in_loop: Option<bool>,
        #[serde(default)]
        after: Option<String>,
        /// The binding may also be declared before this one (shadowing input text).
        #[serde(default)]
        shadows: bool,
    },
    /// A `match` whose scrutinee calls this method (`cmp`).
    MatchOn {
        method: String,
    },
    BannedMethod {
        name: String,
    },
    /// No arithmetic operators (`+ - * / %`, also compound assignment and negation).
    BannedArithmetic,
    /// No call of these macros mentions the binding (as a placeholder or argument).
    MacroMentionForbid {
        macros: Vec<String>,
        name: String,
    },
    /// A call to this function or method happens inside a `loop`/`while`/`for` body.
    CallInLoop {
        name: String,
    },
    /// Every construct in `what` appears (see rb_srccheck's facts: `while`, `op:%`,
    /// `cast:u64`, `call:is_prime`, …), inside function `within` and inside an
    /// enclosing `inside` construct when given.
    Uses {
        what: Vec<String>,
        #[serde(default)]
        within: Option<String>,
        #[serde(default)]
        inside: Option<String>,
        /// Only this file of a package (`src/lib.rs`).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        file: Option<String>,
        /// Shown when the rule is broken.
        #[serde(default)]
        message: Option<String>,
    },
    /// None of these constructs appear (in function `within` when given).
    Forbids {
        what: Vec<String>,
        #[serde(default)]
        within: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        file: Option<String>,
        #[serde(default)]
        message: Option<String>,
    },
    /// A construct (see `Uses`) appears exactly `eq` times.
    Count {
        what: String,
        eq: u32,
        #[serde(default)]
        within: Option<String>,
        #[serde(default)]
        inside: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        file: Option<String>,
        #[serde(default)]
        message: Option<String>,
    },
    /// The package's Rust source files are exactly these.
    RustFiles {
        files: Vec<String>,
    },
    // ---- project
    ManifestEq {
        key: String,
        value: String,
    },
    ManifestTableEmpty {
        table: String,
    },
    ManifestForbidKeys {
        keys: Vec<String>,
    },
    /// The learner's editable files are exactly these.
    FilesAllowlist {
        files: Vec<String>,
    },
    BuildOk,
    /// Path relative to the package; `{name}` = package name, `{exe}` = ".exe" on Windows.
    FileExists {
        path: String,
    },
    /// Tests runs: the tests compiled and every test that ran passed.
    TestsPass,
    /// Tests runs: how many tests ran (passed or failed, not ignored or filtered out).
    TestsRan {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        min: Option<u32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        eq: Option<u32>,
    },
    /// Tests runs: how many tests were listed as ignored.
    TestsIgnored { eq: u32 },
    /// Tests runs: how many tests the filter left out.
    TestsFiltered { eq: u32 },
    /// Tests runs: the test with this name (its full path, or its last segments, such as
    /// `tests::slow`) passed, failed, was ignored or didn't run (`absent`).
    TestOutcome { name: String, outcome: String },
    /// Mutant runs: the tests compiled against the mutant, and at least one failed or ran
    /// out of time. A compile error doesn't count as catching the bug.
    MutantCaught,
    /// Placeholder for a row the importer couldn't convert. Blocks bundling.
    Todo {
        note: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct MacroCall {
    pub name: String,
    /// The format string literal, decoded.
    pub fmt: String,
}

/// Test-writing problems: a mutant's files from its "What changed" cell, one or more
/// `` `before` → `after` `` replacements separated by `;`, in the subject file named first
/// (`` `src/fees.rs`: … ``; the first subject file by default). Each `before` must occur
/// exactly once (after the earlier replacements). A cell without `→` gives no files: the
/// mutant is authored as whole files.
pub fn apply_change(subject: &[SourceFile], change_md: &str) -> Result<Vec<SourceFile>, String> {
    let arrows = change_md.matches('→').count();
    if arrows == 0 {
        return Ok(Vec::new());
    }
    let mut spans = code_spans(change_md);
    let path = match spans.first() {
        Some(p)
            if p.ends_with(".rs") && change_md.trim_start().starts_with(&format!("`{p}`:")) =>
        {
            spans.remove(0)
        }
        _ => subject.first().map(|f| f.path.clone()).unwrap_or_default(),
    };
    if spans.len() != arrows * 2 {
        return Err(format!("{arrows} `→` but {} code spans", spans.len()));
    }
    let base = subject
        .iter()
        .find(|f| f.path == path)
        .ok_or_else(|| format!("no provided file `{path}`"))?;
    let mut text = base.text.clone();
    for pair in spans.chunks(2) {
        let (before, after) = (&pair[0], &pair[1]);
        let found = text.matches(before.as_str()).count();
        if found != 1 {
            return Err(format!(
                "`{before}` occurs {found} times in `{path}` (exactly once needed)"
            ));
        }
        text = text.replacen(before.as_str(), after, 1);
    }
    Ok(vec![SourceFile { path, text }])
}

/// The contents of the `code spans` in a Markdown line, trimmed (`` `a \| b` `` keeps the bar).
fn code_spans(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = s;
    while let Some(i) = rest.find('`') {
        let run = rest[i..].chars().take_while(|c| *c == '`').count();
        let after = &rest[i + run..];
        let fence = "`".repeat(run);
        let mut search = 0;
        let mut end = None;
        while let Some(j) = after[search..].find(&fence) {
            let at = search + j;
            let next = after[at + run..].chars().next();
            if next != Some('`') {
                end = Some(at);
                break;
            }
            search = at + run + after[at + run..].chars().take_while(|c| *c == '`').count();
        }
        let Some(end) = end else { break };
        out.push(after[..end].trim().to_string());
        rest = &after[end + run..];
    }
    out
}

/// Test-writing problems: the learner's files from their parts. A part for a subject file
/// goes below the subject text, after a blank line; other parts are files of their own.
/// Subject files without a part are the subject text alone.
pub fn with_subject(subject: &[SourceFile], parts: Vec<SourceFile>) -> Vec<SourceFile> {
    if subject.is_empty() {
        return parts;
    }
    let mut out: Vec<SourceFile> = subject
        .iter()
        .map(|s| SourceFile {
            path: s.path.clone(),
            text: match parts.iter().find(|p| p.path == s.path) {
                Some(p) => format!("{}\n{}", s.text, p.text),
                None => s.text.clone(),
            },
        })
        .collect();
    out.extend(
        parts
            .into_iter()
            .filter(|p| !subject.iter().any(|s| s.path == p.path)),
    );
    out
}

impl Problem {
    pub fn file(&self, path: &str) -> Option<&SourceFile> {
        self.starter.iter().find(|f| f.path == path)
    }

    /// Test-writing problems: the learner's part of each file, with the supplied subject
    /// text taken off the front. Fails, naming the file, when a subject file is missing or
    /// its supplied part was changed.
    pub fn learner_parts(&self, files: &[SourceFile]) -> Result<Vec<SourceFile>, String> {
        let mut out = Vec::new();
        for s in &self.subject {
            let Some(f) = files.iter().find(|f| f.path == s.path) else {
                return Err(format!("`{}` is missing; it holds the code under test.", s.path));
            };
            let Some(rest) = f.text.strip_prefix(s.text.as_str()) else {
                return Err(format!(
                    "The supplied code at the top of `{}` was changed. It is read-only: write your tests below it.",
                    s.path
                ));
            };
            // A supplied file without a test region (`Cargo.toml`) is read-only as a whole.
            let has_region = self
                .starter
                .iter()
                .any(|f| f.path == s.path && f.text.len() > s.text.len());
            if !has_region && !rest.trim().is_empty() {
                return Err(format!(
                    "`{}` is supplied and read-only; put your tests in the other files.",
                    s.path
                ));
            }
            out.push(SourceFile {
                path: s.path.clone(),
                text: rest.to_string(),
            });
        }
        for f in files {
            if !self.subject.iter().any(|s| s.path == f.path) {
                out.push(f.clone());
            }
        }
        Ok(out)
    }

    /// Test-writing problems: the package to test, with `mutant`'s files (or the subject)
    /// followed by the learner's parts.
    pub fn assemble(&self, parts: &[SourceFile], mutant: Option<&str>) -> Vec<SourceFile> {
        let replaced = mutant.and_then(|id| self.mutants.iter().find(|m| m.id == id));
        let mut out = Vec::new();
        for s in &self.subject {
            let base = replaced
                .and_then(|m| m.files.iter().find(|f| f.path == s.path))
                .unwrap_or(s);
            let tail = parts
                .iter()
                .find(|p| p.path == s.path)
                .map_or("", |p| p.text.as_str());
            out.push(SourceFile {
                path: s.path.clone(),
                text: format!("{}{tail}", base.text),
            });
        }
        for p in parts {
            if !self.subject.iter().any(|s| s.path == p.path) {
                out.push(p.clone());
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbered_pieces() {
        let p = |text: &str, from, to| Piece {
            text: text.into(),
            times: 1,
            from: Some(from),
            to: Some(to),
        };
        assert_eq!(expand(&[p("{n}\n", 1, 3)]), "1\n2\n3\n");
        assert_eq!(expand(&[p("{n:03}\n", 101, 99)]), "101\n100\n099\n");
        assert_eq!(expand(&[p("{n:03}-{n}|", 7, 7)]), "007-7|");
    }
}
