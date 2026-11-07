use crate::framebuffer::Framebuffer;

pub trait Rasterizer {
    fn rasterize(&self, fb: &mut Framebuffer);
}