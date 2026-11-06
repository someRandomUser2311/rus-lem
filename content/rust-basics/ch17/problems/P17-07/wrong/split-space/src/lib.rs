pub async fn messages(text: String) -> Vec<String> {
    let (tx, mut rx) = trpl::channel();
    let send = async move {
        for word in text.split(' ') {
            tx.send(word.to_string()).expect("the receiver is open");
        }
    };
    let receive = async {
        let mut got = Vec::new();
        while let Some(word) = rx.recv().await {
            got.push(word);
        }
        got
    };
    let ((), got) = trpl::join(send, receive).await;
    got
}
