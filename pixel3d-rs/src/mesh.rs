use crate::colour::Colour;
use crate::vector3::Vector3;

// TODO 6.1: Ավելացնել միջանկյալ ClipVertex տիպ՝ Vector4 clip դիրքով և գույնով։
// Clipping-ը պետք է պահպանի երկու տվյալներն էլ՝ առանց սկզբնական Vertex-ը փոխելու։
// TODO 7.1: Քայլ 4.1-ի ScreenVertex-ն ընդլայնել inv_w և գունային տվյալներով։
// inv_w-ն հաշվարկել clipping-ից հետո՝ մինչև սկզբնական w-ն կորցնելը։
// Տվյալներն օգտագործել rasterization-ի perspective-correct ինտերպոլյացիայում։
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Vector3,
    pub colour: Colour,
}

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub vertices: [Vertex; 3],
}
