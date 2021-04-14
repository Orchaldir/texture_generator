use crate::math::shape_factory::ShapeFactory;
use crate::utils::error::ShapeError;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ShapeFactorDefinition {
    Circle,
    Rectangle,
    RoundedRectangle(f32),
}

impl ShapeFactorDefinition {
    pub fn convert(&self) -> Result<ShapeFactory, ShapeError> {
        match self {
            ShapeFactorDefinition::Circle => Ok(ShapeFactory::Circle),
            ShapeFactorDefinition::Rectangle => Ok(ShapeFactory::Rectangle),
            ShapeFactorDefinition::RoundedRectangle(factor) => ShapeFactory::new_rounded(*factor),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_circle() {
        let definition = ShapeFactorDefinition::Circle;
        let factory = ShapeFactory::Circle;

        assert_eq!(factory, definition.convert().unwrap())
    }

    #[test]
    fn test_convert_rectangle() {
        let definition = ShapeFactorDefinition::Rectangle;
        let factory = ShapeFactory::Rectangle;

        assert_eq!(factory, definition.convert().unwrap())
    }

    #[test]
    fn test_convert_rounded() {
        let definition = ShapeFactorDefinition::RoundedRectangle(0.5);
        let factory = ShapeFactory::RoundedRectangle(0.5);

        assert_eq!(factory, definition.convert().unwrap())
    }

    #[test]
    fn test_factor_too_small() {
        let definition = ShapeFactorDefinition::RoundedRectangle(-0.1);

        assert!(definition.convert().is_err())
    }

    #[test]
    fn test_factor_too_large() {
        let definition = ShapeFactorDefinition::RoundedRectangle(1.1);

        assert!(definition.convert().is_err())
    }
}
