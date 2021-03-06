use crate::math::aabb::AABB;
use crate::math::point::Point;
use serde::{Deserialize, Serialize};
use std::ops::Mul;

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

    /// Returns a size with equal width & height.
    pub const fn square(size: u32) -> Size {
        Size::new(size, size)
    }

    /// Returns a new size with switched width & height.
    ///
    /// ```
    ///# use texture_generation::math::size::Size;
    /// let size = Size::new(10, 30);
    /// assert_eq!(size.flip(), Size::new(30, 10));
    /// ```
    pub fn flip(&self) -> Size {
        Size::new(self.height, self.width)
    }

    /// Returns a new size divided by a value.
    ///
    /// ```
    ///# use texture_generation::math::size::Size;
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
    ///# use texture_generation::math::size::Size;
    /// let size = Size::new(2, 3);
    /// assert_eq!(size.len(), 6);
    /// ```
    pub fn len(&self) -> usize {
        (self.width * self.height) as usize
    }

    /// Returns the size along the x-axis.
    ///
    /// ```
    ///# use texture_generation::math::size::Size;
    /// let size = Size::new(2, 3);
    /// assert_eq!(size.width(), 2);
    /// ```
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the size along the y-axis.
    ///
    /// ```
    ///# use texture_generation::math::size::Size;
    /// let size = Size::new(2, 3);
    /// assert_eq!(size.height(), 3);
    /// ```
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Is the [`Point`] inside?
    ///
    /// ```
    ///# use texture_generation::math::point::Point;
    ///# use texture_generation::math::size::Size;
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

    /// Is the area inside?
    ///
    /// ```
    ///# use texture_generation::math::point::Point;
    ///# use texture_generation::math::size::Size;
    /// let size = Size::new(3, 4);
    /// let area_size = Size::new(2, 1);
    /// let inside = Point::new(1, 2);
    /// let outside = Point::new(2, 2);
    /// let negative = Point::new(-1, 1);
    ///
    /// assert!(size.is_area_inside(&inside, &area_size));
    /// assert!(!size.is_area_inside(&outside, &area_size));
    /// assert!(!size.is_area_inside(&negative, &area_size));
    /// ```
    pub fn is_area_inside(&self, start: &Point, size: &Size) -> bool {
        start.x >= 0
            && start.y >= 0
            && start.x as u32 + size.width <= self.width
            && start.y as u32 + size.height <= self.height
    }

    /// Is the [`AABB`] inside?
    ///
    /// ```
    ///# use texture_generation::math::point::Point;
    ///# use texture_generation::math::size::Size;
    ///# use texture_generation::math::aabb::AABB;
    /// let size = Size::new(3, 4);
    /// let area_size = Size::new(2, 1);
    /// let inside = Point::new(1, 2);
    /// let outside = Point::new(2, 2);
    /// let negative = Point::new(-1, 1);
    ///
    /// assert!(size.is_aabb_inside(&AABB::new(inside, area_size)));
    /// assert!(!size.is_aabb_inside(&AABB::new(outside, area_size)));
    /// assert!(!size.is_aabb_inside(&AABB::new(negative, area_size)));
    /// ```
    pub fn is_aabb_inside(&self, aabb: &AABB) -> bool {
        self.is_area_inside(&aabb.start(), &aabb.size())
    }

    /// Converts an index to the x-coordinate of the equivalent [`Point`].
    ///
    /// ```
    ///# use texture_generation::math::size::Size;
    /// let size = Size::new(2, 3);
    /// assert_eq!(size.to_x(5), 1);
    /// ```
    pub fn to_x(&self, index: usize) -> i32 {
        index as i32 % self.width as i32
    }

    /// Converts an index to the y-coordinate of the equivalent [`Point`].
    ///
    /// ```
    ///# use texture_generation::math::size::Size;
    /// let size = Size::new(2, 3);
    /// assert_eq!(size.to_y(5), 2);
    /// ```
    pub fn to_y(&self, index: usize) -> i32 {
        index as i32 / self.width as i32
    }

    /// Converts an index to the equivalent [`Point`].
    ///
    /// ```
    ///# use texture_generation::math::point::Point;
    ///# use texture_generation::math::size::Size;
    /// let size = Size::new(2, 3);
    /// assert_eq!(size.to_point(5), Point::new(1,2));
    /// ```
    pub fn to_point(&self, index: usize) -> Point {
        Point::new(self.to_x(index), self.to_y(index))
    }

    /// Converts a [`Point`] to the equivalent index, if it is inside.
    ///
    /// ```
    ///# use texture_generation::math::point::Point;
    ///# use texture_generation::math::size::Size;
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
    ///# use texture_generation::math::point::Point;
    ///# use texture_generation::math::size::Size;
    /// let size = Size::new(2, 3);
    /// assert_eq!(size.to_index_risky(&Point::new(1, 2)), 5);
    /// ```
    pub fn to_index_risky(&self, point: &Point) -> usize {
        (point.y * self.width as i32 + point.x) as usize
    }

    /// Converts a [`Point`] to the equivalent index, but returns a wrong result if it is outside.
    ///
    /// ```
    ///# use texture_generation::math::point::Point;
    ///# use texture_generation::math::size::Size;
    /// let size = Size::new(2, 3);
    /// assert_eq!(size.convert_x_y(1, 2), 5);
    /// ```
    pub fn convert_x_y(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }
}

/// Multiplies a [`Size`] with a float.
///
/// ```
///# use texture_generation::math::size::Size;
/// let vector = Size::new(10, 30);
///
/// assert_eq!(vector * 1.5, Size::new(15, 45));
/// ```
impl Mul<f32> for Size {
    type Output = Self;

    fn mul(self, value: f32) -> Size {
        Size::new(
            (self.width as f32 * value) as u32,
            (self.height as f32 * value) as u32,
        )
    }
}

/// Multiplies a [`Size`] with another.
///
/// ```
///# use texture_generation::math::size::Size;
/// let a = Size::new(10, 30);
/// let b = Size::new(2, 5);
///
/// assert_eq!(a * b, Size::new(20, 150));
/// ```
impl Mul<Size> for Size {
    type Output = Self;

    fn mul(self, other: Size) -> Size {
        Size::new(self.width * other.width, self.height * other.height)
    }
}
