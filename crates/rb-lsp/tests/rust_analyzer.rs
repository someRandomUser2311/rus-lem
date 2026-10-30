//! End-to-end: rust-analyzer answers completion and hover and reports a syntax error
//! through the bridge. Skipped when rust-analyzer isn't installed.

use rb_lsp::{Session, prepare_workspace};
use std::path::PathBuf;
use std::process::Command;
use std::sync::mpsc;
use std::time::{Duration, Instant};

fn rust_analyzer() -> Option<PathBuf> {
    let home = std::env::var_os("HOME").map(PathBuf::from)?;
    let p = home.join(".cargo/bin/rust-analyzer");
    Command::new(&p)
        .arg("--version")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|_| p)
}

fn wait_for(
    rx: &mpsc::Receiver<serde_json::Value>,
    secs: u64,
    pred: impl Fn(&serde_json::Value) -> bool,
) -> Option<serde_json::Value> {
    let end = Instant::now() + Duration::from_secs(secs);
    while Instant::now() < end {
        if let Ok(v) = rx.recv_timeout(Duration::from_millis(200))
            && pred(&v)
        {
            return Some(v);
        }
    }
    None
}

#[test]
fn completion_hover_and_diagnostics() {
    let Some(ra) = rust_analyzer() else {
        eprintln!("rust-analyzer not installed; skipping");
        return;
    };
    let root = std::env::temp_dir().join(format!("rb-lsp-test-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    // Line 3 misses its `;`.
    let code =
        "fn main() {\n    let name = \"Ferris\";\n    println!(\"{name}\")\n    name.len();\n}\n";
    let paths =
        prepare_workspace(&root, "P01-03", &[("main.rs".into(), code.into())], false).unwrap();
    let uri = format!("file://{}", paths[0].display());
    let root_uri = format!("file://{}", root.display());

    let (tx, rx) = mpsc::channel();
    let session = Session::start(
        &ra,
        &root,
        None,
        move |m| {
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(&m) {
                let _ = tx.send(v);
            }
        },
        || {},
    )
    .unwrap();
    let send = |v: serde_json::Value| session.send(&v.to_string()).unwrap();

    send(
        serde_json::json!({"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {
            "processId": null, "rootUri": root_uri, "capabilities": {
                "textDocument": {"hover": {"contentFormat": ["markdown", "plaintext"]}, "completion": {"completionItem": {"snippetSupport": false}}, "publishDiagnostics": {}}
            }
        }}),
    );
    assert!(
        wait_for(&rx, 30, |v| v["id"] == 1).is_some(),
        "no initialize response"
    );
    send(serde_json::json!({"jsonrpc": "2.0", "method": "initialized", "params": {}}));
    send(
        serde_json::json!({"jsonrpc": "2.0", "method": "textDocument/didOpen", "params": {
            "textDocument": {"uri": uri, "languageId": "rust", "version": 1, "text": code}
        }}),
    );

    // Inline error: the missing `;` is a syntax error rust-analyzer reports by itself.
    let diag = wait_for(&rx, 60, |v| {
        v["method"] == "textDocument/publishDiagnostics"
            && v["params"]["diagnostics"]
                .as_array()
                .is_some_and(|d| !d.is_empty())
    });
    let diag = diag.expect("no diagnostics for the missing semicolon");
    let first = &diag["params"]["diagnostics"][0];
    assert_eq!(first["range"]["start"]["line"], 2, "{first}");

    // Hover on `name` (line 4, column 5) — retried while rust-analyzer finishes indexing.
    let mut hover_text = String::new();
    for i in 0..30 {
        send(
            serde_json::json!({"jsonrpc": "2.0", "id": 100 + i, "method": "textDocument/hover", "params": {
                "textDocument": {"uri": uri}, "position": {"line": 3, "character": 5}
            }}),
        );
        if let Some(r) = wait_for(&rx, 5, |v| v["id"] == 100 + i) {
            hover_text = r["result"]["contents"]["value"]
                .as_str()
                .unwrap_or("")
                .to_string();
            if hover_text.contains("&str") {
                break;
            }
        }
        std::thread::sleep(Duration::from_millis(500));
    }
    assert!(hover_text.contains("&str"), "hover was {hover_text:?}");

    // Completion after `name.` offers `len`.
    let mut labels: Vec<String> = Vec::new();
    for i in 0..90 {
        send(
            serde_json::json!({"jsonrpc": "2.0", "id": 200 + i, "method": "textDocument/completion", "params": {
                "textDocument": {"uri": uri}, "position": {"line": 3, "character": 9}
            }}),
        );
        if let Some(r) = wait_for(&rx, 10, |v| v["id"] == 200 + i) {
            let items = r["result"]["items"]
                .as_array()
                .or(r["result"].as_array())
                .cloned()
                .unwrap_or_default();
            labels = items
                .iter()
                .filter_map(|it| it["label"].as_str().map(str::to_string))
                .collect();
            if labels.iter().any(|l| l.starts_with("len")) {
                eprintln!("completion ready after {i} retries");
                break;
            }
        }
        // rust-analyzer returns an empty, incomplete list until the standard library is indexed.
        std::thread::sleep(Duration::from_millis(500));
    }
    assert!(
        labels.iter().any(|l| l.starts_with("len")),
        "completion labels: {:?}",
        &labels[..labels.len().min(10)]
    );
    drop(session);
    let _ = std::fs::remove_dir_all(&root);
}
