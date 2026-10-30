//! Running a child process with stdin, a wall-clock limit and output caps.

use std::io::{Read, Write};
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct Limits {
    pub timeout: Duration,
    /// Bytes kept per stream; the process is killed once a stream goes past it.
    pub max_output: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Limits {
            timeout: Duration::from_secs(10),
            max_output: 1 << 20,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Output {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    /// Exit code, if the process exited normally.
    pub code: Option<i32>,
    /// Signal that ended the process (Unix), if any.
    pub signal: Option<i32>,
    pub timed_out: bool,
    /// A stream went past `max_output`.
    pub output_limit: bool,
    pub elapsed: Duration,
}

impl Output {
    pub fn success(&self) -> bool {
        self.code == Some(0) && !self.timed_out && !self.output_limit
    }
    pub fn stdout_str(&self) -> String {
        String::from_utf8_lossy(&self.stdout).into_owned()
    }
    pub fn stderr_str(&self) -> String {
        String::from_utf8_lossy(&self.stderr).into_owned()
    }
}

/// Runs `cmd` to completion. The child gets its own process group so a timeout
/// kills everything it started.
pub fn run(
    mut cmd: Command,
    stdin: &[u8],
    limits: &Limits,
    cancel: Option<&AtomicBool>,
) -> std::io::Result<Output> {
    cmd.stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        cmd.process_group(0);
    }
    let start = Instant::now();
    let mut child = cmd.spawn()?;

    let input = stdin.to_vec();
    let mut child_stdin = child.stdin.take();
    let writer = thread::spawn(move || {
        if let Some(s) = child_stdin.as_mut() {
            // The program may exit without reading everything; that's fine.
            let _ = s.write_all(&input);
        }
        drop(child_stdin);
    });

    let over = Arc::new(AtomicBool::new(false));
    let out_reader = reader(child.stdout.take(), limits.max_output, over.clone());
    let err_reader = reader(child.stderr.take(), limits.max_output, over.clone());

    let mut timed_out = false;
    let status = loop {
        if let Some(st) = child.try_wait()? {
            break Some(st);
        }
        let cancelled = cancel.is_some_and(|c| c.load(Ordering::Relaxed));
        if over.load(Ordering::Relaxed) || cancelled || start.elapsed() > limits.timeout {
            timed_out = start.elapsed() > limits.timeout;
            kill(&mut child);
            break child.wait().ok();
        }
        thread::sleep(Duration::from_millis(2));
    };
    let elapsed = start.elapsed();
    // A grandchild could keep the pipes open; make sure the whole group is gone.
    kill_group(&child);
    let _ = writer.join();
    let stdout = out_reader.join().unwrap_or_default();
    let stderr = err_reader.join().unwrap_or_default();

    let (code, signal) = match status {
        Some(st) => {
            #[cfg(unix)]
            {
                use std::os::unix::process::ExitStatusExt;
                (st.code(), st.signal())
            }
            #[cfg(not(unix))]
            {
                (st.code(), None)
            }
        }
        None => (None, None),
    };
    Ok(Output {
        stdout,
        stderr,
        code,
        signal,
        timed_out,
        output_limit: over.load(Ordering::Relaxed),
        elapsed,
    })
}

fn reader<R: Read + Send + 'static>(
    src: Option<R>,
    cap: usize,
    over: Arc<AtomicBool>,
) -> thread::JoinHandle<Vec<u8>> {
    thread::spawn(move || {
        let mut buf = Vec::new();
        let Some(mut src) = src else { return buf };
        let mut chunk = [0u8; 64 * 1024];
        loop {
            match src.read(&mut chunk) {
                Ok(0) | Err(_) => break,
                Ok(n) => {
                    let room = cap.saturating_sub(buf.len());
                    buf.extend_from_slice(&chunk[..n.min(room)]);
                    if n > room {
                        over.store(true, Ordering::Relaxed);
                        // keep draining so the child doesn't block, but drop the bytes
                    }
                }
            }
        }
        buf
    })
}

fn kill(child: &mut Child) {
    kill_group(child);
    let _ = child.kill();
}

fn kill_group(child: &Child) {
    #[cfg(unix)]
    unsafe {
        // The group id equals the child's pid (process_group(0)).
        libc::kill(-(child.id() as i32), libc::SIGKILL);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn echoes_stdin() {
        let out = run(Command::new("cat"), b"hi\n", &Limits::default(), None).unwrap();
        assert_eq!(out.stdout, b"hi\n");
        assert!(out.success());
    }

    #[test]
    fn times_out() {
        let mut c = Command::new("sh");
        c.args(["-c", "sleep 5"]);
        let out = run(
            c,
            b"",
            &Limits {
                timeout: Duration::from_millis(200),
                max_output: 1024,
            },
            None,
        )
        .unwrap();
        assert!(out.timed_out);
        assert!(out.elapsed < Duration::from_secs(2));
    }

    #[test]
    fn caps_output() {
        let mut c = Command::new("sh");
        c.args(["-c", "yes"]);
        let out = run(
            c,
            b"",
            &Limits {
                timeout: Duration::from_secs(5),
                max_output: 1000,
            },
            None,
        )
        .unwrap();
        assert!(out.output_limit);
        assert_eq!(out.stdout.len(), 1000);
    }
}
