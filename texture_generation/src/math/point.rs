use crate::math::size::Size;
use serde::{Deserialize, Serialize};
use std::ops::{Add, Div, Mul, Sub};

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
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    /// Returns a new point.
    ///
    /// ```
    ///# use texture_generation::math::point::Point;
    /// let point = Point::new(2, 3);
    /// assert_eq!(point.x, 2);
    /// assert_eq!(point.y, 3);
    /// ```
    pub const fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    /// Returns a new point with switched x & <height>.
    ///
    /// ```
    ///# use texture_generation::math::point::Point;
    /// let point = Point::new(-10, 30);
    /// assert_eq!(point.flip(), Point::new(30, -10));
    /// ```
    pub fn flip(&self) -> Point {
        Point::new(self.y, self.x)
    }

    /// Calculates the euclidean distance to another point.
    ///
    /// ```
    ///# use texture_generation::math::point::Point;
    /// let a = Point::new(1, 2);
    /// let b = Point::new(4, 6);
    ///
    /// assert_eq!(a.calculate_distance(&a), 0.0);
    /// assert_eq!(a.calculate_distance(&b), 5.0);
    /// assert_eq!(b.calculate_distance(&a), 5.0);
    /// ```
    pub fn calculate_distance(&self, point: &Point) -> f32 {
        (self.x as f32 - point.x as f32).hypot(self.y as f32 - point.y as f32)
    }

    /// Returns a new point with the max coordinates of both points.
    ///
    /// ```
    ///# use texture_generation::math::point::Point;
    /// let a = Point::new(1, 6);
    /// let b = Point::new(4, 2);
    /// let max = Point::new(4, 6);
    ///
    /// assert_eq!(a.max(&a), a);
    /// assert_eq!(a.max(&b), max);
    /// assert_eq!(b.max(&a), max);
    /// assert_eq!(b.max(&b), b);
    /// ```
    pub fn max(&self, point: &Point) -> Self {
        Point::new(self.x.max(point.x), self.y.max(point.y))
    }

    /// Returns a new point with the min coordinates of both points.
    ///
    /// ```
    ///# use texture_generation::math::point::Point;
    /// let a = Point::new(1, 6);
    /// let b = Point::new(4, 2);
    /// let min = Point::new(1, 2);
    ///
    /// assert_eq!(a.min(&a), a);
    /// assert_eq!(a.min(&b), min);
    /// assert_eq!(b.min(&a), min);
    /// assert_eq!(b.min(&b), b);
    /// ```
    pub fn min(&self, point: &Point) -> Self {
        Point::new(self.x.min(point.x), self.y.min(point.y))
    }

    /// Returns a new point with the min coordinates of the point & size.
    ///
    /// ```
    ///# use texture_generation::math::point::Point;
    ///# use texture_generation::math::size::Size;
    /// let point = Point::new(1, 6);
    /// let size = Size::new(4, 2);
    /// let min = Point::new(1, 2);
    ///
    /// assert_eq!(point.limit_to(&size), Point::new(1, 2));
    /// ```
    pub fn limit_to(&self, size: &Size) -> Self {
        Point::new(
            self.x.min(size.width() as i32),
            self.y.min(size.height() as i32),
        )
    }
}

/// Add a [`Point`] to another [`Point`].
///
/// ```
///# use texture_generation::math::point::Point;
/// let a = Point::new(1, 2);
/// let b = Point::new(30, 50);
/// let result = Point::new(31, 52);
///
/// assert_eq!(a + b, result);
/// assert_eq!(b + a, result);
/// ```
impl Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

/// Adds an integer to a [`Point`].
///
/// ```
///# use texture_generation::math::point::Point;
/// let point = Point::new(1, 2);
///
/// assert_eq!(point + 2, Point::new(3, 4));
/// ```
impl Add<i32> for Point {
    type Output = Point;

    fn add(self, value: i32) -> Point {
        Point::new(self.x + value, self.y + value)
    }
}

/// Adds a [`Size`] to a [`Point`].
///
/// ```
///# use texture_generation::math::point::Point;
///# use texture_generation::math::size::Size;
/// let point = Point::new(1, 2);
/// let size = Size::new(30, 50);
///
/// assert_eq!(point + size, Point::new(31, 52));
/// ```
impl Add<Size> for Point {
    type Output = Point;

    fn add(self, size: Size) -> Point {
        Point::new(self.x + size.width() as i32, self.y + size.height() as i32)
    }
}

/// Subtracts an integer from a [`Point`].
///
/// ```
///# use texture_generation::math::point::Point;
/// let point = Point::new(1, 2);
///
/// assert_eq!(point - 2, Point::new(-1, 0));
/// ```
impl Sub<i32> for Point {
    type Output = Point;

    fn sub(self, value: i32) -> Point {
        Point::new(self.x - value, self.y - value)
    }
}

/// Subtracts a [`Point`] from another [`Point`].
///
/// ```
///# use texture_generation::math::point::Point;
/// let a = Point::new(1, 2);
/// let b = Point::new(30, 50);
///
/// assert_eq!(a - b, Point::new(-29, -48));
/// assert_eq!(b - a, Point::new(29, 48));
/// ```
impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

/// Subtracts a [`Size`] from a [`Point`].
///
/// ```
///# use texture_generation::math::point::Point;
///# use texture_generation::math::size::Size;
/// let point = Point::new(1, 2);
/// let size = Size::new(30, 50);
///
/// assert_eq!(point - size, Point::new(-29, -48));
/// ```
impl Sub<Size> for Point {
    type Output = Point;

    fn sub(self, size: Size) -> Point {
        Point::new(self.x - size.width() as i32, self.y - size.height() as i32)
    }
}

/// Multiplies a [`Point`] by an integer.
///
/// ```
///# use texture_generation::math::point::Point;
/// let point = Point::new(10, 30);
///
/// assert_eq!(point * 2, Point::new(20, 60));
/// ```
impl Mul<u32> for Point {
    type Output = Self;

    fn mul(self, value: u32) -> Self::Output {
        Point::new(self.x * value as i32, self.y * value as i32)
    }
}

/// Multiplies a [`Point`] by a [`Size`].
///
/// ```
///# use texture_generation::math::point::Point;
///# use texture_generation::math::size::Size;
/// let point = Point::new(10, 30);
/// let size = Size::new(20, 40);
///
/// assert_eq!(point * size, Point::new(200, 1200));
/// ```
impl Mul<Size> for Point {
    type Output = Self;

    fn mul(self, size: Size) -> Self::Output {
        Point::new(self.x * size.width() as i32, self.y * size.height() as i32)
    }
}

/// Divides a [`Point`] by an integer.
///
/// ```
///# use texture_generation::math::point::Point;
/// let point = Point::new(10, -30);
///
/// assert_eq!(point / 2, Point::new(5, -15));
/// ```
impl Div<u32> for Point {
    type Output = Self;

    fn div(self, value: u32) -> Self::Output {
        Point::new(self.x / value as i32, self.y / value as i32)
    }
}
