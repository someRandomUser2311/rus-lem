# Rusticoder

A desktop app that teaches Rust from *The Rust Programming Language*: short visual lessons, quick questions, and coding problems graded by real tests with the learner's own Rust toolchain. Fully local and offline. Tauri 2, Svelte 5, Rust.

The course content is specified in `../rust-basics-course-plan/` (the reviewed Markdown plan). Rusticoder turns it into data with `course-check`, validates it by compiling and running everything, and bundles it into the app. **Chapters 1–14 are bundled.** The other chapters show as "Coming soon" until they're added (see [docs/adding-a-chapter.md](docs/adding-a-chapter.md)); adding one needs no app code.

## Requirements

- Rust (rustc + Cargo, edition 2024, 1.85 or newer) via [rustup](https://rustup.rs). Learners need it too: the app compiles their code with it.
- Node.js 20+ for the frontend build.
- Optional: `rustup component add rust-analyzer` for the editor assists (off by default; Settings → Editor help).

## Run

The app's content resources (`app/src-tauri/resources/`) are generated, not tracked. Write them once after checking out, and again after changing `content/`:

```bash
cargo run -p course-check -- bundle
```

Then:

```bash
cd app
npm install
npm run tauri dev
```

Release build of the macOS app: `cd app && npx tauri build --bundles app` (output in `target/release/bundle/macos/Rusticoder.app`).

### Browser preview (development)

The same backend can be served over HTTP so the UI runs in a normal browser, which makes it easy to try content and screens quickly. rust-analyzer messages are relayed by polling.

```bash
cargo run -p rusticoder --bin rusticoder-web -- --data /tmp/rusticoder-web   # backend on 127.0.0.1:1430
cd app && npm run dev                                                        # UI on http://localhost:5173 (proxies /invoke)
```

## Test

```bash
cargo test --workspace                                         # engine unit tests (grader, rules, parser, quiz, LSP framing)
cargo run -p course-check -- validate 1 --dynamic              # compile and run every solution, starter, wrong variant and quiz program
cargo run -p rb-cli -- grade P01-06 content/rust-basics/ch01/problems/P01-06/solution --submit
cd app && npx svelte-check --tsconfig ./tsconfig.app.json      # frontend types
```

## Layout

| Path | What it is |
|---|---|
| `crates/rb-content` | The content model (lessons, questions, problems, tests). TypeScript types are generated from it into `app/src/lib/bindings` by `cargo test` (ts-rs). |
| `crates/rb-specparse` | Reads the Markdown course plan into the model. |
| `crates/rb-runner` | Finds the learner's toolchain; runs rustc, Cargo and programs with timeouts, output caps and process-group kills. |
| `crates/rb-srccheck` | Source rules on parsed Rust (macro counts, typed values in format strings, unchanged `let` lines, banned keywords). |
| `crates/rb-grader` | Runs a problem's tests: each distinct run once, then every test's assertions. Run = visible tests, Submit = all. Also repeated runs, generated suites, fixture builds (a named `let` takes a test value), checker programs, and for test-writing problems the learner's own tests against the supplied code and each mutant. |
| `crates/rb-quiz` | Checks answers to the nine question types. |
| `crates/rb-progress` | SQLite progress store and the rules from `progress-and-unlocking.md`: unlocking, XP, streaks, hints, solutions, hidden-test reveal, mastery, review, badges. |
| `crates/rb-lsp` | Runs rust-analyzer and frames LSP messages for the editor. |
| `crates/course-check` | `import` (plan → content), `validate` (static + `--dynamic`), `bundle` (content → app resources), `book` (Rust Book for "Read more"). |
| `crates/rb-cli` | Grade a problem from the terminal. |
| `app/src-tauri` | Tauri shell: `commands.rs` (plain functions), `tauri_cmds.rs` (window), `bin/rusticoder-web.rs` (browser preview). |
| `app/src` | Svelte 5 UI: path, lesson player and visualizers, question player, coding/project screen (CodeMirror 6), progress, review, settings, onboarding. |
| `content/rust-basics` | Course content: generated layer (`*.gen.json`) + authored overlay (TOML, solutions, wrong variants), the Rust Book chapters, and `vendor/` (crates such as `rand`, vendored with `cargo vendor` so Cargo problems build offline). |

Crate downloads during development may need a proxy; pass it as an environment variable on your own commands (for example `CARGO_HTTP_PROXY=socks5h://host:port`), never in the repository's configuration.
