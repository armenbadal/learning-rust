use crate::matrix4x4::Matrix4x4;
use crate::vector3::Vector3;
use crate::vector4::Vector4;

fn look_at(eye: Vector3, target: Vector3, up: Vector3) -> Matrix4x4 {
    let z = (eye - target).normalized().unwrap();
    let x = up.cross(z).normalized().unwrap();
    let y = z.cross(x);
    Matrix4x4::new([
        [x.x, x.y, x.z, -x.dot(eye)],
        [y.x, y.y, y.z, -y.dot(eye)],
        [z.x, z.y, z.z, -z.dot(eye)],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn view_matrix(eye: Vector3, target: Vector3, up: Vector3) -> Matrix4x4 {
    look_at(eye, target, up)
}
