//! Small line-based helpers for the course plan's regular Markdown.

/// A heading section: the heading text (without `#`s) and the lines under it.
#[derive(Debug, Clone)]
pub struct Section<'a> {
    pub heading: String,
    pub lines: Vec<&'a str>,
}

/// Splits `lines` at headings of exactly `level` (`##` = 2). Text before the first
/// heading is dropped. Fenced code blocks are never split.
pub fn sections<'a>(lines: &[&'a str], level: usize) -> Vec<Section<'a>> {
    let prefix = format!("{} ", "#".repeat(level));
    let mut out: Vec<Section<'a>> = Vec::new();
    let mut in_fence = false;
    for &line in lines {
        if is_fence(line) {
            in_fence = !in_fence;
        }
        if !in_fence && line.starts_with(&prefix) {
            out.push(Section {
                heading: line[prefix.len()..].trim().to_string(),
                lines: Vec::new(),
            });
        } else if let Some(s) = out.last_mut() {
            s.lines.push(line);
        }
    }
    out
}

pub fn is_fence(line: &str) -> bool {
    let t = line.trim_start();
    t.starts_with("```") || t.starts_with("~~~")
}

/// Splits "A · B · C" headings.
pub fn dot_parts(heading: &str) -> Vec<String> {
    heading.split(" · ").map(|s| s.trim().to_string()).collect()
}

/// Parses a Markdown table starting at the first `|` line. Returns rows without the
/// header and separator. Pipes inside backtick code spans and `\|` don't split cells.
/// Blank lines inside the table don't end it: rows after them still belong to it,
/// unless they start a new table (a row followed by a `|---` separator).
pub fn table(lines: &[&str]) -> Vec<Vec<String>> {
    let is_row = |l: &str| l.trim_start().starts_with('|');
    let is_separator = |l: &str| {
        let t = l.trim();
        t.starts_with('|')
            && t.contains('-')
            && t.chars().all(|c| matches!(c, '|' | '-' | ':' | ' '))
    };
    let start = match lines.iter().position(|l| is_row(l)) {
        Some(i) => i,
        None => return Vec::new(),
    };
    let mut rows = Vec::new();
    let mut i = start + 2;
    while i < lines.len() {
        if is_row(lines[i]) {
            rows.push(split_row(lines[i]));
            i += 1;
            continue;
        }
        if !lines[i].trim().is_empty() {
            break;
        }
        let next = (i..lines.len()).find(|&j| !lines[j].trim().is_empty());
        match next {
            Some(j) if is_row(lines[j]) && !lines.get(j + 1).is_some_and(|l| is_separator(l)) => {
                i = j
            }
            _ => break,
        }
    }
    rows
}

/// The first table whose header row's first cell is `first_header`.
pub fn table_with_header(lines: &[&str], first_header: &str) -> Vec<Vec<String>> {
    tables_with_header(lines, first_header)
        .into_iter()
        .next()
        .unwrap_or_default()
}

/// Every table whose header row's first cell is `first_header`.
pub fn tables_with_header(lines: &[&str], first_header: &str) -> Vec<Vec<Vec<String>>> {
    let mut out = Vec::new();
    let mut prev_table_line = false;
    for (i, l) in lines.iter().enumerate() {
        let is_row = l.trim_start().starts_with('|');
        if is_row
            && !prev_table_line
            && split_row(l).first().map(|c| c.as_str()) == Some(first_header)
        {
            out.push(table(&lines[i..]));
        }
        prev_table_line = is_row;
    }
    out
}

pub fn split_row(line: &str) -> Vec<String> {
    let t = line.trim();
    let t = t.strip_prefix('|').unwrap_or(t);
    let t = t.strip_suffix('|').unwrap_or(t);
    let mut cells = Vec::new();
    let mut cur = String::new();
    let mut ticks = 0usize; // length of the open backtick run, 0 when outside code
    let chars: Vec<char> = t.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c == '`' {
            let mut run = 0;
            while i + run < chars.len() && chars[i + run] == '`' {
                run += 1;
            }
            if ticks == 0 {
                ticks = run;
            } else if run == ticks {
                ticks = 0;
            }
            cur.extend(std::iter::repeat_n('`', run));
            i += run;
            continue;
        }
        if c == '\\' && i + 1 < chars.len() && chars[i + 1] == '|' {
            cur.push('|');
            i += 2;
            continue;
        }
        if c == '|' && ticks == 0 {
            cells.push(cur.trim().to_string());
            cur.clear();
        } else {
            cur.push(c);
        }
        i += 1;
    }
    cells.push(cur.trim().to_string());
    cells
}

