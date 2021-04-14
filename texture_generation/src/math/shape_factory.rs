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
    /// Creates a shape that fits inside the [`AABB`].
    pub fn create_shape(&self, aabb: &AABB) -> Result<Shape, ShapeError> {
        let size = aabb.size();
        let min_side = size.width().min(size.height());
        let radius = min_side / 2;

        match self {
            ShapeFactory::Circle => Shape::new_circle(radius),
            ShapeFactory::Rectangle => Shape::new_rectangle(size.width(), size.height()),
            ShapeFactory::RoundedRectangle(factor) => {
                let radius = (radius as f32 * *factor) as u32;
                Shape::new_rounded(size.width(), size.height(), radius)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::size::Size;

    const SIZE: Size = Size::new(12, 20);

    #[test]
    fn test_create_circle() {
        let aabb = AABB::with_size(SIZE);
        let factory = ShapeFactory::Circle;

        assert_eq!(factory.create_shape(&aabb), Ok(Shape::Circle(6)));
    }

    #[test]
    fn test_create_rectangle() {
        let aabb = AABB::with_size(SIZE);
        let factory = ShapeFactory::Rectangle;

        assert_eq!(factory.create_shape(&aabb), Shape::new_rectangle(12, 20));
    }

    #[test]
    fn test_create_rounded_rectangle() {
        let aabb = AABB::with_size(SIZE);
        let factory = ShapeFactory::RoundedRectangle(0.5);

        assert_eq!(factory.create_shape(&aabb), Shape::new_rounded(12, 20, 3));
    }
}
