use std::fmt::Debug;
use std::marker::Copy;
use std::ops::{Add, Sub, Mul, Div};

#[derive(Copy, Clone, Debug)]
pub struct Pt {
    pub x: i32,
    pub y: i32
}

pub fn pt(x: i32, y: i32) -> Pt { Pt{x: x, y: y} }

impl Add for Pt {
    type Output = Pt;
    fn add(self, oth: Pt) -> Pt { pt(self.x + oth.x, self.y + oth.y) }
}
impl Sub for Pt {
    type Output = Pt;
    fn sub(self, oth: Pt) -> Pt { pt(self.x - oth.x, self.y - oth.y) }
}
impl Mul for Pt {
    type Output = Pt;
    fn mul(self, oth: Pt) -> Pt { pt(self.x * oth.x, self.y * oth.y) }
}
impl Div for Pt {
    type Output = Pt;
    fn div(self, oth: Pt) -> Pt { pt(self.x / oth.x, self.y / oth.y) }
}

