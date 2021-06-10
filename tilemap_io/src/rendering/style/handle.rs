use anyhow::Result;
use serde::{Deserialize, Serialize};
use texture_generation::definition::generation::component::rendering::RenderingDefinition;
use texture_generation::definition::{convert, convert_size};
use texture_generation::math::size::Size;
use tilemap::rendering::style::handle::{HandlePosition, HandleStyle};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum HandlePositionDefinition {
    Centered,
    DistanceToEnd(u32),
}

impl HandlePositionDefinition {
    pub fn convert(&self, factor: f32) -> HandlePosition {
        match self {
            HandlePositionDefinition::Centered => HandlePosition::Centered,
            HandlePositionDefinition::DistanceToEnd(distance) => {
                HandlePosition::DistanceToEnd(convert(*distance, factor) as i32)
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HandleDefinition {
    position: HandlePositionDefinition,
    offset: u32,
    both_sides: bool,
    size: Size,
    component: RenderingDefinition,
}

impl HandleDefinition {
    pub fn convert(&self, parent: &str, factor: f32) -> Result<HandleStyle> {
        let component = self
            .component
            .convert(&format!("{}.component", parent), factor)?;

        HandleStyle::new(
            self.position.convert(factor),
            convert(self.offset, factor),
            self.both_sides,
            convert_size(&self.size, factor),
            component,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rendering::style::handle::HandlePositionDefinition::DistanceToEnd;
    use texture_generation::generation::component::rendering::RenderingComponent;

    #[test]
    fn test_convert_layout() {
        let definition = HandleDefinition {
            position: DistanceToEnd(1),
            offset: 2,
            both_sides: true,
            size: Size::new(3, 4),
            component: RenderingDefinition::Mock,
        };
        let position = HandlePosition::DistanceToEnd(10);
        let style = HandleStyle::new(
            position,
            20,
            true,
            Size::new(30, 40),
            RenderingComponent::Mock,
        )
        .unwrap();

        assert_eq!(style, definition.convert("test", 10.0).unwrap())
    }
}
