use std::pin::Pin;

pub async fn mixed_jobs(values: Vec<u32>) -> Vec<u64> {
    let mut jobs: Vec<Pin<Box<dyn Future<Output = u64>>>> = Vec::new();
    for v in values {
        if v % 2 == 0 {
            jobs.push(Box::pin(async move { v as u64 * 2 }));
        } else {
            jobs.push(Box::pin(async move {
                trpl::yield_now().await;
                v as u64 * 2
            }));
        }
    }
    trpl::join_all(jobs).await
}
