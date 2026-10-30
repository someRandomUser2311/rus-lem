use crate::Line;

pub fn total(lines: &[Line]) -> Option<u32> {
    let mut sum: u32 = 0;
    for line in lines {
        let Some(cost) = line.dish.price().checked_mul(line.quantity) else {
            return None;
        };
        let Some(next) = sum.checked_add(cost) else {
            return None;
        };
        sum = next;
    }
    Some(sum)
}

pub fn receipt(lines: &[Line]) -> Option<String> {
    let Some(cents) = total(lines) else {
        return None;
    };
    Some(format!("Total: ${}.{:02}", cents / 100, cents % 100))
}
