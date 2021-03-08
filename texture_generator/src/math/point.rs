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
}
