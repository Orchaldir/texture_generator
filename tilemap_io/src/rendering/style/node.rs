use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use texture_generation::definition::convert;
use texture_generation::definition::generation::component::rendering::RenderingDefinition;
use texture_generation::utils::resource::ResourceDefinition;
use tilemap::rendering::style::node::NodeStyle;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeDefinition {
    name: String,
    tile_size: u32,
    size: u32,
    component: RenderingDefinition,
}

impl ResourceDefinition for NodeDefinition {
    type R = NodeStyle;

    fn convert(&self, size: u32) -> Result<NodeStyle> {
        let factor = size as f32 / self.tile_size as f32;
        let size = convert(self.size, factor);
        let component = self
            .component
            .convert("component", factor)
            .context(format!(
                "Failed to convert 'component' of the node '{}'",
                self.name
            ))?;
        Ok(NodeStyle::new(size, component))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use texture_generation::generation::component::rendering::RenderingComponent;
    use texture_generation::math::color::RED;

    #[test]
    fn test_convert() {
        let rendering_definition = RenderingDefinition::FillArea {
            color: "#FF0000".to_string(),
            depth: 123,
        };
        let definition = NodeDefinition {
            name: "window0".to_string(),
            tile_size: 200,
            size: 35,
            component: rendering_definition,
        };
        let component = RenderingComponent::new_fill_area(RED, 123);
        let style = NodeStyle::new(105, component);

        assert_eq!(style, definition.convert(600).unwrap())
    }
}
