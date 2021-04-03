pub mod app;
pub mod input;
pub mod rendering;
pub mod window;

pub type Color3f = (f32, f32, f32);
pub type Point2f = (f32, f32);
pub type Size2d = (u32, u32);
pub type TextureId = usize;
pub type TextureCoordinate = (f32, f32);

pub const RED: Color3f = (1.0, 0.0, 0.0);
pub const GREEN: Color3f = (0.0, 1.0, 0.0);
pub const BLUE: Color3f = (0.0, 0.0, 1.0);
pub const YELLOW: Color3f = (1.0, 1.0, 0.0);
pub const PINK: Color3f = (1.0, 0.0, 0.5);
pub const WHITE: Color3f = (1.0, 1.0, 1.0);
