use crate::vector3::Vector3;
use crate::vector4::Vector4;

pub fn perspective_divide(point: Vector4) -> Option<Vector3> {
    if point.w <= 1e-6 {
        return None
    }

    Some(Vector3::new(point.x/point.w, point.y/point.w, point.z/point.w))
}
