pub fn draft_and_final(text: String) -> (String, String) {
    let mut final_text = text;
    final_text.push('!');
    let draft = final_text.clone();
    (draft, final_text)
}
