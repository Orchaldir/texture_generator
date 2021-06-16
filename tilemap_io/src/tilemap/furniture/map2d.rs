use crate::tilemap::furniture::FurnitureDefinition;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use texture_generation::math::size::Size;
use tilemap::tilemap::furniture::map2d::FurnitureMap2d;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FurnitureMap2dDefinition {
    size: Size,
    furniture: Vec<FurnitureDefinition>,
}

impl FurnitureMap2dDefinition {
    pub fn convert_from_map(map: &FurnitureMap2d) -> Self {
        Self {
            size: map.get_size().clone(),
            furniture: map
                .get_all_furniture()
                .iter()
                .map(|(id, furniture)| FurnitureDefinition::convert_from(furniture, *id))
                .collect(),
        }
    }

    pub fn convert_to_map(self) -> FurnitureMap2d {
        FurnitureMap2d::new(
            self.size,
            self.furniture
                .into_iter()
                .fold(HashMap::new(), |mut map, furniture| {
                    map.insert(furniture.id, furniture.convert_to());
                    map
                }),
        )
    }
}

#[cfg(test)]
mod tests_conversion {
    use super::*;
    use texture_generation::math::point::Point;
    use texture_generation::math::side::Side::*;
    use tilemap::tilemap::furniture::Furniture;

    #[test]
    fn test_empty() {
        let mut furniture_map = FurnitureMap2d::empty(Size::new(5, 10));
        furniture_map.add(Furniture::new(2, Point::new(2, 2), Size::new(3, 2), Right));
        furniture_map.add(Furniture::new(3, Point::new(5, 2), Size::new(1, 2), Left));
        furniture_map.add(Furniture::new(1, Point::new(2, 7), Size::new(6, 1), Top));

        assert_eq!(
            FurnitureMap2dDefinition::convert_from_map(&furniture_map).convert_to_map(),
            furniture_map
        );
    }
}
