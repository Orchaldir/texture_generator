use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use texture_generation::definition::convert;
use texture_generation::definition::generation::component::layout::LayoutDefinition;
use texture_generation::definition::generation::component::rendering::RenderingDefinition;
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
    pub fn convert(&self, parent: &str, factor: f32) -> Result<EdgeStyle> {
        match self {
            EdgeDefinition::Layout { thickness, layout } => {
                let layout = layout.convert(&format!("{}.Layout.layout", parent), factor)?;
                let edge_style = EdgeStyle::new_layout(convert(*thickness, factor), layout)
                    .context(format!("Failed to create '{}.Layout'", parent))?;
                Ok(edge_style)
            }
            EdgeDefinition::Mock(value) => Ok(EdgeStyle::Mock(convert(*value, factor))),
            EdgeDefinition::Solid {
                thickness,
                component,
            } => {
                let component =
                    component.convert(&format!("{}.Solid.component", parent), factor)?;
                let edge_style = EdgeStyle::new_solid(convert(*thickness, factor), component)
                    .context(format!("Failed to create '{}.Solid'", parent))?;
                Ok(edge_style)
            }
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
        let style = EdgeStyle::new_layout(30, LayoutComponent::Mock(42)).unwrap();

        assert_eq!(style, definition.convert("test", 3.0).unwrap())
    }
}
