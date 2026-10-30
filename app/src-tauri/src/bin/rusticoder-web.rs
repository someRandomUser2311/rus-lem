//! Development preview: serves the app's commands over HTTP on localhost so the UI
//! can run in a normal browser against the real engine (content, grader, progress,
//! rust-analyzer). Messages from rust-analyzer are buffered and fetched with
//! `lsp_poll`. Not shipped with the app.
//!
//! `cargo run -p rusticoder --bin rusticoder-web -- [--data DIR] [--port 1430]`

use rusticoder_lib::AppState;
use rusticoder_lib::commands as c;
use serde_json::{Value, json};
use std::path::PathBuf;
use std::sync::Arc;

fn arg(args: &Value, name: &str) -> Value {
    args.get(name).cloned().unwrap_or(Value::Null)
}

fn parse<T: serde::de::DeserializeOwned>(args: &Value, name: &str) -> Result<T, String> {
    serde_json::from_value(arg(args, name)).map_err(|e| format!("argument `{name}`: {e}"))
}

fn ok<T: serde::Serialize>(r: Result<T, String>) -> Result<Value, String> {
    r.and_then(|v| serde_json::to_value(v).map_err(|e| e.to_string()))
}

type Inbox = Arc<std::sync::Mutex<Vec<String>>>;

fn dispatch(st: &AppState, inbox: &Inbox, cmd: &str, a: &Value) -> Result<Value, String> {
    match cmd {
        "app_info" => ok(c::app_info(st)),
        "check_toolchain" => ok(Ok(c::check_toolchain(st))),
        "warm_up" => ok(c::warm_up(st)),
        "save_profile" => ok(c::save_profile(st, parse(a, "profile")?)),
        "set_setting" => ok(c::set_setting(st, parse(a, "key")?, arg(a, "value"))),
        "reset_progress" => ok(c::reset_progress(st)),
        "get_path" => ok(c::get_path(st)),
        "get_chapter" => ok(c::get_chapter(st, parse(a, "id")?)),
        "complete_lesson" => ok(c::complete_lesson(
            st,
            parse(a, "lesson")?,
            parse(a, "tries")?,
        )),
        "check_answer" => ok(c::check_answer(
            st,
            parse(a, "question")?,
            parse(a, "response")?,
            parse(a, "record")?,
        )),
        "complete_set" => ok(c::complete_set(st, parse(a, "set")?, parse(a, "score")?)),
        "get_problem" => ok(c::get_problem(st, parse(a, "id")?)),
        "save_draft" => ok(c::save_draft(st, parse(a, "id")?, parse(a, "files")?)),
        "open_hint" => ok(c::open_hint(st, parse(a, "id")?, parse(a, "n")?)),
        "grade" => {
            let events = std::sync::Mutex::new(Vec::new());
            let r = c::grade(
                st,
                parse(a, "id")?,
                parse(a, "files")?,
                parse(a, "submit")?,
                parse(a, "reviewChapter")?,
                parse(a, "part")?,
                |e| {
                    events
                        .lock()
                        .unwrap()
                        .push(serde_json::to_value(e).unwrap_or(Value::Null))
                },
            )?;
            Ok(json!({ "events": events.into_inner().unwrap(), "result": r }))
        }
        "cancel_grade" => {
            c::cancel_grade(st);
            Ok(Value::Null)
        }
        "get_profile" => ok(c::get_profile(st)),
        "get_review" => ok(c::get_review(st, parse(a, "chapter")?)),
        "read_book" => ok(c::read_book(st, parse(a, "file")?)),
        "lsp_start" => {
            inbox.lock().unwrap().clear();
            let inbox = Arc::clone(inbox);
            ok(c::lsp_start(
                st,
                parse(a, "id")?,
                parse(a, "files")?,
                move |m| inbox.lock().unwrap().push(m),
            ))
        }
        "lsp_poll" => Ok(json!(std::mem::take(&mut *inbox.lock().unwrap()))),
        "lsp_send" => ok(c::lsp_send(st, parse(a, "message")?)),
        "lsp_stop" => {
            c::lsp_stop(st);
            Ok(Value::Null)
        }
        other => Err(format!("unknown command {other}")),
    }
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let flag = |name: &str| {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1))
            .cloned()
    };
    let here = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let resources = flag("--resources")
        .map(PathBuf::from)
        .unwrap_or_else(|| here.join("resources"));
    let data = flag("--data")
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::temp_dir().join("rusticoder-web"));
    let port: u16 = flag("--port").and_then(|p| p.parse().ok()).unwrap_or(1430);
    let st = Arc::new(AppState::load(&resources, data.clone())?);
    let inbox: Inbox = Arc::default();
    let server =
        tiny_http::Server::http(("127.0.0.1", port)).map_err(|e| anyhow::anyhow!("{e}"))?;
    println!(
        "rusticoder-web on http://127.0.0.1:{port} (data in {})",
        data.display()
    );
    for mut req in server.incoming_requests() {
        let st = Arc::clone(&st);
        let inbox = Arc::clone(&inbox);
        std::thread::spawn(move || {
            let cors = [
                tiny_http::Header::from_bytes("Access-Control-Allow-Origin", "*").unwrap(),
                tiny_http::Header::from_bytes("Access-Control-Allow-Headers", "content-type")
                    .unwrap(),
                tiny_http::Header::from_bytes("Content-Type", "application/json").unwrap(),
            ];
            if req.method() == &tiny_http::Method::Options {
                let mut resp = tiny_http::Response::empty(204);
                for h in cors {
                    resp.add_header(h);
                }
                let _ = req.respond(resp);
                return;
            }
            let cmd = req.url().trim_start_matches("/invoke/").to_string();
            let mut body = String::new();
            let _ = req.as_reader().read_to_string(&mut body);
            let args: Value = serde_json::from_str(&body).unwrap_or(Value::Null);
            let (code, out) = match dispatch(&st, &inbox, &cmd, &args) {
                Ok(v) => (200, v),
                Err(e) => (400, Value::String(e)),
            };
            let mut resp = tiny_http::Response::from_string(out.to_string()).with_status_code(code);
            for h in cors {
                resp.add_header(h);
            }
            let _ = req.respond(resp);
        });
    }
    Ok(())
}
