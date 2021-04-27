use crate::math::aabb::AABB;
use crate::math::shape::Shape;
use crate::utils::error::ShapeError;

#[derive(Copy, Clone, Debug, PartialEq)]
/// Different shapes that are centered around (0,0).
pub enum ShapeFactory {
    Circle,
    Rectangle,
    /// A rectangle with rounded corners.
    RoundedRectangle(f32),
}

impl ShapeFactory {
    pub fn new_rounded(factor: f32) -> Result<ShapeFactory, ShapeError> {
        if factor <= 0.0 {
            return Err(ShapeError::FactorTooSmall(factor));
        } else if factor >= 1.0 {
            return Err(ShapeError::FactorTooBig(factor));
        }

        Ok(ShapeFactory::RoundedRectangle(factor))
    }

    /// Creates a shape that fits inside the [`AABB`].
    pub fn create_shape(&self, aabb: &AABB) -> Result<Shape, ShapeError> {
        let size = aabb.size();
        let min_side = size.width().min(size.height());
        let radius = min_side / 2;
        let center = aabb.center();

        match self {
            ShapeFactory::Circle => Shape::new_circle(center, radius),
            ShapeFactory::Rectangle => Shape::new_rectangle(center, size.width(), size.height()),
            ShapeFactory::RoundedRectangle(factor) => {
                if min_side < 5 {
                    return Shape::new_rectangle(center, size.width(), size.height());
                }

                let radius = (radius as f32 * *factor) as u32;
                Shape::new_rounded(center, size.width(), size.height(), radius)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::point::Point;
    use crate::math::size::Size;

    const CENTER: Point = Point::new(6, 10);
    const SIZE: Size = Size::new(12, 20);

    #[test]
    fn test_create_circle() {
        let aabb = AABB::with_size(SIZE);
        let factory = ShapeFactory::Circle;

        assert_eq!(factory.create_shape(&aabb), Shape::new_circle(CENTER, 6));
    }

    #[test]
    fn test_create_rectangle() {
        let aabb = AABB::with_size(SIZE);
        let factory = ShapeFactory::Rectangle;

        assert_eq!(
            factory.create_shape(&aabb),
            Shape::new_rectangle(CENTER, 12, 20)
        );
    }

    #[test]
    fn test_create_rounded_rectangle() {
        let aabb = AABB::with_size(SIZE);
        let factory = ShapeFactory::RoundedRectangle(0.5);

        assert_eq!(
            factory.create_shape(&aabb),
            Shape::new_rounded(CENTER, 12, 20, 3)
        );
    }
}
