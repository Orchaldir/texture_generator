use crate::tilemap::Side;
use texture_generation::math::size::Size;

pub mod map2d;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Furniture {
    pub style_id: usize,
    pub position: usize,
    pub size: Size,
    pub front_side: Side,
}

impl Furniture {
    pub const fn new(style_id: usize, position: usize, size: Size, front: Side) -> Self {
        Furniture {
            style_id,
            position,
            size,
            front_side: front,
        }
    }

    pub fn without_front(style_id: usize, position: usize, size: Size) -> Self {
        Self::new(style_id, position, size, Side::Bottom)
    }
}
