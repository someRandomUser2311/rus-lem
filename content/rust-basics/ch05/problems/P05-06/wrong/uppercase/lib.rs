pub struct Rgb(pub u8, pub u8, pub u8);

pub fn to_hex(c: Rgb) -> String {
    format!("#{:02X}{:02X}{:02X}", c.0, c.1, c.2)
}

pub fn parse_hex(s: &str) -> (bool, Rgb) {
    let text = s.trim();
    let b = text.as_bytes();
    if b.len() != 4 && b.len() != 7 {
        return (false, Rgb(0, 0, 0));
    }
    if b[0] != b'#' {
        return (false, Rgb(0, 0, 0));
    }
    let mut v = [0_u8; 6];
    for i in 1..b.len() {
        let c = b[i];
        let x;
        if c >= b'0' && c <= b'9' {
            x = c - b'0';
        } else if c >= b'a' && c <= b'f' {
            x = c - b'a' + 10;
        } else if c >= b'A' && c <= b'F' {
            x = c - b'A' + 10;
        } else {
            return (false, Rgb(0, 0, 0));
        }
        v[i - 1] = x;
    }
    if b.len() == 4 {
        (true, Rgb(v[0] * 17, v[1] * 17, v[2] * 17))
    } else {
        (
            true,
            Rgb(v[0] * 16 + v[1], v[2] * 16 + v[3], v[4] * 16 + v[5]),
        )
    }
}

pub fn mix(a: Rgb, b: Rgb) -> Rgb {
    Rgb(
        ((a.0 as u16 + b.0 as u16) / 2) as u8,
        ((a.1 as u16 + b.1 as u16) / 2) as u8,
        ((a.2 as u16 + b.2 as u16) / 2) as u8,
    )
}
