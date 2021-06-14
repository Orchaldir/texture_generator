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

    /// Limit the other aabb to this one.
    pub fn limit(&self, other: &AABB) -> AABB {
        let start = self.limit_to(&other.start);
        let end = self.limit_to(&other.end);
        let size = Size::new((end.x - start.x) as u32, (end.y - start.y) as u32);
        AABB { start, end, size }
    }

    /// Limit a [`Point`] to this aabb.
    pub fn limit_to(&self, point: &Point) -> Point {
        point.max(&self.start).min(&self.end)
    }

    /// Rotates the origin of the texture clockwise.
    pub fn rotate_origin(&self, texture_size: Size) -> Self {
        let y = texture_size.width() as i32 - self.end.x;
        let start = Point::new(self.start.y, y);
        AABB::new(start, self.size.flip())
    }

    /// Rotates the origin of the texture counter clockwise.
    pub fn rotate_origin_reverse(&self, texture_size: Size) -> Self {
        let x = texture_size.height() as i32 - self.end.y;
        let start = Point::new(x, self.start.x);
        AABB::new(start, self.size.flip())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const START: Point = Point::new(1, 2);
    const SIZE: Size = Size::new(3, 4);
    const TEXTURE_SIZE: Size = Size::new(20, 30);

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

    #[test]
    fn test_limit_inside() {
        let aabb0 = AABB::with_size(Size::square(10));
        let aabb1 = AABB::new(Point::new(1, 2), Size::new(3, 4));

        assert_eq!(aabb0.limit(&aabb1), aabb1);
        assert_eq!(aabb1.limit(&aabb0), aabb1);
        assert_eq!(aabb1.limit(&aabb1), aabb1);
    }

    #[test]
    fn test_rotate_origin() {
        assert_eq!(aabb0().rotate_origin(TEXTURE_SIZE), aabb1());
    }

    #[test]
    fn test_rotate_origin_4_times() {
        let size = TEXTURE_SIZE;
        let aabb = aabb0();

        assert_eq!(
            aabb.rotate_origin(size)
                .rotate_origin(size)
                .rotate_origin(size)
                .rotate_origin(size),
            aabb
        );
    }

    #[test]
    fn test_rotate_origin_reverse() {
        let size = TEXTURE_SIZE.flip();

        assert_eq!(aabb1().rotate_origin_reverse(size), aabb0());
    }

    #[test]
    fn test_rotate_origin_reverse_4_times() {
        let size = TEXTURE_SIZE;
        let aabb = aabb0();

        assert_eq!(
            aabb.rotate_origin_reverse(size)
                .rotate_origin_reverse(size)
                .rotate_origin_reverse(size)
                .rotate_origin_reverse(size),
            aabb
        );
    }

    fn aabb0() -> AABB {
        AABB::new(Point::new(3, 2), Size::new(5, 4))
    }

    fn aabb1() -> AABB {
        AABB::new(Point::new(2, 12), Size::new(4, 5))
    }
}
