use crate::vector3::Vector3;
use crate::vector4::Vector4;

// TODO 6.2: Առանձին primitive clipping փուլում ներդնել Sutherland–Hodgman ալգորիթմը։
// Աշխատել clip կոորդինատներով՝ perspective_divide-ից ԱՌԱՋ։
// Սկսել near հարթությունից՝ z + w >= 0. w > 0-ը near clipping չէ։
// Նոր հատման գագաթների clip դիրքն ու գույնը գծային ինտերպոլացնել նույն t-ով։
// Ստացված բազմանկյունը տրոհել եռանկյունների (triangle fan)։
// Ստուգել ամբողջությամբ ներսում/դրսում և near-ը հատող 1 կամ 2 գագաթներով դեպքերը։
// TODO 6.3: Ընդլայնել clipping-ը բոլոր վեց հարթությունների վրա՝
// x + w >= 0, w - x >= 0, y + w >= 0, w - y >= 0, z + w >= 0, w - z >= 0։
// Ստուգում՝ յուրաքանչյուր հարթությունը հատող եռանկյունը պահպանում է տեսանելի մասը,
// իսկ կամերայի հետևի կամ հարթության վրա գտնվող գագաթները չեն առաջացնում NaN/panic։

pub fn perspective_divide(point: Vector4) -> Option<Vector3> {
    if point.w <= 1e-6 {
        return None
    }

    Some(Vector3::new(point.x/point.w, point.y/point.w, point.z/point.w))
}
