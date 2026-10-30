pub struct Pixel {
    pub x: u32,
    pub y: u32,
    pub brightness: u8,
}

pub fn make_pixel(x: u32, y: u32, brightness: u8) -> Pixel {
    Pixel { x, y, brightness }
}

pub fn is_lit(p: &Pixel) -> bool {
    p.brightness > 1
}
