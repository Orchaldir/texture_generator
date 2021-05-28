use texture_generation::math::size::Size;

pub mod map2d;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Furniture {
    style_id: usize,
    position: usize,
    size: Size,
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
