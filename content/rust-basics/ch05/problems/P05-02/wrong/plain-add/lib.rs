pub struct Profile {
    pub username: String,
    pub email: String,
    pub active: bool,
    pub sign_in_count: u64,
}

pub fn build_profile(username: &str, email: &str) -> Profile {
    let username = username.trim().to_string();
    let email = email.trim().to_lowercase();
    Profile {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}

pub fn record_sign_in(p: &mut Profile) {
    p.sign_in_count = p.sign_in_count + 1;
    p.active = true;
}
