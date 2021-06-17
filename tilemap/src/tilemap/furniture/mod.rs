use anyhow::{bail, Result};
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
    pub fn new(style_id: usize, start: Point, size: Size, front_side: Side) -> Result<Self> {
        if size.width() == 0 {
            bail!("Argument 'size.width' needs to be greater than 0");
        } else if size.height() == 0 {
            bail!("Argument 'size.height' needs to be greater than 0");
        }

        Ok(Furniture {
            style_id,
            aabb: AABB::new(start, size),
            front_side,
        })
    }

    pub fn without_front(style_id: usize, start: Point, size: Size) -> Result<Self> {
        Self::new(style_id, start, size, Side::Bottom)
    }
}
