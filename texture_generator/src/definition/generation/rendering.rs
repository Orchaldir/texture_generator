use crate::definition::math::shape::ShapeDefinition;
use crate::generation::rendering::{RenderComponent, RenderError};
use crate::math::color::Color;
use crate::math::shape::Shape;
use std::convert::{TryFrom, TryInto};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RenderDefinition {
    Shape {
        shape: ShapeDefinition,
        color: Color,
    },
}

impl TryFrom<RenderDefinition> for RenderComponent {
    type Error = RenderError;

    fn try_from(definition: RenderDefinition) -> Result<Self, Self::Error> {
        match definition {
            RenderDefinition::Shape { shape, color } => {
                let shape: Shape = shape.try_into()?;
                Ok(RenderComponent::Shape { shape, color })
            }
        }
    }
}

impl From<&RenderComponent> for RenderDefinition {
    fn from(shape: &RenderComponent) -> Self {
        match shape {
            RenderComponent::Shape { shape, color } => RenderDefinition::Shape {
                shape: shape.into(),
                color: *color,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generation::rendering::RenderError;
    use crate::math::color::RED;
    use crate::math::shape::ShapeError;
    use std::convert::TryInto;

    #[test]
    fn test_convert_shape() {
        let shape = ShapeDefinition::Circle(42);
        assert_convert(RenderDefinition::Shape { shape, color: RED });
    }

    #[test]
    fn test_convert_invalid_shape() {
        let shape = ShapeDefinition::Circle(0);
        let definition = RenderDefinition::Shape { shape, color: RED };
        let shape_error = ShapeError::RadiusTooSmall(0);
        is_error(definition, RenderError::ShapeError(shape_error))
    }

    fn assert_convert(definition: RenderDefinition) {
        let shape: RenderComponent = definition.clone().try_into().unwrap();
        let result: RenderDefinition = (&shape).into();

        assert_eq!(result, definition)
    }

    fn is_error(data: impl TryInto<RenderComponent, Error = RenderError>, error: RenderError) {
        assert_eq!(data.try_into(), Err(error));
    }
}
