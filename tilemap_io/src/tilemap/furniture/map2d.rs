use crate::tilemap::furniture::FurnitureDefinition;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use texture_generation::definition::{read, write};
use texture_generation::math::size::Size;
use tilemap::tilemap::furniture::map2d::FurnitureMap2d;

pub const FURNITURE_MAP_FILE_ENDING: &str = "ofm";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FurnitureMap2dDefinition {
    size: Size,
    furniture: Vec<FurnitureDefinition>,
}

impl FurnitureMap2dDefinition {
    pub fn convert_from_map(map: &FurnitureMap2d) -> Self {
        let mut furniture: Vec<FurnitureDefinition> = map
            .get_all_furniture()
            .iter()
            .map(|(id, furniture)| FurnitureDefinition::convert_from(furniture, *id))
            .collect();
        furniture.sort_by(|a, b| a.id.cmp(&b.id));
        Self {
            size: *map.get_size(),
            furniture,
        }
    }

    pub fn convert_to_map(self) -> Result<FurnitureMap2d> {
        let mut hashmap = HashMap::new();

        for (i, definition) in self.furniture.iter().enumerate() {
            if !self
                .size
                .is_area_inside(&definition.start, &definition.size)
            {
                bail!("{}.furniture is (partly) outside the furniture map", i + 1)
            }

            let furniture = definition
                .convert_to()
                .context(format!("Failed to convert the {}.furniture", i + 1))?;

            if hashmap.insert(definition.id, furniture).is_some() {
                bail!(
                    "{}.furniture has id '{}', which is already in use",
                    i + 1,
                    definition.id
                )
            }
        }

        FurnitureMap2d::new(self.size, hashmap).context("Failed to create the furniture map")
    }
}

pub fn load_furniture_map(path: &Path) -> Result<FurnitureMap2d> {
    info!("Load furniture map from {:?}", path);
    let definition: FurnitureMap2dDefinition = read(path).context(format!(
        "Failed to load the furniture map definition from {:?}",
        path
    ))?;
    definition.convert_to_map().context(format!(
        "Failed to convert the furniture map from {:?}",
        path
    ))
}

pub fn save_furniture_map(map: &FurnitureMap2d, path: &Path) {
    info!("Save furniture map to {:?}", path);
    let definition = FurnitureMap2dDefinition::convert_from_map(map);
    write(&definition, path).unwrap();
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
        furniture_map.add(Furniture::new(2, Point::new(2, 2), Size::new(3, 2), Right).unwrap());
        furniture_map.add(Furniture::new(3, Point::new(5, 2), Size::new(1, 2), Left).unwrap());
        furniture_map.add(Furniture::new(1, Point::new(2, 7), Size::new(6, 1), Top).unwrap());

        assert_eq!(
            FurnitureMap2dDefinition::convert_from_map(&furniture_map)
                .convert_to_map()
                .unwrap(),
            furniture_map
        );
    }
}
