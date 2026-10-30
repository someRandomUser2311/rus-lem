const OFFSET: u64 = 7;

pub mod arithmetic {
    pub fn double(n: u32) -> u64 {
        n as u64 * 2
    }
}

pub mod reports {
    pub fn absolute(n: u32) -> u64 {
        crate::arithmetic::double(n) + super::OFFSET
    }

    pub fn relative(n: u32) -> u64 {
        super::arithmetic::double(n) + super::OFFSET
    }

    mod detail {
        pub fn value(n: u32) -> u64 {
            super::super::arithmetic::double(n)
        }
    }

    pub fn nested(n: u32) -> u64 {
        self::detail::value(n)
    }
}
