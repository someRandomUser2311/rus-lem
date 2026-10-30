use std::io;
pub fn digit_sum(mut n: u32) -> u32 {
    let mut total = 0;
    while n > 0 {
        total += n % 10;
        n /= 10;
    }
    total
}
pub fn reverse_integer(mut n: u32) -> u64 {
    let mut result = 0_u64;
    while n > 0 {
        result = result * 10 + (n % 10) as u64;
        n /= 10;
    }
    result
}
pub fn is_prime(n: u32) -> bool {
    if n < 1 {
        return false;
    }
    let mut d = 2;
    while d <= n / d {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}
fn main() {
    println!("Number utilities");
    loop {
        println!("Enter a number or quit:");
        let mut line = String::new();
        if io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line")
            == 0
        {
            println!("Goodbye!");
            break;
        }
        let text = line.trim();
        if text == "quit" {
            println!("Goodbye!");
            break;
        }
        let n: u32 = match text.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Input error");
                continue;
            }
        };
        if n > 1_000_000 {
            println!("Input error");
            continue;
        }
        println!("Number: {n}");
        println!("Digit sum: {:02}", digit_sum(n));
        println!("Prime: {}", is_prime(n));
        println!("Reversed: {}", reverse_integer(n));
    }
}
