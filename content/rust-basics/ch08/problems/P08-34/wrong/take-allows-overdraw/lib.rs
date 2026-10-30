use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Operation {
    Add(String, u32),
    Take(String, u32),
    Remove(String),
}

pub fn transactions(ops: Vec<Operation>) -> (Vec<bool>, Vec<(String, u32)>) {
    let mut stock: HashMap<String, u32> = HashMap::new();
    let mut flags = Vec::new();
    for op in ops {
        let ok = match op {
            Operation::Add(name, q) => {
                let count = stock.entry(name).or_insert(0);
                *count = count.saturating_add(q);
                true
            }
            Operation::Take(name, q) => match stock.get_mut(&name) {
                Some(count) => {
                    if *count > 0 {
                        *count = count.saturating_sub(q);
                        true
                    } else {
                        false
                    }
                }
                None => false,
            },
            Operation::Remove(name) => stock.remove(&name).is_some(),
        };
        flags.push(ok);
    }
    let mut out = Vec::new();
    for (name, count) in stock {
        out.push((name, count));
    }
    out.sort();
    (flags, out)
}
