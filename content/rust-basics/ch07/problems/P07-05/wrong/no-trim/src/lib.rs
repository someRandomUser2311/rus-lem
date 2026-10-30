pub mod model {
    #[derive(Debug, Clone, PartialEq)]
    pub struct Ticket {
        name: String,
        price: u32,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Fare {
        Free,
        Paid(u32),
    }

    impl Fare {
        pub fn cents(&self) -> u32 {
            match self {
                Fare::Free => 0,
                Fare::Paid(n) => *n,
            }
        }
    }

    impl Ticket {
        pub fn new(name: &str, price: u32) -> Self {
            Self {
                name: name.to_string(),
                price,
            }
        }

        pub fn name(&self) -> &str {
            &self.name
        }

        pub fn fare(&self) -> Fare {
            if self.price == 0 {
                Fare::Free
            } else {
                Fare::Paid(self.price)
            }
        }
    }
}

pub fn summary(ticket: &model::Ticket) -> String {
    match ticket.fare() {
        model::Fare::Free => format!("{}: free", ticket.name()),
        model::Fare::Paid(n) => format!("{}: {n}", ticket.name()),
    }
}
