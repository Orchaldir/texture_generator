use crate::generation::component::Component;
use crate::generation::data::texture::Texture;
use crate::generation::data::Data;
use crate::math::aabb::AABB;
use crate::math::point::Point;
use crate::math::size::Size;
use anyhow::{bail, Result};

#[svgbobdoc::transform]
/// The [Herringbone Pattern](https://en.wikipedia.org/wiki/Herringbone_pattern) alternates horizontal & vertical [`Component`]s.
///
/// # Diagram
///
/// ```svgbob
///               *-----*-----------*
///               |     |           |
///   *-----*-----*     *-----*-----*-----*
///   |           |     |     |           |
///   *-----*-----*-----*     *-----*-----*
///   |     |           |     |     |
///   |     *-----*-----*-----*     *-----*
///   |     |     |           |     |     |
///   *-----*     *-----*-----*-----*     |
///         |     |     |           |     |
///   *-----*-----*     *-----*-----*-----*
///   |           |     |     |           |
///   *-----------*-----*     *-----------*
/// ```
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
    ) -> Result<HerringbonePattern> {
        if side == 0 {
            bail!("Argument 'side' needs to be greater than 0");
        } else if multiplier < 2 {
            bail!("Argument 'multiplier' needs to be greater than 1");
        }

        let repeating_side = calculate_repeating_side(side, multiplier);
        let size = Size::new(side * multiplier, side);

        Ok(HerringbonePattern {
            side: side as i32,
            multiplier,
            horizontal_size: size,
            vertical_size: size.flip(),
            repeating_side,
            horizontal_component,
            vertical_component,
        })
    }

    /// Generates the pattern in all the repeating areas intersected by the [`AABB`].
    pub fn generate(&self, texture: &mut Texture, data: &Data) {
        let inner = data.get_aabbs().get_inner();
        let start = self.calculate_repeating_point(inner.start());
        let end = self.calculate_repeating_point(inner.end()) + 1i32;

        for y in start.y..end.y {
            for x in start.x..end.x {
                self.generate_repeating_area(texture, data, x, y);
            }
        }
    }

    /// Generates the repeating area of the Herringbone pattern.
    fn generate_repeating_area(&self, texture: &mut Texture, combined: &Data, x: i32, y: i32) {
        let start = Point::new(x, y) * self.repeating_side;
        let repeating_aabb = AABB::new(start, Size::square(self.repeating_side));
        let mut repeating_data = combined.transform(repeating_aabb);
        let multiplier = self.multiplier as i32;

        for i in 0..(multiplier * 2) {
            let aabb = self.get_horizontal_aabb(start, i, i);
            self.horizontal_component
                .generate(texture, &repeating_data.next(aabb));
        }

        for i in 0..(multiplier - 1) {
            let aabb = self.get_horizontal_aabb(start, i - multiplier + 1, i + multiplier + 1);
            self.horizontal_component
                .generate(texture, &repeating_data.next(aabb));
        }

        for i in 0..(multiplier * 2 - 1) {
            let aabb = self.get_vertical_aabb(start, i, i + 1);
            self.vertical_component
                .generate(texture, &repeating_data.next(aabb));
        }

        for i in 0..(multiplier) {
            let aabb = self.get_vertical_aabb(start, i + multiplier, i - multiplier + 1);
            self.vertical_component
                .generate(texture, &repeating_data.next(aabb));
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generation::component::rendering::RenderingComponent;
    use crate::generation::data::texture::Texture;
    use crate::math::color::{Color, BLUE, PINK, WHITE};
    use crate::math::size::Size;

    #[test]
    #[should_panic]
    fn test_new_side_too_small() {
        HerringbonePattern::new(0, 2, Component::Mock(1), Component::Mock(2)).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_new_multiplier_too_small() {
        HerringbonePattern::new(1, 1, Component::Mock(1), Component::Mock(2)).unwrap();
    }

    #[test]
    fn test_herringbone_pattern() {
        let size = Size::square(8);
        let aabb = AABB::with_size(size);
        let mut texture = Texture::new(size, WHITE);
        let horizontal = create_component(PINK);
        let vertical = create_component(BLUE);
        let pattern = HerringbonePattern::new(1, 2, horizontal, vertical).unwrap();

        pattern.generate(&mut texture, &Data::for_texture(aabb));

        #[rustfmt::skip]
        let expected_colors = vec![
            PINK, PINK, BLUE, BLUE, PINK, PINK, BLUE, BLUE,
            BLUE, PINK, PINK, BLUE, BLUE, PINK, PINK, BLUE,
            BLUE, BLUE, PINK, PINK, BLUE, BLUE, PINK, PINK,
            PINK, BLUE, BLUE, PINK, PINK, BLUE, BLUE, PINK,
            PINK, PINK, BLUE, BLUE, PINK, PINK, BLUE, BLUE,
            BLUE, PINK, PINK, BLUE, BLUE, PINK, PINK, BLUE,
            BLUE, BLUE, PINK, PINK, BLUE, BLUE, PINK, PINK,
            PINK, BLUE, BLUE, PINK, PINK, BLUE, BLUE, PINK,
        ];

        assert_eq!(texture.get_color_data(), &expected_colors);
    }

    fn create_component(color: Color) -> Component {
        Component::Rendering(Box::new(RenderingComponent::new_fill_area(color, 1)))
    }
}
