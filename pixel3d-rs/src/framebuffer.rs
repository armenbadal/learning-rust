use crate::canvas::Canvas;
use crate::colour::Colour;

pub struct Framebuffer<'a> {
    pub canvas: &'a mut Canvas,
    z_buffer: Vec<f32>,
}

impl<'a> Framebuffer<'a> {
    // TODO 8.2: Ավելացնել clear(background), որը մաքրում է canvas-ը և
    // z_buffer-ը նորից լցնում f32::INFINITY-ով՝ յուրաքանչյուր կադրի սկզբում։
    // Ստուգում՝ առաջին կադրի մոտ կետը չի խանգարում երկրորդ կադրի հեռու կետին։
    pub fn new(canvas: &'a mut Canvas) -> Self {
        let (width, height) = canvas.size();
        Self{
            canvas,
            z_buffer: vec![f32::INFINITY; width * height],
        }
    }

    // TODO 5.4: Թեստավորել depth test-ը՝ փոքր z-ն հաղթում է անկախ գրելու հերթից,
    // հավասար z-ի դեպքում մնում է առաջինը, սահմաններից դուրս գրառումն անտեսվում է։
    // TODO 9.3: Բացահայտ մերժել ոչ վերջավոր և [-1, 1]-ից դուրս NDC խորությունները։
    // Սահմանային արժեքների մշակումը համաձայնեցնել clipping-ի թվային ճշտության հետ։
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
