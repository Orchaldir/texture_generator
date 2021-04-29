use crate::generation::component::Component;
use crate::generation::data::Data;
use crate::math::aabb::AABB;
use crate::math::point::Point;
use crate::math::size::Size;

/// The [Herringbone Pattern](https://en.wikipedia.org/wiki/RGB_color_model).
#[derive(Clone, Debug, PartialEq)]
pub struct HerringbonePattern {
    side: i32,
    multiplier: u32,
    horizontal_size: Size,
    vertical_size: Size,
    repeating_side: u32,
    horizontal_component: Component,
    vertical_component: Component,
}

impl HerringbonePattern {
    pub fn new(
        side: u32,
        multiplier: u32,
        horizontal_component: Component,
        vertical_component: Component,
    ) -> HerringbonePattern {
        let repeating_side = calculate_repeating_side(side, multiplier);
        let size = Size::new(side * multiplier, side);

        HerringbonePattern {
            side: side as i32,
            multiplier,
            horizontal_size: size,
            vertical_size: size.flip(),
            repeating_side,
            horizontal_component,
            vertical_component,
        }
    }

    /// Generates the pattern in the area defined by the [`AABB`].
    pub fn generate(&self, data: &mut dyn Data, outer: &AABB, inner: &AABB) {
        let start = self.calculate_repeating_point(inner.start());
        let end = self.calculate_repeating_point(inner.end()) + 1i32;
        let limited = outer.limit(inner);

        for y in start.y..end.y {
            for x in start.x..end.x {
                self.generate_repeatable(data, &limited, x, y);
            }
        }
    }

    fn generate_repeatable(&self, data: &mut dyn Data, limited: &AABB, x: i32, y: i32) {
        let start = Point::new(x, y) * self.repeating_side;
        let limited = AABB::new(start, Size::square(self.repeating_side)).limit(limited);
        let multiplier = self.multiplier as i32;

        for i in 0..(multiplier * 2) {
            let aabb = self.get_horizontal_aabb(start, i, i);
            self.horizontal_component.generate(data, &limited, &aabb);
        }

        for i in 0..(multiplier - 1) {
            let aabb = self.get_horizontal_aabb(start, i - multiplier + 1, i + multiplier + 1);
            self.horizontal_component.generate(data, &limited, &aabb);
        }

        for i in 0..(multiplier * 2 - 1) {
            let aabb = self.get_vertical_aabb(start, i, i + 1);
            self.vertical_component.generate(data, &limited, &aabb);
        }

        for i in 0..(multiplier) {
            let aabb = self.get_vertical_aabb(start, i + multiplier, i - multiplier + 1);
            self.vertical_component.generate(data, &limited, &aabb);
        }
    }

    fn get_horizontal_aabb(&self, start: Point, offset_x: i32, offset_y: i32) -> AABB {
        let point = Point::new(
            start.x + offset_x * self.side,
            start.y + offset_y * self.side,
        );
        AABB::new(point, self.horizontal_size)
    }

    fn get_vertical_aabb(&self, start: Point, offset_x: i32, offset_y: i32) -> AABB {
        let point = Point::new(
            start.x + offset_x * self.side,
            start.y + offset_y * self.side,
        );
        AABB::new(point, self.vertical_size)
    }

    /// In which repeating area is this point?
    fn calculate_repeating_point(&self, point: Point) -> Point {
        point / self.repeating_side
    }
}

/// How large is the repeating area of this pattern?
fn calculate_repeating_side(side: u32, multiplier: u32) -> u32 {
    side * multiplier * 2
}
