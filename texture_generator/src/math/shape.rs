use crate::math::point::Point;
use crate::utils::error::ShapeError;

#[derive(Copy, Clone, Debug, PartialEq)]
/// Different shapes that are centered around (0,0).
pub enum Shape {
    Circle(u32),
    Rectangle {
        half_x: i32,
        half_y: i32,
    },
    /// A rectangle with rounded corners.
    RoundedRectangle {
        half_x: i32,
        half_y: i32,
        radius: f32,
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

        let radius = radius as f32;

        Ok(Shape::RoundedRectangle {
            half_x: (width / 2) as i32 - radius as i32,
            half_y: (height / 2) as i32 - radius as i32,
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
                let x = (diff.x.abs() - *half_x).max(0) as f32;
                let y = (diff.y.abs() - *half_y).max(0) as f32;
                x.hypot(y) / *radius
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
        let radius = 10;
        let rectangle = Shape::new_rounded(20, 50, radius).unwrap();

        for x in 0..(radius * 2) {
            let result = x as f32 / 10.0;
            println!("x={} result={}", x, result);
            relative_eq!(
                rectangle.distance(&CENTER, &Point::new(CENTER.x - x as i32, CENTER.y)),
                result
            );
            relative_eq!(
                rectangle.distance(&CENTER, &Point::new(CENTER.x + x as i32, CENTER.y)),
                result
            );
        }

        for y in 0..16 {
            println!("y={}", y);
            relative_eq!(
                rectangle.distance(&CENTER, &Point::new(CENTER.x, CENTER.y - y as i32)),
                0.0
            );
            relative_eq!(
                rectangle.distance(&CENTER, &Point::new(CENTER.x, CENTER.y + y as i32)),
                0.0
            );
        }

        for y in 16..36 {
            let result = (y - 15) as f32 / 10.0;
            println!("y={} result={}", y, result);
            relative_eq!(
                rectangle.distance(&CENTER, &Point::new(CENTER.x, CENTER.y - y as i32)),
                result
            );
            relative_eq!(
                rectangle.distance(&CENTER, &Point::new(CENTER.x, CENTER.y + y as i32)),
                result
            );
        }
    }
}
