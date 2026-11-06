#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Login { user: String, admin: bool },
    Logout { user: String },
    Purchase { user: String, cents: u64 },
    Error { code: u16, message: String },
}

#[derive(Debug, Default, PartialEq)]
pub struct Summary {
    pub logins: u32,
    pub admins: Vec<String>,
    pub revenue: u64,
    pub big_spenders: Vec<String>,
    pub errors: Vec<String>,
}

pub fn reduce(events: Vec<Event>) -> Summary {
    let mut s = Summary::default();
    for event in events {
        match event {
            Event::Login { user, admin: true } => {
                s.logins = s.logins.saturating_add(1);
                s.admins.push(user);
            }
            Event::Login { .. } => s.logins = s.logins.saturating_add(1),
            Event::Logout { .. } => {}
            Event::Purchase {
                user,
                cents: cents @ 10_000..,
            } => {
                s.revenue = s.revenue.saturating_add(cents);
                s.big_spenders.push(user);
            }
            Event::Purchase { cents, .. } => s.revenue = s.revenue.saturating_add(cents),
            Event::Error {
                code: code @ 500..=599,
                message,
            } => s.errors.push(format!("{code} server: {message}")),
            Event::Error {
                code: code @ 400..=499,
                ..
            } => s.errors.push(format!("{code} client")),
            Event::Error { .. } => {}
        }
    }
    s
}
