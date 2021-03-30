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
                let distance = Shape::calculate_rectangle_distance(center, point, half_x, half_y);
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

    /// Calculates the euclidean distance from the shape's border to a [`Point`].
    /// A positive distance means the point is outside.
    ///
    /// ```
    ///# use texture_generator::math::point::Point;
    ///# use texture_generator::math::shape::Shape;
    /// let center = Point::new(10, 20);
    /// let border = Point::new(7, 20);
    /// let outside = Point::new(13, 24);
    /// let circle = Shape::new_circle(3).unwrap();
    ///
    /// assert_eq!(circle.distance_to_border(&center, &center), -3.0);
    /// assert_eq!(circle.distance_to_border(&center, &outside), 2.0);
    /// assert_eq!(circle.distance_to_border(&center, &border), 0.0);
    /// ```
    pub fn distance_to_border(&self, center: &Point, point: &Point) -> f32 {
        match self {
            Shape::Circle(radius) => center.calculate_distance(point) - *radius as f32,
            Shape::Rectangle { half_x, half_y } => {
                Shape::calculate_rectangle_distance(center, point, half_x, half_y)
            }
            Shape::RoundedRectangle { .. } => 0.0,
        }
    }

    fn calculate_rectangle_distance(
        center: &Point,
        point: &Point,
        half_x: &i32,
        half_y: &i32,
    ) -> f32 {
        let diff = *point - *center;
        (diff.x.abs() - *half_x).max(diff.y.abs() - *half_y) as f32
    }

    /// Is the [`Point`] inside?
    ///
    /// ```
    ///# use texture_generator::math::point::Point;
    ///# use texture_generator::math::shape::Shape;
    /// let center = Point::new(10, 20);
    /// let border = Point::new(7, 20);
    /// let outside = Point::new(13, 24);
    /// let circle = Shape::new_circle(3).unwrap();
    ///
    /// assert!(circle.is_inside(&center, &center));
    /// assert!(!circle.is_inside(&center, &outside));
    /// assert!(circle.is_inside(&center, &border));
    /// ```
    pub fn is_inside(&self, center: &Point, point: &Point) -> bool {
        match self {
            Shape::Circle(radius) => center.calculate_distance(point) <= *radius as f32,
            Shape::Rectangle { half_x, half_y } => {
                let diff = *point - *center;
                diff.x >= -*half_x && diff.x < *half_x && diff.y >= -*half_y && diff.y < *half_y
            }
            Shape::RoundedRectangle {
                half_x,
                half_y,
                radius,
            } => {
                let diff = *point - *center;
                let threshold_x = *half_x + *radius;
                let threshold_y = *half_y + *radius;
                diff.x >= -threshold_x
                    && diff.x < threshold_x
                    && diff.y >= -threshold_y
                    && diff.y < threshold_y
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

    #[test]
    fn test_distance_to_border_rectangle() {
        let size = Size::new(5, 7);
        let rectangle = Shape::new_rectangle(2, 4).unwrap();

        #[rustfmt::skip]
        let results = vec![
            1.0, 1.0,  1.0, 1.0, 1.0,
            1.0, 0.0,  0.0, 0.0, 1.0,
            1.0, 0.0, -1.0, 0.0, 1.0,
            1.0, 0.0, -1.0, 0.0, 1.0,
            1.0, 0.0, -1.0, 0.0, 1.0,
            1.0, 0.0,  0.0, 0.0, 1.0,
            1.0, 1.0,  1.0, 1.0, 1.0
        ];

        for (index, result) in results.iter().enumerate() {
            assert_eq!(
                rectangle.distance_to_border(&CENTER, &size.to_point(index)),
                *result
            );
        }
    }

    #[test]
    fn test_is_inside_rectangle() {
        let size = Size::new(4, 6);
        let rectangle = Shape::new_rectangle(2, 4).unwrap();

        #[rustfmt::skip]
        let results = vec![
            false, false, false, false,
            false,  true,  true, false,
            false,  true,  true, false,
            false,  true,  true, false,
            false,  true,  true, false,
            false, false, false, false
        ];

        for (index, result) in results.iter().enumerate() {
            assert_eq!(rectangle.is_inside(&CENTER, &size.to_point(index)), *result);
        }
    }
}
