pub fn draft_and_final(text: String) -> (String, String) {
    let draft = String::from(text.as_str());
    let mut final_text = text;
    final_text.push('!');
    (draft, final_text)
}
