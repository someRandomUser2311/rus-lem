use std::time::Duration;
use trpl::Either;

pub async fn timeout_value<F: Future>(future: F, max_ms: u64) -> Result<F::Output, u64> {
    match trpl::select(future, trpl::sleep(Duration::from_millis(max_ms))).await {
        Either::Left(output) => Ok(output),
        Either::Right(()) => Err(0),
    }
}

pub async fn delayed(value: u32, delay_ms: u64) -> u32 {
    trpl::sleep(Duration::from_millis(delay_ms)).await;
    value
}
