use crate::matrix4x4::Matrix4x4;
use crate::transform::{rotation, scaling, translation};

pub fn model_matrix(t: (f32, f32, f32), r: (f32, f32, f32), s: (f32, f32, f32)) -> Matrix4x4 {
    let tm = translation(t.0, t.1, t.2);
    let rm = rotation(r.0, r.1, r.2);
    let sm = scaling(s.0, s.1, s.2);
    tm * rm * sm
}
