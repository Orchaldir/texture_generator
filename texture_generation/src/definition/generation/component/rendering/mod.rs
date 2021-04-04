use crate::definition::generation::component::rendering::color::ColorSelectorDefinition;
use crate::definition::generation::component::rendering::depth::DepthDefinition;
use crate::definition::math::shape::ShapeDefinition;
use crate::generation::component::rendering::RenderingComponent;
use crate::math::color::Color;
use crate::utils::error::DefinitionError;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

pub mod color;
pub mod depth;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum RenderingDefinition {
    FillArea {
        name: String,
        color: String,
        depth: u8,
    },
    Shape {
        name: String,
        shape: ShapeDefinition,
        color: ColorSelectorDefinition,
        depth: DepthDefinition,
    },
}

impl RenderingDefinition {
    pub fn convert(&self, factor: f32) -> Result<RenderingComponent, DefinitionError> {
        match self {
            RenderingDefinition::FillArea { name, color, depth } => {
                let color = Color::convert(&color)
                    .ok_or_else(|| DefinitionError::invalid_color(name, &color))?;
                Ok(RenderingComponent::new_fill_area(name, color, *depth))
            }
            RenderingDefinition::Shape {
                name,
                shape,
                color,
                depth,
            } => match shape.convert(factor) {
                Ok(shape) => match depth.clone().try_into() {
                    Ok(depth) => {
                        let color = color.convert(name)?;
                        Ok(RenderingComponent::new_shape_with_depth(
                            name, shape, color, depth,
                        ))
                    }
                    Err(error) => Err(error),
                },
                Err(error) => Err(DefinitionError::invalid_shape(name, error)),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generation::component::rendering::color::ColorSelector;
    use crate::generation::component::rendering::depth::DepthCalculator;
    use crate::math::color::ORANGE;
    use crate::math::shape::Shape;

    #[test]
    fn test_convert_fill_area() {
        let definition = RenderingDefinition::FillArea {
            name: "fill".to_string(),
            color: "#FFA500".to_string(),
            depth: 111,
        };
        let component = RenderingComponent::new_fill_area("fill", ORANGE, 111);

        assert_eq!(component, definition.convert(3.0).unwrap())
    }

    #[test]
    fn test_convert_shape() {
        let shape = ShapeDefinition::Circle(42);
        let color = ColorSelectorDefinition::ConstantColor("#FFA500".to_string());
        let depth = DepthDefinition::Uniform(111);
        let definition = RenderingDefinition::Shape {
            name: "circle".to_string(),
            shape,
            color,
            depth,
        };
        let component = RenderingComponent::new_shape_with_depth(
            "circle",
            Shape::Circle(126),
            ColorSelector::ConstantColor(ORANGE),
            DepthCalculator::Uniform(111),
        );

        assert_eq!(component, definition.convert(3.0).unwrap())
    }

    #[test]
    fn test_convert_invalid_shape() {
        let shape = ShapeDefinition::Circle(0);
        let color = ColorSelectorDefinition::ConstantColor("#FFA500".to_string());
        let depth = DepthDefinition::Uniform(111);
        let definition = RenderingDefinition::Shape {
            name: "brick".to_string(),
            shape,
            color,
            depth,
        };

        assert!(definition.convert(2.0).is_err());
    }
}
