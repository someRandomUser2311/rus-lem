pub trait Item {
    fn name(&self) -> String;
    fn unit_cents(&self) -> u64;
    fn quantity(&self) -> u32 {
        1
    }
    fn line_cents(&self) -> Option<u64> {
        self.unit_cents().checked_mul(self.quantity() as u64)
    }
}

pub struct Book {
    pub title: String,
    pub cents: u64,
}

pub struct Bulk {
    pub name: String,
    pub cents: u64,
    pub count: u32,
}

impl Item for Book {
    fn name(&self) -> String {
        self.title.clone()
    }

    fn unit_cents(&self) -> u64 {
        self.cents
    }
}

impl Item for Bulk {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn unit_cents(&self) -> u64 {
        self.cents
    }

    fn quantity(&self) -> u32 {
        self.count
    }
}

pub fn money(cents: u64) -> String {
    format!("${}.{:02}", cents / 100, cents % 100)
}

pub struct Cart {
    items: Vec<Box<dyn Item>>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart { items: Vec::new() }
    }

    pub fn add(&mut self, item: Box<dyn Item>) {
        self.items.push(item);
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn total_cents(&self) -> Option<u64> {
        let mut total: u64 = 0;
        for item in &self.items {
            total = total.saturating_add(item.line_cents()?);
        }
        Some(total)
    }

    pub fn lines(&self) -> Vec<String> {
        self.items
            .iter()
            .map(|item| {
                let price = match item.line_cents() {
                    Some(cents) => money(cents),
                    None => String::from("overflow"),
                };
                format!("{} x{}: {price}", item.name(), item.quantity())
            })
            .collect()
    }

    pub fn priciest(&self) -> Option<String> {
        let mut best: Option<(u128, String)> = None;
        for item in &self.items {
            let cents = item.line_cents().map_or(u128::MAX, u128::from);
            if best.as_ref().is_none_or(|(b, _)| cents > *b) {
                best = Some((cents, item.name()));
            }
        }
        best.map(|(_, name)| name)
    }
}
