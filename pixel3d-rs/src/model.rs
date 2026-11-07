use crate::matrix4x4::Matrix4x4;
use crate::transform::{rotation, scaling, translation};
use crate::vector3::Vector3;

pub fn model_matrix(t: Vector3, r: Vector3, s: Vector3) -> Matrix4x4 {
    let tm = translation(t.x, t.y, t.z);
    let rm = rotation(r.x, r.y, r.z);
    let sm = scaling(s.x, s.y, s.z);
    tm * rm * sm
}
