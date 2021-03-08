use crate::math::size::Size;

#[svgbobdoc::transform]
/// Defines a point in 2 dimensions.
///
/// # Diagram
///
/// ```svgbob
///      0  1
///   +--*--*----> x-axis
///   |
/// 0 *
///   |
/// 1 *
///   |
/// 2 *     * (1,2)
///   |
///   v
/// y-axis
/// ```
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    /// Returns a new point.
    ///
    /// ```
    ///# use texture_generator::math::point::Point;
    /// let point = Point::new(2, 3);
    /// assert_eq!(point.x, 2);
    /// assert_eq!(point.y, 3);
    /// ```
    pub const fn new(x: u32, y: u32) -> Point {
        Point { x, y }
    }

    /// Creates a new point by adding a [`Size`] to it.
    ///
    /// ```
    ///# use texture_generator::math::point::Point;
    ///# use texture_generator::math::size::Size;
    /// let point = Point::new(1, 2);
    /// let size = Size::new(30, 50);
    ///
    /// assert_eq!(point.add_size(&size), Point::new(31, 52));
    /// ```
    pub fn add_size(&self, size: &Size) -> Point {
        Point::new(self.x + size.width(), self.y + size.height())
    }

    /// Calculates the euclidean distance to another point.
    ///
    /// ```
    ///# use texture_generator::math::point::Point;
    /// let a = Point::new(1, 2);
    /// let b = Point::new(4, 6);
    ///
    /// assert_eq!(a.calculate_distance(&a), 0.0);
    /// assert_eq!(a.calculate_distance(&b), 5.0);
    /// assert_eq!(b.calculate_distance(&a), 5.0);
    /// ```
    pub fn calculate_distance(&self, point: &Point) -> f32 {
        ((self.x as f32 - point.x as f32).powf(2.0) + (self.y as f32 - point.y as f32).powf(2.0))
            .sqrt()
    }
}
