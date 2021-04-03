use crate::interface::{Color3f, Point2f, TextureCoordinate};

#[derive(Copy, Clone)]
pub struct ColoredVertex {
    pub position: Point2f,
    pub color: Color3f,
}

#[derive(Copy, Clone)]
pub struct TexturedVertex {
    pub position: Point2f,
    pub color: Color3f,
    pub tc: TextureCoordinate,
}
