use crate::model::model_matrix;
use crate::perspective::perspective_divide;
use crate::projection::projection_matrix;
use crate::vector3::Vector3;
use crate::vector4::Vector4;
use crate::view::view_matrix;

mod vector4;
mod matrix4x4;
mod transform;
mod view;
mod model;
mod projection;
mod perspective;
mod vector3;

fn graphics_pipeline(point: Vector3) -> Vector3 {
    let model = model_matrix(
        (0.0, 0.0, 0.0),
        (0.0, 0.0, 0.0),
        (1.0, 1.0, 1.0)
    );

    let view = view_matrix(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(1.0, 1.0, 1.0),
        Vector3::new(0.0, 0.0, 1.0)
    );

    let projection = projection_matrix(
        std::f32::consts::PI /3.0,
        600.0 / 400.0,
        20.0, 40.0
    );

    perspective_divide(projection * view * model * Vector4::from_vector3(point, 1.0)).unwrap()
}


fn main() {
    let p0 = Vector3::new(1.0, 2.0, 3.0);
    let _p1 = graphics_pipeline(p0);
}
