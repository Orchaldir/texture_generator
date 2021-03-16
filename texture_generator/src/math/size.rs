use crate::math::point::Point;
use serde::{Deserialize, Serialize};

#[svgbobdoc::transform]
/// Defines the size of something (e.g. a texture) in 2 dimensions.
///
/// # Diagram
///
/// ```svgbob
///       0   1
///   +----------> x-axis
///   |
///   | +---+---+
/// 0 | | 0 | 1 |
///   | +---+---+
/// 1 | | 2 | 3 |
///   | +---+---+
/// 2 | | 4 | 5 |
///   | +---+---+
///   v
/// y-axis
/// ```
///
/// A size with width 2 & height 3.
/// The numbers are indices of each cell.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Size {
    width: u32,
    height: u32,
}

impl Size {
    /// Returns a new size
    pub const fn new(width: u32, height: u32) -> Size {
        Size { width, height }
    }

    /// Returns a new size divided by a value.
    ///
    /// ```
    ///# use texture_generator::math::size::Size;
    /// let size = Size::new(10, 30);
    /// assert_eq!(size.divide(2), Size::new(5, 15));
    /// ```
    pub fn divide(&self, value: u32) -> Size {
        Size {
            width: self.width / value,
            height: self.height / value,
        }
    }

    /// Returns the number of cells covered by this size.
    ///
    /// ```
    ///# use texture_generator::math::size::Size;
    /// let size = Size::new(2, 3);
    /// assert_eq!(size.get_number_of_cells(), 6);
    /// ```
    pub fn get_number_of_cells(&self) -> usize {
        (self.width * self.height) as usize
    }

    /// Returns the size along the x-axis.
    ///
    /// ```
    ///# use texture_generator::math::size::Size;
    /// let size = Size::new(2, 3);
    /// assert_eq!(size.width(), 2);
    /// ```
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the size along the y-axis.
    ///
    /// ```
    ///# use texture_generator::math::size::Size;
    /// let size = Size::new(2, 3);
    /// assert_eq!(size.height(), 3);
    /// ```
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Is the [`Point`] inside?
    ///
    /// ```
    ///# use texture_generator::math::point::Point;
    ///# use texture_generator::math::size::Size;
    /// let size = Size::new(2, 3);
    /// let inside = Point::new(1, 2);
    /// let outside = Point::new(4, 5);
    /// let negative = Point::new(-1, 1);
    ///
    /// assert!(size.is_inside(&inside));
    /// assert!(!size.is_inside(&outside));
    /// assert!(!size.is_inside(&negative));
    /// ```
    pub fn is_inside(&self, point: &Point) -> bool {
        point.x >= 0 && point.y >= 0 && point.x < self.width as i32 && point.y < self.height as i32
    }

    /// Converts an index to the x-coordinate of the equivalent [`Point`].
    ///
    /// ```
    ///# use texture_generator::math::size::Size;
    /// let size = Size::new(2, 3);
    /// assert_eq!(size.to_x(5), 1);
    /// ```
    pub fn to_x(&self, index: usize) -> i32 {
        index as i32 % self.width as i32
    }

    /// Converts an index to the y-coordinate of the equivalent [`Point`].
    ///
    /// ```
    ///# use texture_generator::math::size::Size;
    /// let size = Size::new(2, 3);
    /// assert_eq!(size.to_y(5), 2);
    /// ```
    pub fn to_y(&self, index: usize) -> i32 {
        index as i32 / self.width as i32
    }

    /// Converts an index to the equivalent [`Point`].
    ///
    /// ```
    ///# use texture_generator::math::point::Point;
    ///# use texture_generator::math::size::Size;
    /// let size = Size::new(2, 3);
    /// assert_eq!(size.to_point(5), Point::new(1,2));
    /// ```
    pub fn to_point(&self, index: usize) -> Point {
        Point::new(self.to_x(index), self.to_y(index))
    }

    /// Converts a [`Point`] to the equivalent index, if it is inside.
    ///
    /// ```
    ///# use texture_generator::math::point::Point;
    ///# use texture_generator::math::size::Size;
    /// let size = Size::new(2, 3);
    /// let inside = Point::new(1, 2);
    /// let outside = Point::new(4, 5);
    /// let negative = Point::new(1, -1);
    ///
    /// assert_eq!(size.to_index(&inside), Some(5));
    /// assert_eq!(size.to_index(&outside), None);
    /// assert_eq!(size.to_index(&negative), None);
    /// ```
    pub fn to_index(&self, point: &Point) -> Option<usize> {
        if self.is_inside(point) {
            return Some(self.to_index_risky(point));
        }

        None
    }

    /// Converts a [`Point`] to the equivalent index, but returns a wrong result if it is outside.
    ///
    /// ```
    ///# use texture_generator::math::point::Point;
    ///# use texture_generator::math::size::Size;
    /// let size = Size::new(2, 3);
    /// assert_eq!(size.to_index_risky(&Point::new(1, 2)), 5);
    /// ```
    pub fn to_index_risky(&self, point: &Point) -> usize {
        (point.y * self.width as i32 + point.x) as usize
    }

    /// Converts a [`Point`] to the equivalent index, but returns a wrong result if it is outside.
    ///
    /// ```
    ///# use texture_generator::math::point::Point;
    ///# use texture_generator::math::size::Size;
    /// let size = Size::new(2, 3);
    /// assert_eq!(size.convert_x_y(1, 2), 5);
    /// ```
    pub fn convert_x_y(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }
}
