use crate::rendering::style::edge::EdgeDefinition;
use serde::{Deserialize, Serialize};
use texture_generation::utils::error::DefinitionError;
use texture_generation::utils::resource::ResourceDefinition;
use tilemap::rendering::style::window::WindowStyle;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowDefinition {
    name: String,
    size: u32,
    pane_style: EdgeDefinition,
    stool_style: EdgeDefinition,
}

impl ResourceDefinition for WindowDefinition {
    type R = WindowStyle;

    fn convert(&self, size: u32) -> Result<WindowStyle, DefinitionError> {
        let factor = size as f32 / self.size as f32;
        let pane_style = self.pane_style.convert(factor)?;
        let stool_style = self.stool_style.convert(factor)?;
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
            size: 200,
            pane_style: EdgeDefinition::Mock(10),
            stool_style: EdgeDefinition::Mock(30),
        };
        let style = WindowStyle::new("window0", EdgeStyle::Mock(30), EdgeStyle::Mock(90));

        assert_eq!(style, definition.convert(600).unwrap())
    }
}
