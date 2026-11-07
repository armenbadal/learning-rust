use std::io::Write;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use crate::colour::Colour;
use crate::pixmap::PortablePixmap;


pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Colour>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self{
            width,
            height,
            pixels: vec![Colour::BLACK; width * height],
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn clear(&mut self, background_color: Colour) {
        self.pixels.clear();
        self.pixels.resize(self.width * self.height, background_color)
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, colour: Colour) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            let index = (y as usize) * self.width + (x as usize);
            self.pixels[index] = colour;
        }
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> Option<Colour> {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            let index = (y as usize) * self.width + (x as usize);
            Some(self.pixels[index])
        } else {
            None
        }
    }
}

impl PortablePixmap for Canvas {
    fn save(&self, filepath: impl AsRef<Path>) -> std::io::Result<()> {
        let file = File::create(filepath)?;
        let mut writer = BufWriter::new(file);

        writeln!(writer, "P3")?;
        writeln!(writer, "{} {}", self.width, self.height)?;
        writeln!(writer, "255")?;
        for row in 0..self.height {
            for column in 0..self.width {
                match self.get_pixel(column as i32, row as i32) {
                    Some(pixel) => writeln!(writer, "{} ", pixel)?,
                    None => continue,
                }
            }
            writeln!(writer, "")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::canvas::{Canvas, Colour};

    #[test]
    fn create_canvas() {
        let canvas = Canvas::new(10, 20);
        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);
        assert_eq!(canvas.pixels.len(), 10*20);
    }

    #[test]
    fn clear_canvas() {
        let mut canvas = Canvas::new(10, 10);
        canvas.clear(Colour::BLACK);
        assert_eq!(canvas.pixels.len(), 10*10);
        assert_eq!(canvas.get_pixel(0,0).unwrap(), Colour::BLACK);
    }
}


