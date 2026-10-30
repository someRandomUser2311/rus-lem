//! Runs rust-analyzer and moves LSP messages between it and the editor.
//!
//! The editor (CodeMirror's LSP client) sends and receives plain JSON-RPC strings;
//! this crate adds and strips the `Content-Length` framing on stdio.

use anyhow::{Context, Result};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Session {
    child: Child,
    stdin: Arc<Mutex<ChildStdin>>,
    pub root: PathBuf,
}

impl Session {
    /// Starts rust-analyzer in `root`. `on_message` receives each message from the
    /// server (a JSON string); `on_exit` runs when the server stops.
    pub fn start(
        ra: &Path,
        root: &Path,
        cargo_dir: Option<&Path>,
        on_message: impl Fn(String) + Send + 'static,
        on_exit: impl FnOnce() + Send + 'static,
    ) -> Result<Session> {
        let mut cmd = Command::new(ra);
        cmd.current_dir(root)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null());
        // rust-analyzer runs `cargo metadata`; it needs the toolchain on PATH.
        if let Some(dir) = cargo_dir {
            let mut path = vec![dir.to_path_buf()];
            if let Some(p) = std::env::var_os("PATH") {
                path.extend(std::env::split_paths(&p));
            }
            if let Ok(joined) = std::env::join_paths(path) {
                cmd.env("PATH", joined);
            }
        }
        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            cmd.process_group(0);
        }
        let mut child = cmd
            .spawn()
            .with_context(|| format!("starting {}", ra.display()))?;
        let stdin = Arc::new(Mutex::new(child.stdin.take().context("no stdin")?));
        let stdout = child.stdout.take().context("no stdout")?;
        thread::spawn(move || {
            let mut r = BufReader::new(stdout);
            while let Ok(Some(msg)) = read_message(&mut r) {
                on_message(msg);
            }
            on_exit();
        });
        Ok(Session {
            child,
            stdin,
            root: root.to_path_buf(),
        })
    }

    pub fn send(&self, msg: &str) -> Result<()> {
        let mut w = self
            .stdin
            .lock()
            .map_err(|_| anyhow::anyhow!("rust-analyzer stdin poisoned"))?;
        write!(w, "Content-Length: {}\r\n\r\n{msg}", msg.len())?;
        w.flush()?;
        Ok(())
    }

    pub fn stop(&mut self) {
        #[cfg(unix)]
        unsafe {
            libc::kill(-(self.child.id() as i32), libc::SIGKILL);
        }
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Reads one framed message; `None` at end of stream.
pub fn read_message(r: &mut impl BufRead) -> std::io::Result<Option<String>> {
    let mut len: Option<usize> = None;
    loop {
        let mut line = String::new();
        if r.read_line(&mut line)? == 0 {
            return Ok(None);
        }
        let line = line.trim_end();
        if line.is_empty() {
            break;
        }
        if let Some(v) = line.strip_prefix("Content-Length:") {
            len = v.trim().parse().ok();
        }
    }
    let Some(len) = len else {
        return Ok(Some(String::new()));
    };
    let mut buf = vec![0; len];
    r.read_exact(&mut buf)?;
    Ok(Some(String::from_utf8_lossy(&buf).into_owned()))
}

/// Writes the editor workspace for a problem so rust-analyzer sees a Cargo package.
/// Single-file problems get a generated manifest with the file at `src/main.rs`.
/// Returns the on-disk path of each editor file, in order.
pub fn prepare_workspace(
    root: &Path,
    package: &str,
    files: &[(String, String)],
    cargo_project: bool,
) -> Result<Vec<PathBuf>> {
    std::fs::create_dir_all(root)?;
    let mut paths = Vec::new();
    if !cargo_project {
        let name: String = package
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() {
                    c.to_ascii_lowercase()
                } else {
                    '_'
                }
            })
            .collect();
        std::fs::write(
            root.join("Cargo.toml"),
            format!(
                "[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\n"
            ),
        )?;
    }
    for (path, text) in files {
        let rel = if cargo_project {
            PathBuf::from(path)
        } else {
            PathBuf::from("src").join(path)
        };
        let p = root.join(rel);
        if let Some(dir) = p.parent() {
            std::fs::create_dir_all(dir)?;
        }
        std::fs::write(&p, text)?;
        paths.push(p);
    }
    Ok(paths)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn framing_round_trip() {
        let data = b"Content-Length: 7\r\n\r\n{\"a\":1}Content-Length: 2\r\n\r\n{}";
        let mut r = BufReader::new(&data[..]);
        assert_eq!(read_message(&mut r).unwrap().as_deref(), Some("{\"a\":1}"));
        assert_eq!(read_message(&mut r).unwrap().as_deref(), Some("{}"));
        assert_eq!(read_message(&mut r).unwrap(), None);
    }
}
