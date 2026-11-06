pub type Res<T> = Result<T, String>;

pub fn parse_age(text: &str) -> Res<u8> {
    text
        .parse::<u8>()
        .map_err(|_| format!("not an age: {}", text.trim()))
}
