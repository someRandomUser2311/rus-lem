pub async fn both(a: u32, b: u32) -> (u64, u64) {
    let doubled = trpl::spawn_task(async move { a as u64 * 2 });
    let squared = async move { b as u64 * b as u64 };
    let (d, s) = trpl::join(doubled, squared).await;
    (s, d.expect("the task doesn't panic"))
}
