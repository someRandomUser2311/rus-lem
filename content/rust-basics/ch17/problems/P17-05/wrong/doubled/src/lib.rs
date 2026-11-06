pub async fn task_value(n: u32) -> u64 {
    let handle = trpl::spawn_task(async move { n as u64 * 2 });
    handle.await.expect("the task doesn't panic")
}
