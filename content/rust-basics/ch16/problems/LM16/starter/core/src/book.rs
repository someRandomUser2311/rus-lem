use std::fmt;

use crate::{
    fees::{days_late, due_day, late_fee_cents},
    member::Member,
    text::normalize_title,
};
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
        let Status::Borrowed { member_id, due_day } = self.status else {
            return None;
        };
        if member.id != member_id {
            return None;
        }
        let fee = late_fee_cents(days_late(due_day, today));
        self.status = Status::Available;
        member.record_return();
        Some(fee)
    }
    pub fn mark_lost(&mut self) {
        self.status = Status::Lost;
    }
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.summary())
    }
}
