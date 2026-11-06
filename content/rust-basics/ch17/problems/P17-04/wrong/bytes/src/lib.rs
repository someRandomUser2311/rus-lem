async fn length(text: String) -> usize {
    text.len()
}

pub fn lengths(texts: Vec<String>) -> Vec<usize> {
    trpl::block_on(async {
        let mut out = Vec::new();
        for text in texts {
            out.push(length(text).await);
        }
        out
    })
}
