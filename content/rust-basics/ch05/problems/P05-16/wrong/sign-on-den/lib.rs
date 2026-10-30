#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fraction {
    pub num: i64,
    pub den: i64, // always > 0, and gcd(num, den) == 1
}

impl Fraction {
    pub fn new(num: i64, den: i64) -> Fraction {
        let mut a = num.abs();
        let mut b = den.abs();
        while b != 0 {
            let r = a % b;
            a = b;
            b = r;
        }
        let mut n = num / a;
        let mut d = den / a;
        Fraction { num: n, den: d }
    }

    pub fn add(&self, other: &Fraction) -> Fraction {
        Fraction::new(
            self.num * other.den + other.num * self.den,
            self.den * other.den,
        )
    }

    pub fn mul(&self, other: &Fraction) -> Fraction {
        Fraction::new(self.num * other.num, self.den * other.den)
    }

    pub fn display(&self) -> String {
        if self.den == 1 {
            format!("{}", self.num)
        } else {
            format!("{}/{}", self.num, self.den)
        }
    }
}
