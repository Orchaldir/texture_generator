use crate::math::point::Point;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Shape {
    Circle { radius: u32 },
}

impl Shape {
    pub fn new_circle(radius: u32) -> Shape {
        Shape::Circle { radius }
    }

    /// Calculates the euclidean distance to a [`Point`].
    ///
    /// ```
    ///# use texture_generator::math::point::Point;
    ///# use texture_generator::math::shape::Shape;
    /// let center = Point::new(10, 20);
    /// let border = Point::new(7, 20);
    /// let outside = Point::new(13, 24);
    /// let circle = Shape::new_circle(3);
    ///
    /// assert_eq!(circle.distance_to_border(&center, &center), -3.0);
    /// assert_eq!(circle.distance_to_border(&center, &outside), 2.0);
    /// assert_eq!(circle.distance_to_border(&center, &border), 0.0);
    /// ```
    pub fn distance_to_border(&self, center: &Point, point: &Point) -> f32 {
        match self {
            Shape::Circle { radius } => center.calculate_distance(point) - *radius as f32,
        }
    }

    /// Is the [`Point`] inside?
    ///
    /// ```
    ///# use texture_generator::math::point::Point;
    ///# use texture_generator::math::shape::Shape;
    /// let center = Point::new(10, 20);
    /// let border = Point::new(7, 20);
    /// let outside = Point::new(13, 24);
    /// let circle = Shape::new_circle(3);
    ///
    /// assert!(circle.is_inside(&center, &center));
    /// assert!(!circle.is_inside(&center, &outside));
    /// assert!(circle.is_inside(&center, &border));
    /// ```
    pub fn is_inside(&self, center: &Point, point: &Point) -> bool {
        match self {
            Shape::Circle { radius } => center.calculate_distance(point) <= *radius as f32,
        }
    }
}
