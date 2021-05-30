use texture_generation::math::size::Size;

pub mod map2d;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Furniture {
    pub style_id: usize,
    pub position: usize,
    pub size: Size,
}

impl Furniture {
    pub fn new(style_id: usize, position: usize, size: Size) -> Self {
        Furniture {
            style_id,
            position,
            size,
        }
    }
}
