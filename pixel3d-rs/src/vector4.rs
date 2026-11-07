use std::ops::{Sub, Add, Mul};
use crate::vector3::Vector3;

#[derive(Debug,Copy,Clone,PartialEq)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl Vector4 {
    pub const ZERO: Self = Self {x: 0.0, y: 0.0, z: 0.0, w: 0.0};

    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self{x, y, z, w}
    }

    pub fn from_vector3(v: Vector3, w: f32) -> Self {
        Self::new(v.x, v.y, v.z, w)
    }
}

impl Default for Vector4 {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Sub for Vector4 {
    type Output = Self;

    fn sub(self, v: Self) -> Self::Output {
        Self::new(self.x-v.x, self.y-v.y, self.z-v.z, self.w-v.w)
    }
}

impl Add for Vector4 {
    type Output = Self;

    fn add(self, v: Self) -> Self::Output {
        Self::new(self.x+v.x, self.y+v.y, self.z+v.z, self.w+v.w)
    }
}

impl Mul<f32> for Vector4 {
    type Output = Vector4;

    fn mul(self, f: f32) -> Self::Output {
        Self::new(self.x * f, self.y * f, self.z * f, self.w * f)
    }
}
