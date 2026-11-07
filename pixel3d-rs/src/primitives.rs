use crate::framebuffer::Framebuffer;
use crate::rasterization::Rasterizer;
use crate::vector3::Vector3;

pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Rasterizer for Point {
    fn rasterize(&self, fb: &mut Framebuffer) {
        todo!()
    }
}

pub struct Segment {
    pub start: Point,
    pub end: Point,
}

pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}
