use std::f32::consts::PI;
use crate::model::model_matrix;
use crate::perspective::perspective_divide;
use crate::projection::projection_matrix;
use crate::vector3::Vector3;
use crate::vector4::Vector4;
use crate::view::{look_at};

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
mod primitives;
mod rasterization;
mod framebuffer;

fn graphics_pipeline(point: Vector3) -> Result<Vector3,String> {
    let model = model_matrix(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(1.0, 1.0, 1.0)
    );

    let view = look_at(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(1.0, 1.0, 1.0),
        Vector3::new(0.0, 0.0, 1.0)
    )?;

    let projection = projection_matrix(
        PI /3.0,
        600.0 / 400.0,
        20.0, 40.0
    )?;

    let p = projection * view * model * Vector4::from_vector3(point, 1.0);
    let p = perspective_divide(p).ok_or(String::from("Could not perspective divide"))?;
    Ok(p)
}


fn main() {
    let p0 = Vector3::new(1.0, 2.0, 3.0);
    let _p1 = graphics_pipeline(p0);
}
