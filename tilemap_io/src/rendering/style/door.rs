use crate::rendering::style::edge::EdgeDefinition;
use crate::rendering::style::handle::HandleDefinition;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use texture_generation::utils::resource::ResourceDefinition;
use tilemap::rendering::style::door::DoorStyle;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DoorDefinition {
    tile_size: u32,
    edge_style: EdgeDefinition,
    handle_style: Option<HandleDefinition>,
    is_centered: bool,
}

impl ResourceDefinition for DoorDefinition {
    type R = DoorStyle;

    fn convert(&self, name: &str, size: u32) -> Result<DoorStyle> {
        let factor = size as f32 / self.tile_size as f32;
        let edge_style = self
            .edge_style
            .convert("edge_style", factor)
            .context(format!(
                "Failed to convert 'edge_style' of the door '{}'",
                name
            ))?;
        let handle_style = self
            .handle_style
            .clone()
            .map(|h| h.convert("handle_style", factor))
            .map_or(Ok(None), |v| v.map(Some))
            .context(format!(
                "Failed to convert 'handle_style' of the door '{}'",
                name
            ))?;
        Ok(DoorStyle::new(
            name,
            edge_style,
            handle_style,
            self.is_centered,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tilemap::rendering::style::edge::EdgeStyle;

    #[test]
    fn test_convert_without_handle() {
        let definition = DoorDefinition {
            tile_size: 200,
            edge_style: EdgeDefinition::Mock(10),
            handle_style: None,
            is_centered: true,
        };
        let style = DoorStyle::new("door0", EdgeStyle::Mock(30), None, true);

        assert_eq!(style, definition.convert("door0", 600).unwrap())
    }
}
