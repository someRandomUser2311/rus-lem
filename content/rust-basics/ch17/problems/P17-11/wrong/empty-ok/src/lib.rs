use std::time::Duration;
use trpl::Either;

#[derive(Debug, Clone, PartialEq)]
pub struct Fetch {
    pub name: String,
    pub delay_ms: u64,
    pub size: u32,
}

pub fn job(name: &str, delay_ms: u64, size: u32) -> Fetch {
    Fetch {
        name: name.to_string(),
        delay_ms,
        size,
    }
}

async fn fetch(job: Fetch, limit_ms: u64) -> Result<(String, u32), String> {
    let work = async {
        trpl::sleep(Duration::from_millis(job.delay_ms)).await;
        job.size
    };
    match trpl::select(work, trpl::sleep(Duration::from_millis(limit_ms))).await {
        Either::Left(size) => Ok((job.name, size)),
        Either::Right(()) => Err(format!("{} timed out", job.name)),
    }
}

pub async fn fetch_batch(jobs: Vec<Fetch>, limit_ms: u64) -> Vec<Result<(String, u32), String>> {
    let all: Vec<_> = jobs.into_iter().map(|job| fetch(job, limit_ms)).collect();
    trpl::join_all(all).await
}
