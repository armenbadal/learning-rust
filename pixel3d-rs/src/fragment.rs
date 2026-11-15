use crate::colour::Colour;

#[derive(Debug, Clone, Copy)]
pub struct Fragment {
    pub x: i32,
    pub y: i32,
    pub depth: f32,
    pub colour: Colour,
}
