pub fn sum_valid(lines: &[&str]) -> u32 {
    let mut total: u32 = 0;
    for line in lines {
        let n: u32 = match line.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };
        total = total.saturating_add(n);
    }
    total
}

pub fn first_big(values: &[i32], limit: i32) -> Option<i32> {
    let mut i = 0;
    loop {
        if i == values.len() {
            break None;
        }
        if values[i] >= limit {
            break Some(values[i]);
        }
        i += 1;
    }
}

pub fn parse_all(lines: &[&str]) -> Option<Vec<u32>> {
    let mut out = Vec::new();
    for line in lines {
        let Ok(n) = line.trim().parse::<u32>() else {
            return None;
        };
        out.push(n);
    }
    Some(out)
}
