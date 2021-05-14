use crate::rendering::style::edge::EdgeDefinition;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use texture_generation::utils::resource::ResourceDefinition;
use tilemap::rendering::style::wall::WallStyle;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WallDefinition {
    name: String,
    tile_size: u32,
    edge_style: EdgeDefinition,
    node_style: Option<usize>,
    corner_style: usize,
}

impl ResourceDefinition for WallDefinition {
    type R = WallStyle;

    fn convert(&self, size: u32) -> Result<WallStyle> {
        let factor = size as f32 / self.tile_size as f32;
        let edge_style = self
            .edge_style
            .convert("edge_style", factor)
            .context(format!(
                "Failed to convert 'edge_style' of the wall '{}'",
                self.name
            ))?;
        Ok(WallStyle::new(
            self.name.clone(),
            edge_style,
            self.node_style,
            self.corner_style,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tilemap::rendering::style::edge::EdgeStyle;

    #[test]
    fn test_convert() {
        let definition = WallDefinition {
            name: "wall0".to_string(),
            tile_size: 200,
            edge_style: EdgeDefinition::Mock(10),
            node_style: Some(4),
            corner_style: 3,
        };
        let style = WallStyle::new("wall0", EdgeStyle::Mock(30), Some(4), 3);

        assert_eq!(style, definition.convert(600).unwrap())
    }
}
