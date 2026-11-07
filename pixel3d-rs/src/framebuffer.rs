use crate::canvas::Canvas;

pub struct Framebuffer {
    pub canvas: Canvas,
    z_buffer: Vec<f32>,
}

impl Framebuffer {
    pub fn new(canvas: Canvas) -> Self {
        Self{
            canvas,
            z_buffer: Vec::new(),
        }
    }
}
