use anyhow::Result;
use serde::{Deserialize, Serialize};
use texture_generation::math::point::Point;
use texture_generation::math::side::Side;
use texture_generation::math::size::Size;
use tilemap::tilemap::furniture::Furniture;

pub mod map2d;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FurnitureDefinition {
    id: usize,
    style_id: usize,
    start: Point,
    size: Size,
    front_side: Side,
}

impl FurnitureDefinition {
    pub fn convert_from(furniture: &Furniture, id: usize) -> Self {
        Self {
            id,
            style_id: furniture.style_id,
            start: furniture.aabb.start(),
            size: furniture.aabb.size(),
            front_side: furniture.front_side,
        }
    }

    pub fn convert_to(&self) -> Result<Furniture> {
        Furniture::new(self.style_id, self.start, self.size, self.front_side)
    }
}
