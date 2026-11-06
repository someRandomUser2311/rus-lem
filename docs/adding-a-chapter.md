# Adding a chapter

Adding a chapter is a content change. The app code doesn't change unless the chapter is the first to need a new kind of test or visual (listed at the end).

## 1. Fix the spec

The chapter's Markdown in `../rust-basics-course-plan/chNN-*/` is the source. It must follow the formats in `00-foundations/` (problem-spec-format.md, question-format.md, lesson-spec-format.md). Fix the spec first; never patch the generated files.

## 2. Import

```bash
cargo run -p course-check -- import NN
```

This writes `content/rust-basics/chNN/chapter.gen.json` and refreshes `course.gen.json` (concepts, chapter list). It prints counts, parse errors and how many test rows still need authoring. Re-running it is safe: the authored overlay is separate.

The importer converts what it can read reliably:

| Spec | Becomes |
|---|---|
| Lesson fields, screen text, code, captions, Coming-from cards, error output and annotations, compare panels, summary, completion | Lesson screens (visual panels are authored, below) |
| Every question, option, answer, accepted answer and explanation | Questions, with `verify` facts for compile/output questions |
| Problem fields, description, examples, constraints, hints, approach, starter files | Problem |
| Command-line rows `` `args: ["-n", "2", "poem.txt"]` `file poem.txt: "…"` `env IGNORE_CASE: "1"` `` → `stdout: "…"`, `stderr: "…"`, `stderr starts with "…"`, `exit code N` (joined with `;`); `` `cargo test` `` rows in any problem run the learner's own tests | Process runs with files and environment, and test runs |
| Test rows `stdin: "…"` → `stdout: "…"`, `stdout line N is "…"`, `stderr is empty; exit code 0`, `the same stdout as test N`, and an expected failure `stdout: "…"; stderr contains "…"; exit code N` | Runs and assertions |
| Function rows `crate::f(…)` → a Rust value, or `panics with "…"` (the call must panic with that text) | Calls and assertions |
| Any other row | A `todo` assertion that blocks bundling |

## 3. Author the overlay

Overlay files are merged onto the generated layer: objects field by field, lists of items with a `key`, `id` or `n` item by item.

**Problems:** `content/rust-basics/chNN/problems/<ID>/`

- `solution/<files>` is the reference solution, laid out like the starter (`main.rs`, or `Cargo.toml` + `src/main.rs`).
- `overlay.toml` holds the assertions for rows the importer left as `todo`, by test key, plus any problem-level fields:

```toml
isolated_target = true          # Cargo problems that must build into their own target/

[[tests]]
key = "t04"
asserts = [
  { check = "line_count", count = 2 },
  { check = "ends_with", text = "\n" },
]

[[tests]]
key = "t08"
run = { kind = "manifest" }      # change how the row is observed
asserts = [{ check = "manifest_forbid_keys", keys = ["bin", "workspace"] }]

[[wrong]]                        # a deliberately wrong program and exactly the tests it must fail
name = "typed-values"
fails = ["t09"]
```

- `wrong/<name>/<files>` are the wrong variants. When a variant's failures depend on random draws, list those tests in `may_fail`; `fails` must still fail every time. Give every rule test (category `constraint`) and every tricky row at least one wrong variant, so the checks are proven to catch real mistakes.

**Run kinds**

| Kind | What it does |
|---|---|
| `process` | Runs the program. `args`; `files` (`{ path, text }` or `{ path, bytes }`) and `env` run it in a fresh, empty working folder holding only those files, with only those variables set besides `PATH`. `stdin`, plus `stdin_pieces` for large inputs (`{ text, times }`, or `{ text = "{n}\n", from, to }` for numbered lines); `times = 100` runs 100 fresh processes, all of which must pass; `fixture = { secret_number = "42" }` runs a build whose named `let` takes that value. |
| `suite` | Many inputs, each a fresh process. Either `cases = [{ stdin, fixture, stdout }]` written out, or a seeded `generator` (the course's LCG: `seed`, `cases`, `lines`, `items`, optional `fixture` draw); with `reference = true` course-check fills in each case's expected output by running the reference solution. When the reference's `main` returns `Err` on a program case (exit code 1), that case expects the same exit code and stderr. |
| `build` | `command = "build" \| "check"`. |
| `source`, `manifest` | Look at the code or `Cargo.toml` only. |
| `fixture` | A test program (`main`) built against the learner's code. In a Cargo package it's an outside crate that uses the library by its crate name (`module_lab::text::normalize("x")`), so privacy applies; with `compile_error` it's a compiler probe (`E0603`). `remove = ["src/fees/rate.rs"]` deletes files from a copy first; an empty `main` just builds the library. |

Unless a test states otherwise (`exit_code`, `stderr_contains`, `stderr_empty`), every execution must exit 0 with nothing on stderr.

**Problem fields an overlay can set**

- `mode = "cargo"`: a Cargo problem. The spec's `main.rs` starter becomes `src/main.rs`, next to the problem's own `problems/<ID>/Cargo.toml` or else the chapter's `chNN/Cargo.toml`. Solutions and wrong variants may leave out files the starter supplies.
- Multi-file Cargo problems (kind `multi-file`, from Chapter 7): the spec labels each fence with a first-line `// src/text.rs`. Learners can create and delete their own `src/…/*.rs` files. Calls run in a harness whose crate root is the learner's `src/lib.rs` itself, so rows write `crate::measure::square(3)` (or the package's crate name). A wrong variant is the whole package.
- `fixtures = ["secret_number"]`: the bindings fixture runs replace (each must be declared exactly once, with a type).
- `checkpoints = true` and `parts = [...]`: each part is graded on its own submission (tests carry `part = n`); the problem is Accepted with the last part. Reference solutions of earlier parts go in `part-<n>/`.
- `checkers/<name>.rs`: a checker program for output a fixed expectation can't judge (random results). It gets the stdin and stdout file paths and exits 0 to pass, otherwise prints why.

