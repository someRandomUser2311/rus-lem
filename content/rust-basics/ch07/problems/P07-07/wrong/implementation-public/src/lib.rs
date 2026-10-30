pub mod implementation {
    #[derive(Debug, Clone, PartialEq)]
    pub struct Stamp {
        pub label: String,
        pub code: u32,
    }

    pub fn stamp(label: &str, code: u32) -> Stamp {
        Stamp {
            label: label.trim().to_string(),
            code,
        }
    }
}

pub use implementation::{Stamp as Ticket, stamp as make_ticket};

pub mod prelude {
    pub use crate::{Ticket, make_ticket};
}

pub mod report {
    use crate::prelude::*;

    pub fn display(ticket: &Ticket) -> String {
        format!("{}#{}", ticket.label, ticket.code)
    }
}
