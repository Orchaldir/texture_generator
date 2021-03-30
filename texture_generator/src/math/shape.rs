use crate::math::point::Point;
use crate::utils::error::ShapeError;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// Different shapes that are centered around (0,0).
pub enum Shape {
    Circle(u32),
    Rectangle {
        half_x: i32,
        half_y: i32,
    },
    RoundedRectangle {
        half_x: i32,
        half_y: i32,
        radius: i32,
    },
}

impl Shape {
    pub fn new_circle(radius: u32) -> Result<Shape, ShapeError> {
        if radius < 1 {
            return Err(ShapeError::RadiusTooSmall(radius));
        }

        Ok(Shape::Circle(radius))
    }

    pub fn new_rectangle(width: u32, height: u32) -> Result<Shape, ShapeError> {
        if width < 1 {
            return Err(ShapeError::WidthTooSmall(width));
        } else if height < 1 {
            return Err(ShapeError::HeightTooSmall(height));
        }

        Ok(Shape::Rectangle {
            half_x: (width / 2) as i32,
            half_y: (height / 2) as i32,
        })
    }

    pub fn new_rounded(width: u32, height: u32, radius: u32) -> Result<Shape, ShapeError> {
        if width < 1 {
            return Err(ShapeError::WidthTooSmall(width));
        } else if height < 1 {
            return Err(ShapeError::HeightTooSmall(height));
        } else if radius == 0 {
            return Err(ShapeError::RadiusTooSmall(radius));
        } else if radius * 2 > width || radius * 2 > height {
            return Err(ShapeError::RadiusTooBig(radius));
        }

        let radius = radius as i32;

        Ok(Shape::RoundedRectangle {
            half_x: (width / 2) as i32 - radius,
            half_y: (height / 2) as i32 - radius,
            radius,
        })
    }

    /// Calculates the euclidean distance from the shape's center to a [`Point`].
    /// Values larger than 1 are outside.
    ///
    /// ```
    ///# use texture_generator::math::point::Point;
    ///# use texture_generator::math::shape::Shape;
    /// let center = Point::new(10, 20);
    /// let border = Point::new(7, 20);
    /// let outside = Point::new(10, 26);
    /// let circle = Shape::new_circle(3).unwrap();
    ///
    /// assert_eq!(circle.distance(&center, &center), 0.0);
    /// assert_eq!(circle.distance(&center, &outside), 2.0);
    /// assert_eq!(circle.distance(&center, &border), 1.0);
    /// ```
    pub fn distance(&self, center: &Point, point: &Point) -> f32 {
        match self {
            Shape::Circle(radius) => center.calculate_distance(point) / *radius as f32,
            Shape::Rectangle { half_x, half_y } => {
                let max_half = *half_x.min(half_y) as f32;
                let diff = *point - *center;
                let distance = (diff.x.abs() - *half_x).max(diff.y.abs() - *half_y) as f32;
                (distance + max_half) / max_half
            }
            Shape::RoundedRectangle {
                half_x,
                half_y,
                radius,
            } => {
                let diff = *point - *center;
                let squared_x = (diff.x.abs() - *half_x).max(0).pow(2);
                let squared_y = (diff.y.abs() - *half_y).max(0).pow(2);
                ((squared_x + squared_y) as f32).sqrt() - *radius as f32
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::size::Size;

    const CENTER: Point = Point::new(2, 3);

    #[test]
    fn test_distance_rectangle() {
        let size = Size::new(5, 7);
        let rectangle = Shape::new_rectangle(2, 4).unwrap();

        #[rustfmt::skip]
        let results = vec![
            2.0, 2.0,  2.0, 2.0, 2.0,
            2.0, 1.0,  1.0, 1.0, 2.0,
            2.0, 1.0,  0.0, 1.0, 2.0,
            2.0, 1.0,  0.0, 1.0, 2.0,
            2.0, 1.0,  0.0, 1.0, 2.0,
            2.0, 1.0,  1.0, 1.0, 2.0,
            2.0, 2.0,  2.0, 2.0, 2.0
        ];

        for (index, result) in results.iter().enumerate() {
            assert_eq!(rectangle.distance(&CENTER, &size.to_point(index)), *result);
        }
    }

    #[test]
    fn test_distance_rounded() {
        let size = Size::new(5, 7);
        let rectangle = Shape::new_rounded(2, 4, 1).unwrap();

        let c2 = 2.6457512;

        #[rustfmt::skip]
        let results = vec![
             c2, 2.0,  2.0, 2.0,  c2,
            2.0, 1.0,  1.0, 1.0, 2.0,
            2.0, 1.0,  0.0, 1.0, 2.0,
            2.0, 1.0,  0.0, 1.0, 2.0,
            2.0, 1.0,  0.0, 1.0, 2.0,
            2.0, 1.0,  1.0, 1.0, 2.0,
             c2, 2.0,  2.0, 2.0,  c2
        ];

        for (index, result) in results.iter().enumerate() {
            relative_eq!(rectangle.distance(&CENTER, &size.to_point(index)), *result);
        }
    }
}
