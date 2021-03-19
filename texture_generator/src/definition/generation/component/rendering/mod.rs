use crate::definition::generation::component::rendering::depth::DepthDefinition;
use crate::definition::math::shape::ShapeDefinition;
use crate::generation::component::rendering::RenderingComponent;
use crate::math::color::Color;
use crate::utils::error::GenerationError;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

pub mod depth;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum RenderingDefinition {
    Shape {
        name: String,
        shape: ShapeDefinition,
        color: Color,
        depth: DepthDefinition,
    },
}

impl RenderingDefinition {
    pub fn convert(&self, factor: f32) -> Result<RenderingComponent, GenerationError> {
        match self {
            RenderingDefinition::Shape {
                name,
                shape,
                color,
                depth,
            } => match shape.convert(factor) {
                Ok(shape) => match depth.clone().try_into() {
                    Ok(depth) => Ok(RenderingComponent::new_shape_with_depth(
                        name, shape, *color, depth,
                    )),
                    Err(error) => Err(error),
                },
                Err(error) => Err(GenerationError::invalid_shape(name, error)),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generation::component::rendering::depth::DepthCalculator;
    use crate::math::color::RED;
    use crate::math::shape::Shape;

    #[test]
    fn test_convert_shape() {
        let shape = ShapeDefinition::Circle(42);
        let depth = DepthDefinition::Uniform(111);
        let definition = RenderingDefinition::Shape {
            name: "circle".to_string(),
            shape,
            color: RED,
            depth,
        };
        let component = RenderingComponent::new_shape_with_depth(
            "circle",
            Shape::Circle(126),
            RED,
            DepthCalculator::Uniform(111),
        );

        assert_eq!(component, definition.convert(3.0).unwrap())
    }

    #[test]
    fn test_convert_invalid_shape() {
        let shape = ShapeDefinition::Circle(0);
        let depth = DepthDefinition::Uniform(111);
        let definition = RenderingDefinition::Shape {
            name: "brick".to_string(),
            shape,
            color: RED,
            depth,
        };

        assert!(definition.convert(2.0).is_err());
    }
}
