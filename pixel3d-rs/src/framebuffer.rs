use crate::canvas::Canvas;
use crate::colour::Colour;

pub struct Framebuffer<'a> {
    pub canvas: &'a mut Canvas,
    z_buffer: Vec<f32>,
}

impl<'a> Framebuffer<'a> {
    pub fn new(canvas: &'a mut Canvas) -> Self {
        let (width, height) = canvas.size();
        Self{
            canvas,
            z_buffer: vec![f32::INFINITY; width * height],
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, z: f32, c: Colour) {
        let (w, h) = self.canvas.size();
        if x >= 0 && x < w as i32 && y >= 0 && y < h as i32 {
            let idx = (y as usize) * w + (x as usize);
            if z < self.z_buffer[idx] {
                self.z_buffer[idx] = z;
                self.canvas.set_pixel(x, y, c);
            }
        }
    }
}
