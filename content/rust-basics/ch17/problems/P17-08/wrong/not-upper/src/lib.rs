pub async fn task_messages(texts: Vec<String>) -> Vec<String> {
    let (tx, mut rx) = trpl::channel();
    for (i, text) in texts.into_iter().enumerate() {
        let tx = tx.clone();
        trpl::spawn_task(async move {
            let label = format!("{i}:{}", text);
            tx.send((i, label)).expect("the receiver is open");
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
