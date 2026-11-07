use std::f32::consts::PI;
use crate::matrix4x4::Matrix4x4;

pub fn projection_matrix(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Result<Matrix4x4, String> {
    if fov <= 0.0 || fov >= PI {
        return Err("fov-ը պետք է լինի 0-ից PI միջակայքում".into());
    }

    if aspect_ratio <= 0.0 {
        return Err("aspect_ratio-ն պետք է լինի դրական թիվ".into());
    }

    if near <= 0.0 {
        return Err("near-ը պետք է լինի դրական թիվ (near > 0)".into());
    }

    if far <= near {
        return Err("far-ը պետք է մեծ լինի near-ից (far > near)".into());
    }

    let scale_factor = 1.0 / (fov / 2.0).tan();

    let mut result = Matrix4x4::zero();

    result.set(0, 0, scale_factor / aspect_ratio);
    result.set(1, 1, scale_factor);

    result.set(2, 2, -(far + near) / (far - near));
    result.set(2, 3, -2.0 * far * near / (far - near));

    result.set(3, 2, -1.0);

    Ok(result)
}
