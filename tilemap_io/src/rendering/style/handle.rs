use anyhow::Result;
use serde::{Deserialize, Serialize};
use texture_generation::definition::generation::component::rendering::RenderingDefinition;
use texture_generation::definition::{convert, convert_size};
use texture_generation::math::size::Size;
use tilemap::rendering::style::handle::HandleStyle;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HandleDefinition {
    distance_to_end: u32,
    offset: u32,
    size: Size,
    component: RenderingDefinition,
}

impl HandleDefinition {
    pub fn convert(&self, parent: &str, factor: f32) -> Result<HandleStyle> {
        let component = self
            .component
            .convert(&format!("{}.component", parent), factor)?;

        HandleStyle::new(
            convert(self.distance_to_end, factor),
            convert(self.offset, factor),
            convert_size(&self.size, factor),
            component,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use texture_generation::generation::component::rendering::RenderingComponent;

    #[test]
    fn test_convert_layout() {
        let definition = HandleDefinition {
            distance_to_end: 1,
            offset: 2,
            size: Size::new(3, 4),
            component: RenderingDefinition::Mock,
        };
        let style = HandleStyle::new(10, 20, Size::new(30, 40), RenderingComponent::Mock).unwrap();

        assert_eq!(style, definition.convert("test", 10.0).unwrap())
    }
}
