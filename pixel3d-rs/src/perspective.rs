use crate::vector3::Vector3;
use crate::vector4::Vector4;

// TODO 4: Homogeneous clipping (Sutherland–Hodgman)
// Perspective divide-ից առաջ կտրել պրիմիտիվները near plane-ով (w > 0)։
// Հիմա ֆունկցիան պարզապես վերադարձնում է None, երբ w <= 1e-6։

pub fn perspective_divide(point: Vector4) -> Option<Vector3> {
    if point.w <= 1e-6 {
        return None
    }

    Some(Vector3::new(point.x/point.w, point.y/point.w, point.z/point.w))
}
