use crate::rendering::style::front::FrontDefinition;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use texture_generation::definition::generation::component::ComponentDefinition;
use texture_generation::utils::resource::ResourceDefinition;
use tilemap::rendering::style::furniture::FurnitureStyle;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FurnitureDefinition {
    tile_size: u32,
    component: ComponentDefinition,
    front: FrontDefinition,
}

impl ResourceDefinition for FurnitureDefinition {
    type R = FurnitureStyle;

    fn convert(&self, name: &str, size: u32) -> Result<FurnitureStyle> {
        let factor = size as f32 / self.tile_size as f32;
        let component = self
            .component
            .convert("component", factor)
            .context(format!(
                "Failed to convert 'component' of the furniture '{}'",
                name
            ))?;
        let front = self.front.convert("front", factor).context(format!(
            "Failed to convert 'front' of the furniture '{}'",
            name
        ))?;
        Ok(FurnitureStyle::new(name, component, front))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use texture_generation::generation::component::Component;
    use tilemap::rendering::style::front::FrontStyle;

    #[test]
    fn test_convert_without_handle() {
        let definition = FurnitureDefinition {
            tile_size: 200,
            component: ComponentDefinition::Mock(56),
            front: FrontDefinition::One(99),
        };
        let style = FurnitureStyle::new("furniture0", Component::Mock(56), FrontStyle::One(99));

        assert_eq!(style, definition.convert("furniture0", 600).unwrap())
    }
}
