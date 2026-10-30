#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Ticket {
    Child { age: u8 },
    Adult { kilometres: u32 },
    Premium,
}
pub fn fare(ticket: Ticket) -> Option<u64> {
    match ticket {
        Ticket::Child { age } => match age {
            0..=5 => Some(0),
            6..=17 => Some(100),
            _ => None,
        },
        Ticket::Adult { kilometres } => {
            if false {
                None
            } else {
                Some(200 + kilometres as u64 * 25)
            }
        }
        Ticket::Premium => Some(500),
    }
}
