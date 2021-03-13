use crate::math::shape::Shape;
use crate::utils::error::ShapeError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ShapeDefinition {
    Circle(u32),
    Rectangle { width: u32, height: u32 },
}

impl TryFrom<ShapeDefinition> for Shape {
    type Error = ShapeError;

    fn try_from(definition: ShapeDefinition) -> Result<Self, Self::Error> {
        match definition {
            ShapeDefinition::Circle(radius) => Shape::new_circle(radius),
            ShapeDefinition::Rectangle { width, height } => Shape::new_rectangle(width, height),
        }
    }
}

impl From<&Shape> for ShapeDefinition {
    fn from(shape: &Shape) -> Self {
        match shape {
            Shape::Circle(radius) => ShapeDefinition::Circle(*radius),
            Shape::Rectangle { half_x, half_y } => ShapeDefinition::Rectangle {
                width: *half_x as u32 * 2,
                height: *half_y as u32 * 2,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn test_convert_circle() {
        assert_convert(ShapeDefinition::Circle(11));
    }

    #[test]
    fn test_convert_rectangle() {
        assert_convert(ShapeDefinition::Rectangle {
            width: 10,
            height: 20,
        });
    }

    fn assert_convert(definition: ShapeDefinition) {
        let shape: Shape = definition.clone().try_into().unwrap();
        let result: ShapeDefinition = (&shape).into();

        assert_eq!(result, definition)
    }
}
