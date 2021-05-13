use crate::rendering::style::edge::EdgeDefinition;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use texture_generation::utils::resource::ResourceDefinition;
use tilemap::rendering::style::window::WindowStyle;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowDefinition {
    name: String,
    tile_size: u32,
    pane_style: EdgeDefinition,
    stool_style: EdgeDefinition,
}

impl ResourceDefinition for WindowDefinition {
    type R = WindowStyle;

    fn convert(&self, size: u32) -> Result<WindowStyle> {
        let factor = size as f32 / self.tile_size as f32;
        let pane_style = self
            .pane_style
            .convert("pane_style", factor)
            .context(format!(
                "Failed to convert 'pane_style' of the window '{}'",
                self.name
            ))?;
        let stool_style = self
            .stool_style
            .convert("stool_style", factor)
            .context(format!(
                "Failed to convert 'stool_style' of the window '{}'",
                self.name
            ))?;
        Ok(WindowStyle::new(self.name.clone(), pane_style, stool_style))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tilemap::rendering::style::edge::EdgeStyle;

    #[test]
    fn test_convert() {
        let definition = WindowDefinition {
            name: "window0".to_string(),
            tile_size: 200,
            pane_style: EdgeDefinition::Mock(10),
            stool_style: EdgeDefinition::Mock(30),
        };
        let style = WindowStyle::new("window0", EdgeStyle::Mock(30), EdgeStyle::Mock(90));

        assert_eq!(style, definition.convert(600).unwrap())
    }
}
