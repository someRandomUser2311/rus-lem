#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Millimetres(pub u32);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Parcel {
    pub width: Millimetres,
    pub height: Millimetres,
}
impl Parcel {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width: Millimetres(width),
            height: Millimetres(height),
        }
    }
    pub fn area(&self) -> u64 {
        (self.width.0 * self.height.0) as u64
    }
    pub fn fits(&self, space: &Self) -> bool {
        (self.width.0 <= space.width.0 && self.height.0 <= space.height.0)
            || (self.height.0 <= space.width.0 && self.width.0 <= space.height.0)
    }
}
pub fn choose(parcel: &Parcel, spaces: &[Parcel]) -> usize {
    let mut best = spaces.len();
    let mut best_area = 0;
    for i in 0..spaces.len() {
        if parcel.fits(&spaces[i]) && (best == spaces.len() || spaces[i].area() < best_area) {
            best = i;
            best_area = spaces[i].area();
        }
    }
    best
}
