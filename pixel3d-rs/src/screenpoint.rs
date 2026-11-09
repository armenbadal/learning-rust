use crate::colour::Colour;
use crate::framebuffer::Framebuffer;
use crate::rasterization::Rasterizer;

pub struct ScreenPoint {
    pub x: i32,
    pub y: i32,
    pub z: f32,
    pub c: Colour
}

impl Rasterizer for ScreenPoint {
    fn rasterize(&self, fb: &mut Framebuffer) {
        fb.set_pixel(self.x, self.y, self.z, self.c)
    }
}
