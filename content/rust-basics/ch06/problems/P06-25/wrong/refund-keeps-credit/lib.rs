#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Purchase {
    Coin(u32),
    Buy { price: u32 },
    Refund,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Outcome {
    Credit(u32),
    Dispensed { change: u32 },
    Refunded(u32),
    Rejected,
}
pub fn apply(credit: &mut u32, action: Purchase) -> Outcome {
    match action {
        Purchase::Coin(n) => {
            if let Some(total) = credit.checked_add(n) {
                *credit = total;
                Outcome::Credit(total)
            } else {
                Outcome::Rejected
            }
        }
        Purchase::Buy { price } => {
            if price == 0 || price > *credit {
                return Outcome::Rejected;
            }
            let change = *credit - price;
            *credit = 0;
            Outcome::Dispensed { change }
        }
        Purchase::Refund => {
            let amount = *credit;
            Outcome::Refunded(amount)
        }
    }
}
