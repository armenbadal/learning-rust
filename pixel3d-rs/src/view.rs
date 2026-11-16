use crate::matrix4x4::Matrix4x4;
use crate::vector3::Vector3;

// TODO 1.2: Թեստերով ամրագրել look_at-ի պայմանավորվածությունները։
// eye-ը դառնում է (0, 0, 0), իսկ target-ը՝ բացասական z ունեցող կետ։
// Ստուգել նաև տեղափոխված կամերան, eye == target և հայացքին համագիծ up դեպքերը։
pub fn look_at(eye: Vector3, target: Vector3, up: Vector3) -> Result<Matrix4x4,String> {
    let backward = (eye - target).normalized()
                           .ok_or("Կամերայի դիրքը և նպատակակետը նույնն են".to_string())?;
    let right = up.cross(backward).normalized()
                        .ok_or("Up վեկտորը համագիծ է հայացքի ուղղությանը".to_string())?;
    let up_prime = backward.cross(right);
    Ok(Matrix4x4::new([
        [right.x, right.y, right.z, -right.dot(eye)],
        [up_prime.x, up_prime.y, up_prime.z, -up_prime.dot(eye)],
        [backward.x, backward.y, backward.z, -backward.dot(eye)],
        [0.0, 0.0, 0.0, 1.0],
    ]))
}
