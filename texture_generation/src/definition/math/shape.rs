use crate::definition::convert;
use crate::math::shape::Shape;
use crate::utils::error::ShapeError;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ShapeDefinition {
    Circle(u32),
    Rectangle {
        width: u32,
        height: u32,
    },
    RoundedRectangle {
        width: u32,
        height: u32,
        radius: u32,
    },
}

impl ShapeDefinition {
    pub fn convert(&self, factor: f32) -> Result<Shape, ShapeError> {
        match self {
            ShapeDefinition::Circle(radius) => Shape::new_circle(convert(*radius, factor)),
            ShapeDefinition::Rectangle { width, height } => {
                Shape::new_rectangle(convert(*width, factor), convert(*height, factor))
            }
            ShapeDefinition::RoundedRectangle {
                width,
                height,
                radius,
            } => Shape::new_rounded(
                convert(*width, factor),
                convert(*height, factor),
                convert(*radius, factor),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_circle() {
        let definition = ShapeDefinition::Circle(20);
        let shape = Shape::Circle(10);

        assert_eq!(shape, definition.convert(0.5).unwrap())
    }

    #[test]
    fn test_convert_rectangle() {
        let definition = ShapeDefinition::Rectangle {
            width: 10,
            height: 20,
        };
        let shape = Shape::new_rectangle(15, 30).unwrap();

        assert_eq!(shape, definition.convert(1.5).unwrap())
    }

    #[test]
    fn test_convert_rounded() {
        let definition = ShapeDefinition::RoundedRectangle {
            width: 10,
            height: 20,
            radius: 4,
        };
        let shape = Shape::new_rounded(15, 30, 6).unwrap();

        assert_eq!(shape, definition.convert(1.5).unwrap())
    }
}
