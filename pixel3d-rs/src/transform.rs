use crate::matrix4x4::Matrix4x4;

pub fn translation(dx: f32, dy: f32, dz: f32) -> Matrix4x4 {
    let mut identity = Matrix4x4::identity();
    identity.set(0, 3, dx);
    identity.set(1, 3, dy);
    identity.set(2, 3, dz);
    identity
}

pub fn scaling(sx: f32, sy: f32, sz: f32) -> Matrix4x4 {
    let mut identity = Matrix4x4::identity();
    identity.set(0, 0, sx);
    identity.set(1, 1, sy);
    identity.set(2, 2, sz);
    identity
}

fn rotation_by_x(angle: f32) -> Matrix4x4 {
    let cosa = angle.cos();
    let sina = angle.sin();
    let mut result = Matrix4x4::identity();
    result.set(1, 1, cosa);
    result.set(1, 2, -sina);
    result.set(2, 1, sina);
    result.set(2, 2, cosa);
    result
}

fn rotation_by_y(angle: f32) -> Matrix4x4 {
    let cosa = angle.cos();
    let sina = angle.sin();
    let mut result = Matrix4x4::identity();
    result.set(0, 0, cosa);
    result.set(0, 2, sina);
    result.set(2, 0, -sina);
    result.set(2, 2, cosa);
    result
}

fn rotation_by_z(angle: f32) -> Matrix4x4 {
    let cosa = angle.cos();
    let sina = angle.sin();
    let mut result = Matrix4x4::identity();
    result.set(0, 0, cosa);
    result.set(0, 1, -sina);
    result.set(1, 0, sina);
    result.set(1, 1, cosa);
    result
}

pub fn rotation(ax: f32, ay: f32, az: f32) -> Matrix4x4 {
    rotation_by_z(az) * rotation_by_y(ay) * rotation_by_x(ax)
}