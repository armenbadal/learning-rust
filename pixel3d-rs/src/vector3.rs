use std::ops::{Add, Sub, Mul, Neg};

#[derive(Debug,Copy,Clone,PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector3 {
    pub const ZERO: Self = Self{x: 0.0, y: 0.0, z: 0.0};

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self{x, y, z}
    }

    pub fn length_squared(self) -> f32 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }
    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn dot(self, v: Self) -> f32 {
        self.x*v.x + self.y*v.y + self.z*v.z
    }

    pub fn cross(self, v: Self) -> Self {
        Self::new(
            self.y*v.z - self.z*v.y,
            self.z*v.x - self.x*v.z,
            self.x*v.y - self.y*v.x,
        )
    }

    pub fn normalized(self) -> Option<Self> {
        let len = self.length();
        if len == 0.0 {
            return None
        }

        Some(Self::new(self.x/len, self.y/len, self.z/len))
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, v: Self) -> Self::Output {
        Self::new(self.x+v.x, self.y+v.y, self.z+v.z)
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, v: Self) -> Self::Output {
        Self::new(self.x-v.x, self.y-v.y, self.z-v.z)
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, f: f32) -> Self::Output {
        Self::new(self.x * f, self.y * f, self.z * f)
    }
}
