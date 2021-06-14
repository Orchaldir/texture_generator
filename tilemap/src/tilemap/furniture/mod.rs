use texture_generation::math::aabb::AABB;
use texture_generation::math::point::Point;
use texture_generation::math::side::Side;
use texture_generation::math::size::Size;

pub mod map2d;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Furniture {
    pub style_id: usize,
    pub aabb: AABB,
    pub front_side: Side,
}

impl Furniture {
    pub fn new(style_id: usize, start: Point, size: Size, front: Side) -> Self {
        Furniture {
            style_id,
            aabb: AABB::new(start, size),
            front_side: front,
        }
    }

    pub fn without_front(style_id: usize, start: Point, size: Size) -> Self {
        Self::new(style_id, start, size, Side::Bottom)
    }
}
