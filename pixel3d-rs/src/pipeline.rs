use crate::colour::Colour;
use crate::fragment::Fragment;
use crate::framebuffer::Framebuffer;
use crate::matrix4x4::Matrix4x4;
use crate::mesh::{Triangle, Vertex};
use crate::perspective::perspective_divide;
use crate::screenpoint::ScreenPoint;
use crate::vector3::Vector3;
use crate::vector4::Vector4;

// TODO 9.2: Վավերացնել viewport-ը՝ լայնությունն ու բարձրությունը պետք է դրական լինեն։
// Նկարելիս ստուգել նաև համապատասխանությունը framebuffer-ի չափերին։
// Ստուգում՝ անվավեր չափերը մշակվում են բացահայտ՝ առանց սխալ ինդեքսավորման։
pub type Viewport = (usize, usize);

pub struct Pipeline {
    pub model: Matrix4x4,
    pub view: Matrix4x4,
    pub projection: Matrix4x4,
    pub viewport: Viewport,
}

impl Pipeline {
    pub fn new(model: Matrix4x4, view: Matrix4x4, projection: Matrix4x4, viewport: Viewport) -> Self {
        Self { model, view, projection, viewport}
    }

    pub fn vertex_to_clip(&self, vertex: &Vertex) -> Vector4 {
        self.projection * self.view * self.model * Vector4::from_vector3(vertex.position, 1.0)
    }

    pub fn clip_to_ndc(&self, clip: Vector4) -> Option<Vector3> {
        perspective_divide(clip)
    }

    // TODO 4.1: Եռանկյան համար վերադարձնել f32 դիրքեր ունեցող ScreenVertex։
    // ScreenPoint-ի ամբողջ թվային դիրքերը պահել առանձին կետ նկարելու համար։
    // Ընտրել viewport mapping՝ համաձայն պիքսելների (x + 0.5, y + 0.5) կենտրոնների,
    // և չկլորացնել եռանկյան գագաթները մինչև coverage-ի ստուգումը։
    pub fn ndc_to_screen(&self, ndc: Vector3, colour: Colour) -> ScreenPoint {
        ScreenPoint {
            x: ((ndc.x + 1.0) / 2.0 * (self.viewport.0 as f32 - 1.0)).round() as i32,
            y: ((1.0 - ndc.y) / 2.0 * (self.viewport.1 as f32 - 1.0)).round() as i32,
            z: ndc.z,
            c: colour
        }
    }

    // TODO 2.2: Հեռացնել perspective divide-ի արդյունքի unwrap()-ը։
    // Մինչև clipping-ը՝ անվավեր գագաթի դեպքում անվտանգ բաց թողնել եռանկյունը։
    // Քայլ 6-ում փոխել API-ն՝ կտրումից հետո հնարավոր են 0, 1 կամ մի քանի եռանկյուններ։
    // Ստուգում՝ կամերայի հետևի գագաթը չի առաջացնում panic։
    pub fn triangle_to_screen(&self, triangle: &Triangle) -> [ScreenPoint; 3] {
        let f = |v| self.ndc_to_screen(self.clip_to_ndc(self.vertex_to_clip(v)).unwrap(), v.colour);
        [f(&triangle.vertices[0]), f(&triangle.vertices[1]), f(&triangle.vertices[2])]
    }

    // TODO 4.3: Միացնել triangle -> screen vertices -> rasterization շղթան։
    // Սկզբում աշխատել ամբողջությամբ տեսադաշտի ներսում գտնվող եռանկյուններով։
    // Քայլ 5-ում ռաստերացումից ստացած Fragment-ները փոխանցել write_fragment-ին,
    // իսկ քայլ 6-ում perspective divide-ից առաջ ավելացնել clipping։
    pub fn rasterize_triangle(&self, triangle: &Triangle, fb: &mut Framebuffer) {
        todo!()
    }

    // TODO 5.1: Fragment-ի դիրքը, depth-ն ու colour-ը փոխանցել Framebuffer::set_pixel-ին։
    // Խորության համեմատությունը պահել Framebuffer-ում՝ մեկ ընդհանուր տեղում։
    pub fn write_fragment(&self, fragment: Fragment, fb: &mut Framebuffer) {
        todo!()
    }
}