/// Field/Value table → (field, value) pairs.
pub fn fields(lines: &[&str]) -> Vec<(String, String)> {
    table_with_header(lines, "Field")
        .into_iter()
        .filter(|r| r.len() >= 2)
        .map(|r| (r[0].clone(), r[1].clone()))
        .collect()
}

pub fn field<'a>(fields: &'a [(String, String)], name: &str) -> Option<&'a str> {
    fields
        .iter()
        .find(|(k, _)| k == name)
        .map(|(_, v)| v.as_str())
}

/// A fenced code block: info string and body.
#[derive(Debug, Clone)]
pub struct Fence {
    pub info: String,
    pub text: String,
}

/// Every fenced block in `lines`, in order.
pub fn fences(lines: &[&str]) -> Vec<Fence> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let t = lines[i].trim_start();
        if is_fence(t) {
            let marker: String = t.chars().take_while(|c| *c == '`' || *c == '~').collect();
            let info = t[marker.len()..].trim().to_string();
            let mut body = Vec::new();
            i += 1;
            while i < lines.len() && !lines[i].trim_start().starts_with(&marker) {
                body.push(lines[i]);
                i += 1;
            }
            out.push(Fence {
                info,
                text: body.join("\n"),
            });
        }
        i += 1;
    }
    out
}

/// Lines between a line starting with `start` (exclusive) and the next line that
/// starts a bold label `**…**` at column 0 or a heading (exclusive). Fences are kept whole.
pub fn block_after<'a>(lines: &[&'a str], start: &str) -> Option<Vec<&'a str>> {
    let i = lines.iter().position(|l| l.starts_with(start))?;
    let mut out = Vec::new();
    let first_rest = lines[i][start.len()..].trim();
    if !first_rest.is_empty() && first_rest != ":" {
        out.push(&lines[i][start.len()..]);
    }
    let mut in_fence = false;
    for &l in &lines[i + 1..] {
        if is_fence(l) {
            in_fence = !in_fence;
        }
        if !in_fence && (is_bold_label(l) || l.starts_with('#')) {
            break;
        }
        out.push(l);
    }
    Some(out)
}

pub fn is_bold_label(l: &str) -> bool {
    l.starts_with("**") && l[2..].contains("**") && !l.starts_with("**Idiomatic note")
}

/// Joins lines and trims blank lines at both ends.
pub fn text(lines: &[&str]) -> String {
    let s = lines.join("\n");
    s.trim_matches('\n').trim_end().to_string()
}

/// Strips one level of surrounding backticks: "`x`" → "x".
pub fn unbacktick(s: &str) -> String {
    let t = s.trim();
    let run = t.chars().take_while(|c| *c == '`').count();
    if run > 0 && t.len() > 2 * run && t.ends_with(&"`".repeat(run)) {
        t[run..t.len() - run].trim().to_string()
    } else {
        t.to_string()
    }
}

/// All backtick code spans in a string, unwrapped.
pub fn code_spans(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '`' {
            let mut run = 0;
            while i + run < chars.len() && chars[i + run] == '`' {
                run += 1;
            }
            let start = i + run;
            let mut j = start;
            let mut found = None;
            while j < chars.len() {
                if chars[j] == '`' {
                    let mut r = 0;
                    while j + r < chars.len() && chars[j + r] == '`' {
                        r += 1;
                    }
                    if r == run {
                        found = Some(j);
                        break;
                    }
                    j += r;
                } else {
                    j += 1;
                }
            }
            match found {
                Some(end) => {
                    out.push(
                        chars[start..end]
                            .iter()
                            .collect::<String>()
                            .trim()
                            .to_string(),
                    );
                    i = end + run;
                }
                None => break,
            }
        } else {
            i += 1;
        }
    }
    out
}

