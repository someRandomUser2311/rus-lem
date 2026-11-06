use std::time::Duration;
use trpl::Either;

#[derive(Debug, Clone, PartialEq)]
pub struct Job {
    pub name: String,
    pub delay_ms: u64,
    pub size: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Outcome {
    Done(String, u32),
    Empty(String),
    TimedOut(String),
}

pub fn parse_jobs(text: &str) -> Result<Vec<Job>, String> {
    let mut jobs = Vec::new();
    for (i, line) in text.lines().enumerate() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.is_empty() {
            continue;
        }
        let n = i + 1;
        if fields.len() != 3 {
            return Err(format!("line {n}: expected name, delay and size"));
        }
        let delay_ms = fields[1]
            .parse::<u64>()
            .map_err(|_| format!("line {n}: bad delay"))?;
        let size = fields[2]
            .parse::<u32>()
            .map_err(|_| format!("line {n}: bad size"))?;
        jobs.push(Job {
            name: fields[0].to_string(),
            delay_ms,
            size,
        });
    }
    Ok(jobs)
}

pub async fn download(job: Job, limit_ms: u64) -> Outcome {
    let work = trpl::sleep(Duration::from_millis(job.delay_ms));
    let limit = trpl::sleep(Duration::from_millis(limit_ms));
    match trpl::select(work, limit).await {
        Either::Left(()) => Outcome::Done(job.name, job.size),
        Either::Right(()) => Outcome::TimedOut(job.name),
    }
}

pub async fn download_all(jobs: Vec<Job>, limit_ms: u64) -> Vec<Outcome> {
    let (tx, mut rx) = trpl::channel();
    for (i, job) in jobs.into_iter().enumerate() {
        let tx = tx.clone();
        trpl::spawn_task(async move {
            let outcome = download(job, limit_ms).await;
            tx.send((i, outcome)).expect("the receiver is open");
        });
    }
    drop(tx);
    let mut got = Vec::new();
    while let Some(message) = rx.recv().await {
        got.push(message);
    }
    got.sort_by_key(|m| m.0);
    got.into_iter().map(|m| m.1).collect()
}

pub fn run(text: &str, limit_ms: u64) -> String {
    let jobs = match parse_jobs(text) {
        Ok(jobs) => jobs,
        Err(e) => return format!("error: {e}\n"),
    };
    let outcomes = trpl::block_on(download_all(jobs, limit_ms));
    let mut out = String::new();
    let (mut done, mut failed, mut bytes) = (0, 0, 0u64);
    for outcome in &outcomes {
        match outcome {
            Outcome::Done(name, size) => {
                done += 1;
                bytes += *size as u64;
                out.push_str(&format!("{name}: {size} bytes\n"));
            }
            Outcome::Empty(name) => {
                failed += 1;
                out.push_str(&format!("{name}: empty\n"));
            }
            Outcome::TimedOut(name) => {
                failed += 1;
                out.push_str(&format!("{name}: timed out\n"));
            }
        }
    }
    out.push_str(&format!("{done} done, {failed} failed, {bytes} bytes\n"));
    out
}
