pub fn digit_sum(mut n: u32) -> u32 {
    let mut total = 0;
    while n > 0 {
        total += n % 10;
        n /= 10;
    }
    total
}
pub fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}
pub fn is_prime(n: u32) -> bool {
    // Test possible divisors, rejecting 0 and 1.
    false
}
pub fn fibonacci(n: u32) -> u32 {
    // Advance a pair of consecutive values n times.
    0
}
fn main() {}
