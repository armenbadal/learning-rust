use std::f32::consts::PI;
use crate::canvas::Canvas;
use crate::colour::Colour;
use crate::framebuffer::Framebuffer;
use crate::model::model_matrix;
use crate::perspective::perspective_divide;
use crate::pixmap::PortablePixmap;
use crate::projection::projection_matrix;
use crate::rasterization::Rasterizer;
use crate::screenpoint::ScreenPoint;
use crate::vector3::Vector3;
use crate::vector4::Vector4;
use crate::view::look_at;

mod vector4;
mod matrix4x4;
mod transform;
mod view;
mod model;
mod projection;
mod perspective;
mod vector3;
mod canvas;
mod pixmap;
mod colour;
mod rasterization;
mod framebuffer;
mod pipeline;
mod screenpoint;

fn demo_first() {
    const WIDTH: usize = 10;
    const HEIGHT: usize = 10;
    let mut canvas = Canvas::new(10, 10);
    let mut fb = Framebuffer::new(&mut canvas);

    let p0 = Vector3::new(0.0, 0.0, 25.0);

    let model = model_matrix(
        Vector3::ZERO,
        Vector3::ZERO,
        Vector3::new(1.0, 1.0, 1.0)
    );
    let view = look_at(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 1.0),
        Vector3::new(0.0, 1.0, 0.0),
    ).expect("Չհաջողվեց կառուցել view մատրիցը");
    let proj = projection_matrix(
        PI / 3.0,
        (WIDTH as f32) / (HEIGHT as f32),
        20.0,
        40.0,
    ).expect("Չհաջողվեց կառուցել projection մատրիցը");

    let clip = proj * view * model * Vector4::from_vector3(p0, 1.0);
    let ndc = perspective_divide(clip).expect("Perspective divide failed");

    // 3. Viewport Transform (NDC -> Screen Pixel Coordinates)
    let screen_x = ((ndc.x + 1.0) / 2.0 * (WIDTH as f32 - 1.0)).round() as i32;
    let screen_y = ((1.0 - ndc.y) / 2.0 * (HEIGHT as f32 - 1.0)).round() as i32;

    let point = ScreenPoint {
        x: screen_x,
        y: screen_y,
        z: ndc.z,
        c: Colour::new(255, 255, 255),
    };
    point.rasterize(&mut fb);

    canvas.save("./demo_first.ppm").expect("Չհաջողվեց պահպանել demo_first.ppm-ը");
}


fn main() {
    demo_first();
}
