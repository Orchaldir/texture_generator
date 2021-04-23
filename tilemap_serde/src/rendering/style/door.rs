use crate::rendering::style::edge::EdgeDefinition;
use serde::{Deserialize, Serialize};
use texture_generation::utils::error::DefinitionError;
use tilemap::rendering::style::door::DoorStyle;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DoorDefinition {
    name: String,
    size: u32,
    edge_style: EdgeDefinition,
    is_centered: bool,
}

impl DoorDefinition {
    pub fn convert(&self, size: u32) -> Result<DoorStyle, DefinitionError> {
        let factor = size as f32 / self.size as f32;
        let edge_style = self.edge_style.convert(factor)?;
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
            size: 200,
            edge_style: EdgeDefinition::Mock(10),
            is_centered: true,
        };
        let style = DoorStyle::new("door0", EdgeStyle::Mock(30), true);

        assert_eq!(style, definition.convert(600).unwrap())
    }
}
