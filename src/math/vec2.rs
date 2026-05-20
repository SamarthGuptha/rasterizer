use std::ops::{Add, Sub, Mul, Div};
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self {x, y}
    }

}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x+other.x, self.y+other.y)
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y-other.y)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Self::new(self.x*scalar, self.y*scalar)
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;
    fn div(self, scalar: f32) -> Self {
        Self::new(self.x/scalar, self.y/scalar)
    }
}