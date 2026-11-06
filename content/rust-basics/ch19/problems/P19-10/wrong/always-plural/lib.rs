#[derive(Debug, Clone, PartialEq)]
pub struct Order {
    pub id: u32,
    pub customer: String,
    pub items: Vec<String>,
    pub note: Option<String>,
}

pub fn items_and_customer(order: Order) -> (Vec<String>, String) {
    let Order { items, .. } = order;
    (items, order.customer)
}

pub fn summary(order: &Order) -> String {
    match order {
        Order {
            id, note: Some(n), ..
        } => format!("#{id}: {n}"),
        Order { id, items, .. } => {
            let unit = "items";
            format!("#{id}: {} {unit}", items.len())
        }
    }
}

pub fn notes(orders: Vec<Order>) -> Vec<String> {
    let mut out = Vec::new();
    for order in orders {
        if let Order { note: Some(n), .. } = order {
            out.push(n);
        }
    }
    out
}