**Test-writing problems** (kind `test-writing`, from Chapter 11): the learner writes the tests.

- The spec's `**Provided code**` fences are the code under test (`subject`), supplied and read-only: the editor locks it, and the learner's tests go below it in the same file or in `tests/*.rs`. `Cargo.toml` is locked too. A milestone whose provided code is an earlier package gives it as `problems/<ID>/subject/` instead.
- Starter, reference solution, `part-<n>/` and `wrong/<name>/` files hold **only the learner's tests**; course-check puts each below the provided code of the same path.
- `**Mutants**` table: `| M01 | title | `` `src/fees.rs`: `before` → `after` `` | a test that catches it |`. Each `before` must occur exactly once; several replacements are separated by `;`. A change too big for a cell goes in `mutants/<ID>/<files>` as whole files.
- Test rows: `` `cargo test -- --include-ignored` `` on the provided code → `Every test passes; At least 3 tests run` (also `N tests run`, `N tests ignored`, `N tests filtered out`, `` `name` is ignored ``/`passes`/`fails`/`doesn't run`); a mutant ID → `A test fails` (the tests compile against it and one fails or times out; a compile error doesn't count). Rule rows look at the learner's tests only (facts such as `attr:test`, `stray_test`, `should_panic_expected`, `should_panic_bare`, `assert_msg`, `assert_bare`, `cfg_test`). Checkpoint rows start with `Part N:`.
- Weaker suites go in `wrong/`: each names the mutant rows it lets through.

**Assertions**

| Group | Checks |
|---|---|
| Output | `stdout_eq`, `stdout_pieces`, `stdout_matches` (regex), `same_as_reference` (suites), `checker`, `line_eq`, `line_count`, `line_chars`, `ends_with`, `contains`, `not_contains`, `not_starts_with`, `no_trailing_whitespace`, `stderr_empty`, `stderr_contains`, `stderr_eq`, `stderr_starts_with`, `exit_code` |
| Source (by fact) | `uses`, `forbids` and `count` (exactly n) over the facts in `crates/rb-srccheck/src/facts.rs`: modules (`mod:NAME`, `mod_file:NAME`), visibility (`pub:NAME`, `pub_field:NAME`), imports (`use:crate::a::b`, `pub_use:…`, `use_glob`, `use_group`), paths (`call_path:super::f`, `path:super::X`), literals (`lit:25`), traits and bounds (`trait:NAME`, `default_fn:NAME`, `bound:Clone`, `where`, `ret:implGreet+use<>`) and more; `file = "src/fees.rs"` limits a check to one file and `a\|b` accepts either fact. `rust_files` requires an exact set of Rust files. |
| Source | `macro_count`, `macro_sequence`, `fmt_literal_forbid`, `fmt_inline_placeholders`, `macro_mention_forbid`, `lets_unchanged`, `let_init`, `call_count`, `call_chain`, `call_in_loop`, `match_on`, `banned_keyword`, `required_keyword`, `banned_method`, `banned_arithmetic` |
| Project | `manifest_eq`, `manifest_table_empty`, `manifest_forbid_keys`, `files_allowlist`, `build_ok`, `file_exists` |

The model in `crates/rb-content/src/model.rs` documents every field.

**Lesson visuals:** `content/rust-basics/chNN/overlay/lessons.toml`. Each stepper screen gets panels and per-step ops, written from the spec's "Visual state" column. Static visuals on concept screens are panels without steps.

```toml
[[lessons]]
id = "L1.2"
read_more = ["ch01-02-hello-world.md"]       # Rust Book chapters for "Read more"

[[lessons.screens]]
key = "s7"                                    # screens are s1, s2, …; quick checks qc1, qc2, …
panels = [{ kind = "terminal", id = "out", title = "Output", cursor = true }]
steps = [
  { n = 1, ops = [{ panel = "out", op = "print", text = "Hello, " }] },
  { n = 4, ops = [{ panel = "out", op = "print", text = "2 + 3 = 5\n" }],
    links = [{ from = "5:15-16", to = "5:30-30" }] },      # arrows between code spans
]
```

- **Multi-file screens:** a screen with several `**Code** (\`path\`):` blocks for different files shows them as tabs; step rows written `src/lib.rs:2` switch to that file.
- **Panels:** `terminal` (printed output) and `scene` (boxes and arrows; `layout = "row" | "tree" | "grid"`; nodes with `kind = box | file | folder | code`, `parent`, `state`).
- **Program output:** a `**Program output** (\`cargo run -- …\`):` block shows a program's output in a terminal panel: from the start on concept screens, on the last step of a stepper. It isn't checked against a run (the run may need arguments or files).
- **Test output:** a screen whose code has `#[test]` functions and no `main` is checked with `cargo test`. Its `**Test output** (\`cargo test\`):` block (or `**Runtime output**`, for tests that fail on purpose) becomes a terminal panel: shown from the start on concept and compare screens, printed on the last step of a stepper. Overlay ops on a step are added to the imported ones.
- **Ops:** `print`, `badge`, `node` (`hidden`, `style`, `lit`, `ghost`, `badge`, `note`, `value`; an empty string clears a badge or value), `edge`, `reset`. `value` shows a variable's or result's value in code font.
- **Step extras:** `notes` (badges next to code lines) and `links` (arrows between code spans).
- Memory diagrams, ownership and borrow arrows are scenes too: stack and heap groups, `cells` nodes, pointer edges, and `ghost` for moved values.

## 4. Validate

```bash
cargo run -p course-check -- validate NN             # static rules: concepts, forward references, visible tests, panels, todos
cargo run -p course-check -- validate NN --dynamic   # compiles and runs everything with real rustc
```

The dynamic run checks the following:
- every solution is Accepted;
- every starter compiles (Guided stage) and is not Accepted;
- every wrong variant fails exactly the tests it names;
- every "Does this compile?" and "Predict the output" answer matches rustc;
- every lesson program compiles, and every error screen fails with the message it shows.

Both runs must be clean.

## 5. Bundle

```bash
cargo run -p course-check -- bundle
```

This refuses to bundle a chapter with errors. Rebuild the app, and the chapter switches from "Coming soon" to playable. Learner progress is keyed by stable IDs, so content edits never lose it.

## When the engine needs to grow

A new kind of run, assertion or visual is a small, one-time engine change in `rb-content` (model), `rb-grader` and the frontend (`Panels.svelte` or `QuestionCard.svelte`). These are expected first in:

| Chapter | Needs |
|---|---|
| 2 | Done: generated suites, fixtures, checker programs, checkpoint parts, vendored `rand` (`content/<course>/vendor/`, from `cargo vendor`) |
| 3 | `unit` runs (hidden `#[test]` functions) |
| 4 | `compile_fail` runs (expected error codes) |
| 9 | Done: panic rows, expected-failure program rows and suite cases, warning screens in lessons |
| 10 | Done: source facts for traits and generics (`trait:NAME`, `required_fn:`/`default_fn:`, `bound:TRAIT`, `where`, `dyn`, `lifetime:NAME`, `impl_unbounded`, `borrow`, `deref`, `field_access:NAME`); steppers over a rejected program (their **Compiler output** is checked and shown on the last step); lib-plus-main lesson packages named by `main.rs`'s `use` |
| 11 | Done: test-writing problems (supplied read-only code, the learner's tests run against it and against mutants, `cargo test` command rows), test facts in source rules, lesson and quiz code checked with `cargo test` |
| 12 | Done: command-line rows (arguments, files, environment variables, exact or prefix stderr), `cargo test` rows in any problem, program output in lessons |
| 13 | Done: source facts `closure_move` (a `move` closure) and `range` (any `a..b` or `a..=b` expression) |
| 14 | Done: workspaces (a starter may list `member/Cargo.toml` and `member/src/…` files, and learners can create them); `` `cargo test --doc` `` rows (Cargo runs the doc tests directly) and `-p NAME` in `cargo test` rows; `cargo doc …` and `cargo build …` rows with arguments (`build_ok`); manifest rows for a member's manifest (`` `cli/Cargo.toml` ``, key `cli/Cargo.toml:dependencies.x.path`) and non-string values compared as TOML text (`["core", "cli"]`); source facts `doc_crate`, `doc:NAME`, `doc_heading:TEXT`, `doc_code` |
| 15 | Done: function rows in a workspace call the library of its one member with a `src/lib.rs` (`core/src/lib.rs`) when the root has none |
| 17 | Done: vendored `trpl`; lesson and quiz programs that use `trpl::` (like `rand::`) are built with Cargo |
| 19 | Done: a destructuring `let` records its struct and variant patterns (`pat:NAME`), as `match` arms do |
| 21 | Done: rows that open local TCP connections (`127.0.0.1` only); validate these chapters outside any network sandbox |
