pub struct Celsius(pub f64);

pub struct Fahrenheit(pub f64);

pub fn to_fahrenheit(c: Celsius) -> Fahrenheit {
    Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
}

pub fn to_celsius(f: Fahrenheit) -> Celsius {
    Celsius(f.0 - 32.0 * 5.0 / 9.0)
}
