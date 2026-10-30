//! Copies the Rust Book's chapters into the content folder for the offline
//! "Read more" panel, resolving mdBook's `{{#include}}` / `{{#rustdoc_include}}`
//! directives and dropping lines mdBook hides (`# ` inside Rust blocks).

use anyhow::{Context, Result};
use std::path::Path;

pub fn import(book: &Path, out: &Path) -> Result<usize> {
    let src = book.join("src");
    std::fs::create_dir_all(out)?;
    let mut n = 0;
    let mut entries: Vec<_> = std::fs::read_dir(&src)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .collect();
    entries.sort();
    for p in entries {
        let name = p
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        if !(name.starts_with("ch") && name.ends_with(".md")) {
            continue;
        }
        let text = std::fs::read_to_string(&p)?;
        let resolved = resolve(&text, &src).with_context(|| format!("resolving {name}"))?;
        std::fs::write(out.join(&name), hide_lines(&resolved))?;
        n += 1;
    }
    for f in ["LICENSE-MIT", "LICENSE-APACHE", "COPYRIGHT"] {
        if book.join(f).exists() {
            std::fs::copy(book.join(f), out.join(f))?;
        }
    }
    std::fs::write(
        out.join("ABOUT.md"),
        "Adapted from *The Rust Programming Language* by Steve Klabnik, Carol Nichols and Chris Krycho, with contributions from the Rust Community. Dual-licensed under MIT and Apache-2.0; see LICENSE-MIT and LICENSE-APACHE.\n",
    )?;
    Ok(n)
}

fn resolve(text: &str, base: &Path) -> Result<String> {
    let mut out = String::new();
    let mut rest = text;
    while let Some(start) = rest.find("{{#") {
        out.push_str(&rest[..start]);
        let after = &rest[start..];
        let Some(end) = after.find("}}") else {
            out.push_str(after);
            return Ok(out);
        };
        let directive = &after[3..end];
        let (kind, arg) = directive.split_once(' ').unwrap_or((directive, ""));
        out.push_str(&match kind {
            "include" | "rustdoc_include" => include(arg.trim(), base).unwrap_or_default(),
            _ => String::new(),
        });
        rest = &after[end + 2..];
    }
    out.push_str(rest);
    Ok(out)
}

/// `path`, `path:anchor`, `path:start:end`, `path:start:` or `path::end`.
fn include(arg: &str, base: &Path) -> Option<String> {
    let mut parts = arg.splitn(3, ':');
    let path = parts.next()?;
    let a = parts.next();
    let b = parts.next();
    let text = std::fs::read_to_string(base.join(path)).ok()?;
    let lines: Vec<&str> = text.lines().collect();
    let body: Vec<&str> = match (a, b) {
        (None, _) => lines,
        (Some(x), None) if x.parse::<usize>().is_err() && !x.is_empty() => {
            let open = format!("ANCHOR: {x}");
            let close = format!("ANCHOR_END: {x}");
            let s = lines.iter().position(|l| l.contains(&open))?;
            let e = lines
                .iter()
                .skip(s)
                .position(|l| l.contains(&close))
                .map(|i| i + s)
                .unwrap_or(lines.len());
            lines[s + 1..e].to_vec()
        }
        (Some(x), y) => {
            let s = x.parse::<usize>().unwrap_or(1).max(1) - 1;
            let e = y
                .and_then(|y| y.parse::<usize>().ok())
                .unwrap_or(lines.len())
                .min(lines.len());
            lines.get(s..e)?.to_vec()
        }
    };
    let kept: Vec<&str> = body
        .into_iter()
        .filter(|l| !l.contains("ANCHOR: ") && !l.contains("ANCHOR_END: "))
        .collect();
    Some(kept.join("\n"))
}

/// Drops mdBook's hidden lines (`# …` and a lone `#`) inside ```rust blocks and
/// strips extra attributes from the fence info.
fn hide_lines(text: &str) -> String {
    let mut out = Vec::new();
    let mut in_fence = false;
    let mut in_rust = false;
    for line in text.lines() {
        let t = line.trim_start();
        if t.starts_with("```") {
            if in_fence {
                in_fence = false;
                in_rust = false;
                out.push(line.to_string());
            } else {
                in_fence = true;
                let info = t.trim_start_matches('`');
                in_rust = info.is_empty() || info.starts_with("rust");
                out.push(if in_rust {
                    "```rust".to_string()
                } else {
                    line.to_string()
                });
            }
            continue;
        }
        if in_rust && (t == "#" || t.starts_with("# ")) {
            continue;
        }
        out.push(line.to_string());
    }
    out.join("\n") + "\n"
}
