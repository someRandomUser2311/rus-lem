pub async fn decorated(text: String) -> String {
    format!("[{}]", text)
}

pub fn shout(text: String) -> impl Future<Output = String> {
    async move { text.to_uppercase() }
}
