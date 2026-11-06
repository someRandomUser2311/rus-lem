pub async fn many(values: Vec<u32>) -> Vec<u64> {
    let jobs: Vec<_> = values
        .into_iter()
        .map(|v| async move {
            trpl::yield_now().await;
            v as u64 + 3
        })
        .collect();
    trpl::join_all(jobs).await
}
