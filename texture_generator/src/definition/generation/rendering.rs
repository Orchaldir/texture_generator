use crate::definition::math::shape::ShapeDefinition;
use crate::generation::rendering::{RenderingComponent, RenderingError};
use crate::math::color::Color;
use crate::math::shape::Shape;
use std::convert::{TryFrom, TryInto};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RenderingDefinition {
    Shape {
        name: String,
        shape: ShapeDefinition,
        color: Color,
    },
}

impl TryFrom<RenderingDefinition> for RenderingComponent {
    type Error = RenderingError;

    fn try_from(definition: RenderingDefinition) -> Result<Self, Self::Error> {
        match definition {
            RenderingDefinition::Shape { name, shape, color } => {
                let shape: Shape = shape.try_into()?;
                Ok(RenderingComponent::Shape { name, shape, color })
            }
        }
    }
}

impl From<&RenderingComponent> for RenderingDefinition {
    fn from(component: &RenderingComponent) -> Self {
        match component {
            RenderingComponent::Shape { name, shape, color } => RenderingDefinition::Shape {
                name: name.clone(),
                shape: shape.into(),
                color: *color,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generation::rendering::RenderingError;
    use crate::math::color::RED;
    use crate::math::shape::ShapeError;
    use std::convert::TryInto;

    #[test]
    fn test_convert_shape() {
        let shape = ShapeDefinition::Circle(42);
        assert_convert(RenderingDefinition::Shape {
            name: "circle".to_string(),
            shape,
            color: RED,
        });
    }

    #[test]
    fn test_convert_invalid_shape() {
        let shape = ShapeDefinition::Circle(0);
        let definition = RenderingDefinition::Shape {
            name: "brick".to_string(),
            shape,
            color: RED,
        };
        let shape_error = ShapeError::RadiusTooSmall(0);
        is_error(definition, RenderingError::ShapeError(shape_error))
    }

    fn assert_convert(definition: RenderingDefinition) {
        let shape: RenderingComponent = definition.clone().try_into().unwrap();
        let result: RenderingDefinition = (&shape).into();

        assert_eq!(result, definition)
    }

    fn is_error(
        data: impl TryInto<RenderingComponent, Error = RenderingError>,
        error: RenderingError,
    ) {
        assert_eq!(data.try_into(), Err(error));
    }
}
