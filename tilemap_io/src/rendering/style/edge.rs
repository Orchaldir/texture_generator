use serde::{Deserialize, Serialize};
use texture_generation::definition::convert;
use texture_generation::definition::generation::component::layout::LayoutDefinition;
use texture_generation::definition::generation::component::rendering::RenderingDefinition;
use texture_generation::utils::error::DefinitionError;
use tilemap::rendering::style::edge::EdgeStyle;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EdgeDefinition {
    Layout {
        thickness: u32,
        layout: LayoutDefinition,
    },
    Mock(u32),
    Solid {
        thickness: u32,
        component: RenderingDefinition,
    },
}

impl EdgeDefinition {
    pub fn convert(&self, factor: f32) -> Result<EdgeStyle, DefinitionError> {
        match self {
            EdgeDefinition::Layout { thickness, layout } => Ok(EdgeStyle::new_layout(
                convert(*thickness, factor),
                layout.convert(factor)?,
            )),
            EdgeDefinition::Mock(value) => Ok(EdgeStyle::Mock(convert(*value, factor))),
            EdgeDefinition::Solid {
                thickness,
                component,
            } => Ok(EdgeStyle::new_solid(
                convert(*thickness, factor),
                component.convert(factor)?,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use texture_generation::generation::component::layout::LayoutComponent;

    #[test]
    fn test_convert_layout() {
        let definition = EdgeDefinition::Layout {
            thickness: 10,
            layout: LayoutDefinition::Mock(42),
        };
        let style = EdgeStyle::new_layout(30, LayoutComponent::Mock(42));

        assert_eq!(style, definition.convert(3.0).unwrap())
    }
}