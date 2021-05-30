use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use texture_generation::definition::generation::component::ComponentDefinition;
use texture_generation::utils::resource::ResourceDefinition;
use tilemap::rendering::style::furniture::FurnitureStyle;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FurnitureDefinition {
    tile_size: u32,
    component: ComponentDefinition,
}

impl ResourceDefinition for FurnitureDefinition {
    type R = FurnitureStyle;

    fn convert(&self, name: &str, size: u32) -> Result<FurnitureStyle> {
        let factor = size as f32 / self.tile_size as f32;
        let component = self
            .component
            .convert("edge_style", factor)
            .context(format!(
                "Failed to convert 'component' of the furniture '{}'",
                name
            ))?;
        Ok(FurnitureStyle::new(name, component))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use texture_generation::generation::component::Component;

    #[test]
    fn test_convert_without_handle() {
        let definition = FurnitureDefinition {
            tile_size: 200,
            component: ComponentDefinition::Mock(56),
        };
        let style = FurnitureStyle::new("furniture0", Component::Mock(56));

        assert_eq!(style, definition.convert("furniture0", 600).unwrap())
    }
}