/// Concept IDs (`area.name`) in a cell, in order.
pub fn concept_ids(s: &str) -> Vec<String> {
    code_spans(s)
        .into_iter()
        .filter(|c| {
            let mut parts = c.split('.');
            matches!((parts.next(), parts.next(), parts.next()), (Some(a), Some(b), None)
                if !a.is_empty() && !b.is_empty()
                && a.chars().all(|ch| ch.is_ascii_lowercase() || ch == '_')
                && b.chars().all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_'))
        })
        .collect()
}

/// Decodes a Rust-style string literal body: `\n`, `\t`, `\"`, `\\`, `\u{…}`.
pub fn unescape(s: &str) -> String {
    let mut out = String::new();
    let mut it = s.chars().peekable();
    while let Some(c) = it.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match it.next() {
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some('r') => out.push('\r'),
            Some('0') => out.push('\0'),
            Some('"') => out.push('"'),
            Some('\'') => out.push('\''),
            Some('\\') => out.push('\\'),
            Some('u') if it.peek() == Some(&'{') => {
                it.next();
                let hex: String = it.by_ref().take_while(|c| *c != '}').collect();
                if let Some(ch) = u32::from_str_radix(&hex, 16).ok().and_then(char::from_u32) {
                    out.push(ch);
                }
            }
            Some(other) => {
                out.push('\\');
                out.push(other);
            }
            None => out.push('\\'),
        }
    }
    out
}

/// If `s` is exactly `prefix "…"` (optionally wrapped in backticks), returns the decoded string.
pub fn quoted_after(s: &str, prefix: &str) -> Option<String> {
    let t = unbacktick(s);
    let rest = t.strip_prefix(prefix)?.trim();
    // Also `prefix `"…"``: the backticks around the string only.
    let rest = if rest.starts_with('`') {
        unbacktick(rest)
    } else {
        rest.to_string()
    };
    let inner = rest.strip_prefix('"')?.strip_suffix('"')?;
    if has_bare_quote(inner) {
        // several quoted strings (`"a" + "b" repeated …`)
        return None;
    }
    Some(unescape(inner))
}

/// A `"` not escaped with a backslash.
fn has_bare_quote(s: &str) -> bool {
    let mut escaped = false;
    for c in s.chars() {
        match c {
            '"' if !escaped => return true,
            '\\' => escaped = !escaped,
            _ => escaped = false,
        }
    }
    false
}

/// A whole cell that is a single quoted string, e.g. `` `"abc"` `` → abc.
pub fn single_quoted(s: &str) -> Option<String> {
    let t = unbacktick(s);
    let inner = t.strip_prefix('"')?.strip_suffix('"')?;
    if inner.contains("\" ") || inner.contains(" \"") {
        // two quoted strings in one cell
        return None;
    }
    Some(unescape(inner))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_continues_after_blank_line() {
        let lines = [
            "| # | X |",
            "|---|---|",
            "| 1 | a |",
            "| 2 | b |",
            "",
            "| 3 | c |",
            "",
            "| # | Y |",
            "|---|---|",
            "| 9 | z |",
        ];
        let rows = table(&lines);
        assert_eq!(
            rows.iter().map(|r| r[0].as_str()).collect::<Vec<_>>(),
            ["1", "2", "3"]
        );
    }

    #[test]
    fn row_keeps_pipes_in_code() {
        let r = split_row("| 1 | V | `a|b` | x \\| y |");
        assert_eq!(r, vec!["1", "V", "`a|b`", "x | y"]);
    }

    #[test]
    fn decodes_strings() {
        assert_eq!(
            quoted_after("`stdout: \"a\\nb\\u{3000}\"`", "stdout:").unwrap(),
            "a\nb\u{3000}"
        );
        assert_eq!(code_spans("x `a` and ``b`c`` y"), vec!["a", "b`c"]);
        assert_eq!(
            concept_ids("`basic.println` (primary), `basic.main`"),
            vec!["basic.println", "basic.main"]
        );
    }
}
