use crate::rendering::style::edge::EdgeDefinition;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use texture_generation::utils::resource::ResourceDefinition;
use tilemap::rendering::style::door::DoorStyle;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DoorDefinition {
    name: String,
    tile_size: u32,
    edge_style: EdgeDefinition,
    is_centered: bool,
}

impl ResourceDefinition for DoorDefinition {
    type R = DoorStyle;

    fn convert(&self, size: u32) -> Result<DoorStyle> {
        let factor = size as f32 / self.tile_size as f32;
        let edge_style = self
            .edge_style
            .convert("edge_style", factor)
            .context(format!(
                "Failed to convert 'edge_style' of the door '{}'",
                self.name
            ))?;
        Ok(DoorStyle::new(
            self.name.clone(),
            edge_style,
            self.is_centered,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tilemap::rendering::style::edge::EdgeStyle;

    #[test]
    fn test_convert_layout() {
        let definition = DoorDefinition {
            name: "door0".to_string(),
            tile_size: 200,
            edge_style: EdgeDefinition::Mock(10),
            is_centered: true,
        };
        let style = DoorStyle::new("door0", EdgeStyle::Mock(30), true);

        assert_eq!(style, definition.convert(600).unwrap())
    }
}
