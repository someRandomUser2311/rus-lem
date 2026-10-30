pub fn normalize_title(title: &str) -> String {
    let mut out = String::new();
    for word in title.split_whitespace() {
        if !out.is_empty() {
            out.push(' ');
        }
        out.push_str(word);
    }
    out
}
pub fn sort_key(title: &str) -> &str {
    if title.starts_with("The ") {
        &title[4..]
    } else if title.starts_with("A ") {
        &title[2..]
    } else if title.starts_with("An ") {
        &title[3..]
    } else {
        title
    }
}
pub fn title_matches(title: &str, query: &str) -> bool {
    title.to_lowercase().contains(&query.to_lowercase())
}
pub const LOAN_DAYS: u32 = 14;
pub const PREMIUM_LOAN_DAYS: u32 = 28;
pub const FEE_CAP_CENTS: u32 = 2_000;

pub fn due_day(borrow_day: u32, premium: bool) -> u32 {
    borrow_day
        + if premium {
            PREMIUM_LOAN_DAYS
        } else {
            LOAN_DAYS
        }
}
pub fn days_late(due_day: u32, return_day: u32) -> u32 {
    if return_day > due_day {
        return_day - due_day
    } else {
        0
    }
}
pub fn late_fee_cents(days_late: u32) -> u32 {
    if days_late >= 44 {
        FEE_CAP_CENTS
    } else if days_late <= 7 {
        days_late * 25
    } else {
        175 + (days_late - 7) * 50
    }
}
pub fn format_cents(cents: u32) -> String {
    format!("${}.{:02}", cents / 100, cents % 100)
}
use std::io;
fn main() {
    println!("==============================");
    println!("  Riverside Community Library");
    println!("==============================");
    println!("Open daily, 9:00 to 17:00");
    loop {
        println!("1) List books");
        println!("2) Borrow a book");
        println!("3) Return a book");
        println!("4) Quit");
        println!("Choose an option:");
        let mut choice = String::new();
        let bytes = io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        match bytes {
            0 => {
                println!("Goodbye!");
                break;
            }
            _ => {}
        }
        let choice: u32 = match choice.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please type a number from 1 to 4.");
                continue;
            }
        };
        match choice {
            1 => println!("You chose: List books"),
            2 => println!("You chose: Borrow a book"),
            3 => println!("You chose: Return a book"),
            4 => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Please type a number from 1 to 4."),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub year: u16,
    pub status: Status,
}
impl Book {
    pub fn new(id: u32, title: &str, author: &str, year: u16) -> Book {
        Book {
            id,
            title: normalize_title(title),
            author: author.trim().to_string(),
            year,
            status: Status::Available,
        }
    }
    pub fn summary(&self) -> String {
        format!("{} by {} ({})", self.title, self.author, self.year)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    pub id: u32,
    pub name: String,
    pub premium: bool,
    pub borrowed: u8,
}
impl Member {
    pub fn new(id: u32, name: &str, premium: bool) -> Member {
        Member {
            id,
            name: name.to_string(),
            premium,
            borrowed: 0,
        }
    }
    pub fn limit(&self) -> u8 {
        if self.premium { 6 } else { 3 }
    }
    pub fn can_borrow(&self) -> bool {
        self.borrowed < self.limit()
    }
    pub fn record_borrow(&mut self) -> bool {
        if !self.can_borrow() {
            return false;
        }
        self.borrowed += 1;
        true
    }
    pub fn record_return(&mut self) {
        if self.borrowed > 0 {
            self.borrowed -= 1;
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
    Available,
    Borrowed { member_id: u32, due_day: u32 },
    Lost,
}
impl Book {
    pub fn is_available(&self) -> bool {
        match self.status {
            Status::Available => true,
            _ => false,
        }
    }
    pub fn borrower(&self) -> Option<u32> {
        match self.status {
            Status::Borrowed {
                member_id,
                due_day: _,
            } => Some(member_id),
            _ => None,
        }
    }
    pub fn checkout(&mut self, member: &mut Member, today: u32) -> bool {
        if !self.is_available() || !member.can_borrow() {
            return false;
        }
        let due = due_day(today, member.premium);
        member.record_borrow();
        self.status = Status::Borrowed {
            member_id: member.id,
            due_day: due,
        };
        true
    }
    pub fn return_to_shelf(&mut self, member: &mut Member, today: u32) -> Option<u32> {
        None
    }
    pub fn mark_lost(&mut self) {}
}
