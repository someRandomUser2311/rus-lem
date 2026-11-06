use std::time::Duration;
use trpl::Either;

pub async fn timed_receive(values: Vec<u32>, gap_ms: u64, limit_ms: u64) -> (Vec<u32>, bool) {
    let (tx, mut rx) = trpl::channel();
    trpl::spawn_task(async move {
        for (i, value) in values.into_iter().enumerate() {
            if i > 0 {
                trpl::sleep(Duration::from_millis(gap_ms)).await;
            }
            tx.send(value).expect("the receiver is open");
        }
    });
    let mut got = Vec::new();
    loop {
        match trpl::select(rx.recv(), trpl::sleep(Duration::from_millis(limit_ms))).await {
            Either::Left(Some(value)) => got.push(value),
            Either::Left(None) => return (got, true),
            Either::Right(()) => return (got, true),
        }
    }
}
