use crate::definition::generation::component::rendering::color::ColorSelectorDefinition;
use crate::definition::generation::component::rendering::depth::DepthDefinition;
use crate::definition::math::shape_factor::ShapeFactorDefinition;
use crate::generation::component::rendering::depth_factory::DepthFactory;
use crate::generation::component::rendering::RenderingComponent;
use crate::math::color::Color;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

pub mod color;
pub mod depth;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RenderingDefinition {
    FillArea {
        color: String,
        depth: u8,
    },
    Mock,
    Shape {
        shape_factory: ShapeFactorDefinition,
        color: ColorSelectorDefinition,
        depth: DepthDefinition,
    },
}

impl RenderingDefinition {
    pub fn convert(&self, parent: &str, _factor: f32) -> Result<RenderingComponent> {
        match self {
            RenderingDefinition::FillArea { color, depth } => {
                let color = Color::convert(&color).context(format!(
                    "Failed to convert 'color' of '{}.FillArea'",
                    parent
                ))?;
                Ok(RenderingComponent::new_fill_area(color, *depth))
            }
            RenderingDefinition::Mock => Ok(RenderingComponent::Mock),
            RenderingDefinition::Shape {
                shape_factory,
                color,
                depth,
            } => {
                let shape_factory = shape_factory
                    .convert()
                    .context(format!("Failed to convert 'shape' of '{}.Shape'", parent))?;
                let color = color
                    .convert()
                    .context(format!("Failed to convert 'color' of '{}.Shape'", parent))?;
                let depth: DepthFactory = depth
                    .clone()
                    .convert()
                    .context(format!("Failed to convert 'depth' of '{}.Shape'", parent))?;

                Ok(RenderingComponent::new_shape_with_depth(
                    shape_factory,
                    color,
                    depth,
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generation::component::rendering::color::ColorSelector;
    use crate::generation::component::rendering::depth_factory::DepthFactory;
    use crate::math::color::ORANGE;
    use crate::math::shape_factory::ShapeFactory;

    #[test]
    fn test_convert_fill_area() {
        let definition = RenderingDefinition::FillArea {
            color: "#FFA500".to_string(),
            depth: 111,
        };
        let component = RenderingComponent::new_fill_area(ORANGE, 111);

        assert_eq!(component, definition.convert("test", 3.0).unwrap())
    }

    #[test]
    fn test_convert_shape() {
        let shape_factory = ShapeFactorDefinition::Circle;
        let color = ColorSelectorDefinition::ConstantColor("#FFA500".to_string());
        let depth = DepthDefinition::Uniform(111);
        let definition = RenderingDefinition::Shape {
            shape_factory,
            color,
            depth,
        };
        let component = RenderingComponent::new_shape_with_depth(
            ShapeFactory::Circle,
            ColorSelector::ConstantColor(ORANGE),
            DepthFactory::Uniform(111),
        );

        assert_eq!(component, definition.convert("test", 3.0).unwrap())
    }

    #[test]
    fn test_convert_invalid_shape() {
        let shape_factory = ShapeFactorDefinition::RoundedRectangle(-1.0);
        let color = ColorSelectorDefinition::ConstantColor("#FFA500".to_string());
        let depth = DepthDefinition::Uniform(111);
        let definition = RenderingDefinition::Shape {
            shape_factory,
            color,
            depth,
        };

        assert!(definition.convert("test", 2.0).is_err());
    }
}
