use crate::math::point::Point;
use crate::math::size::Size;

pub type AABB = AxisAlignedBoundingBox;

#[svgbobdoc::transform]
/// Defines an axis aligned bounding box.
///
/// # Diagram
///
/// ```svgbob
///   +---------------------> x-axis
///   |     start
///   |     *---------*
///   |     |         |
///   |     |         |
///   |     |         |
///   |     *---------*
///   |           end = start + size
///   v
/// y-axis
/// ```
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct AxisAlignedBoundingBox {
    start: Point,
    end: Point,
    size: Size,
}

impl AxisAlignedBoundingBox {
    /// Returns a new axis aligned bounding box.
    ///
    /// ```
    ///# use texture_generation::math::aabb::AxisAlignedBoundingBox;
    ///# use texture_generation::math::point::Point;
    ///# use texture_generation::math::size::Size;
    /// let start = Point::new(2, 3);
    /// let size = Size::new(30, 50);
    /// let aabb = AxisAlignedBoundingBox::new(start, size);
    ///
    /// assert_eq!(aabb.start(), start);
    /// assert_eq!(aabb.end(), Point::new(32, 53));
    /// assert_eq!(aabb.size(), size);
    /// ```
    pub fn new(start: Point, size: Size) -> Self {
        let end = start + size;
        AxisAlignedBoundingBox { start, end, size }
    }

    /// Returns a new axis aligned bounding box.
    ///
    /// ```
    ///# use texture_generation::math::aabb::AxisAlignedBoundingBox;
    ///# use texture_generation::math::point::Point;
    ///# use texture_generation::math::size::Size;
    /// let size = Size::new(30, 50);
    /// let aabb = AxisAlignedBoundingBox::with_size(size);
    ///
    /// assert_eq!(aabb.start(), Point::new(0, 0));
    /// assert_eq!(aabb.end(), Point::new(30, 50));
    /// assert_eq!(aabb.size(), size);
    /// ```
    pub fn with_size(size: Size) -> Self {
        let start = Point::new(0, 0);
        let end = start + size;
        AxisAlignedBoundingBox { start, end, size }
    }

    pub fn start(&self) -> Point {
        self.start
    }

    /// Returns the center of the axis aligned bounding box.
    ///
    /// ```
    ///# use texture_generation::math::aabb::AxisAlignedBoundingBox;
    ///# use texture_generation::math::point::Point;
    ///# use texture_generation::math::size::Size;
    /// let start = Point::new(2, 3);
    /// let size = Size::new(30, 50);
    /// let aabb = AxisAlignedBoundingBox::new(start, size);
    ///
    /// assert_eq!(aabb.center(), Point::new(17, 28));
    /// ```
    pub fn center(&self) -> Point {
        self.start + self.size.divide(2)
    }

    pub fn end(&self) -> Point {
        self.end
    }

    pub fn size(&self) -> Size {
        self.size
    }

    /// Is the [`Point`] inside?
    ///
    /// ```
    ///# use texture_generation::math::aabb::AxisAlignedBoundingBox;
    ///# use texture_generation::math::point::Point;
    ///# use texture_generation::math::size::Size;
    /// let start = Point::new(10, 20);
    /// let size = Size::new(30, 40);
    /// let aabb = AxisAlignedBoundingBox::new(start, size);
    ///
    /// assert!(aabb.is_inside(&Point::new(25, 40)));
    /// assert!(!aabb.is_inside(&Point::new(0, 0)));
    /// ```
    pub fn is_inside(&self, point: &Point) -> bool {
        point.x >= self.start.x
            && point.y >= self.start.y
            && point.x < self.end.x
            && point.y < self.end.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const START: Point = Point::new(1, 2);
    const SIZE: Size = Size::new(3, 4);

    #[test]
    fn test_is_inside() {
        let test_size = Size::new(6, 8);
        let aabb = AxisAlignedBoundingBox::new(START, SIZE);

        #[rustfmt::skip]
        let results = vec![
            false, false, false, false, false, false,
            false, false, false, false, false, false,
            false,  true,  true,  true, false, false,
            false,  true,  true,  true, false, false,
            false,  true,  true,  true, false, false,
            false,  true,  true,  true, false, false,
            false, false, false, false, false, false,
            false, false, false, false, false, false
        ];

        for (index, result) in results.iter().enumerate() {
            assert_eq!(aabb.is_inside(&test_size.to_point(index)), *result);
        }
    }
}
