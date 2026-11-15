use crate::colour::Colour;
use crate::vector3::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Vector3,
    pub colour: Colour,
}

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub vertices: [Vertex; 3],
}
