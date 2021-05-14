use crate::generation::component::Component;
use crate::generation::data::texture::Texture;
use crate::generation::data::Data;
use crate::math::aabb::AABB;
use crate::math::point::Point;
use crate::math::size::Size;
use anyhow::{bail, Result};

#[svgbobdoc::transform]
/// A simple brick wall:
///
/// ```svgbob
///   +-----*-----*-----*-----*
///   |     |     |     |     |
///   *--*--*--*--*--*--*--*--*--*
///      |     |     |     |     |
///   *--*--*--*--*--*--*--*--*--*
///   |     |     |     |     |
///   *--*--*--*--*--*--*--*--*
/// ```
///
/// Use an offset of 0 and a square size for a square pattern.
///
/// ```svgbob
///   +--*--*--*
///   |  |  |  |
///   *--*--*--*
///   |  |  |  |
///   *--*--*--*
///   |  |  |  |
///   *--*--*--*
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct BrickPattern {
    name: String,
    brick: Size,
    offset: u32,
    component: Component,
}

impl BrickPattern {
    pub fn new<S: Into<String>>(
        name: S,
        brick: Size,
        offset: u32,
        component: Component,
    ) -> Result<BrickPattern> {
        if brick.width() < 1 {
            bail!("Argument 'brick.width' needs to be greater than 1");
        } else if brick.height() < 1 {
            bail!("Argument 'brick.height' needs to be greater than 1");
        } else if offset >= brick.width() {
            bail!("Argument 'offset' needs to be greater than or equal to 'brick.width'");
        }

        Ok(BrickPattern {
            name: name.into(),
            brick,
            offset,
            component,
        })
    }

    pub fn new_square<S: Into<String>>(
        name: S,
        side: u32,
        component: Component,
    ) -> Result<BrickPattern> {
        if side < 1 {
            bail!("Argument 'side' needs to be greater than 1");
        }

        Ok(BrickPattern {
            name: name.into(),
            brick: Size::square(side),
            offset: 0,
            component,
        })
    }

    /// Generates the pattern in all the repeating areas intersected by the [`AABB`].
    pub fn generate(&self, texture: &mut Texture, data: Data) {
        let aabb = data.get_inner();
        let (start_column, start_row) = self.calculate_column_row(aabb.start(), 0);
        let (end_column, end_row) = self.calculate_column_row(aabb.end(), 1);
        let total_bricks = self.calculate_total_bricks(texture);

        for row in start_row..end_row {
            let mut point = self.calculate_brick_start(start_column, row);
            let mut index = calculate_brick_index(total_bricks, start_column, row);

            for _column in start_column..end_column {
                let brick_aabb = AABB::new(point, self.brick);
                let brick_data = data.set(index, brick_aabb);

                self.component.generate(texture, &brick_data);

                index += 1;
                point.x += self.brick.width() as i32;
            }
        }
    }

    /// Calculates the starting point of a brick.
    fn calculate_brick_start(&self, column: i32, row: i32) -> Point {
        let mut x = column * self.brick.width() as i32;

        if is_offset_row(row) {
            x -= self.offset as i32;
        }

        Point::new(x, row * self.brick.height() as i32)
    }

    /// Calculates the number of columns & rows of bricks in the whole texture.
    fn calculate_total_bricks(&self, texture: &mut Texture) -> Size {
        Size::new(
            (texture.get_size().width() as f32 / self.brick.width() as f32).ceil() as u32,
            (texture.get_size().height() as f32 / self.brick.height() as f32).ceil() as u32,
        )
    }

    /// In which column & row of bricks is this point?
    fn calculate_column_row(&self, point: Point, bonus: i32) -> (i32, i32) {
        let row = point.y / self.brick.height() as i32;
        let mut x = point.x;

        if is_offset_row(row) {
            x += self.offset as i32;
        }

        let column = x / self.brick.width() as i32;
        (column + bonus, row + bonus)
    }
}

/// Is the offset applied to this row?
fn is_offset_row(row: i32) -> bool {
    row % 2 == 1
}

/// Calculates the global index of this brick.
fn calculate_brick_index(total_bricks: Size, column: i32, row: i32) -> usize {
    let off_by_1_for_every_two_rows = ((row - 1).max(0) as f32 / 2.0).ceil() as usize;
    total_bricks.convert_x_y(column as u32, row as u32) + off_by_1_for_every_two_rows
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generation::component::layout::tests::create_component;
    use crate::generation::data::texture::Texture;
    use crate::math::color::{RED, WHITE};
    use crate::math::size::Size;

    #[test]
    fn test_brick_wall() {
        let size = Size::new(10, 15);
        let aabb = AABB::with_size(size);
        let mut texture = Texture::new(size, WHITE);
        let layout = BrickPattern::new("test", Size::square(5), 2, create_component()).unwrap();

        layout.generate(&mut texture, Data::for_texture(aabb));

        #[rustfmt::skip]
        let expected_colors = vec![
            WHITE, WHITE, WHITE, WHITE, WHITE,   WHITE, WHITE, WHITE, WHITE, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,   WHITE,   RED,   RED,   RED, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,   WHITE,   RED,   RED,   RED, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,   WHITE,   RED,   RED,   RED, WHITE,
            WHITE, WHITE, WHITE, WHITE, WHITE,   WHITE, WHITE, WHITE, WHITE, WHITE,

            WHITE, WHITE, WHITE,   WHITE, WHITE, WHITE, WHITE, WHITE,   WHITE, WHITE,
              RED,   RED, WHITE,   WHITE,   RED,   RED,   RED, WHITE,   WHITE,   RED,
              RED,   RED, WHITE,   WHITE,   RED,   RED,   RED, WHITE,   WHITE,   RED,
              RED,   RED, WHITE,   WHITE,   RED,   RED,   RED, WHITE,   WHITE,   RED,
            WHITE, WHITE, WHITE,   WHITE, WHITE, WHITE, WHITE, WHITE,   WHITE, WHITE,

            WHITE, WHITE, WHITE, WHITE, WHITE,   WHITE, WHITE, WHITE, WHITE, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,   WHITE,   RED,   RED,   RED, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,   WHITE,   RED,   RED,   RED, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,   WHITE,   RED,   RED,   RED, WHITE,
            WHITE, WHITE, WHITE, WHITE, WHITE,   WHITE, WHITE, WHITE, WHITE, WHITE,
        ];

        assert_eq!(texture.get_color_data(), &expected_colors);
    }

    #[test]
    fn test_square_pattern() {
        let size = Size::new(10, 15);
        let aabb = AABB::with_size(size);
        let mut texture = Texture::new(size, WHITE);
        let layout = BrickPattern::new_square("test", 5, create_component()).unwrap();

        layout.generate(&mut texture, Data::for_texture(aabb));

        #[rustfmt::skip]
        let expected_colors = vec![
            WHITE, WHITE, WHITE, WHITE, WHITE,   WHITE, WHITE, WHITE, WHITE, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,   WHITE,   RED,   RED,   RED, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,   WHITE,   RED,   RED,   RED, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,   WHITE,   RED,   RED,   RED, WHITE,
            WHITE, WHITE, WHITE, WHITE, WHITE,   WHITE, WHITE, WHITE, WHITE, WHITE,

            WHITE, WHITE, WHITE, WHITE, WHITE,   WHITE, WHITE, WHITE, WHITE, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,   WHITE,   RED,   RED,   RED, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,   WHITE,   RED,   RED,   RED, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,   WHITE,   RED,   RED,   RED, WHITE,
            WHITE, WHITE, WHITE, WHITE, WHITE,   WHITE, WHITE, WHITE, WHITE, WHITE,

            WHITE, WHITE, WHITE, WHITE, WHITE,   WHITE, WHITE, WHITE, WHITE, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,   WHITE,   RED,   RED,   RED, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,   WHITE,   RED,   RED,   RED, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,   WHITE,   RED,   RED,   RED, WHITE,
            WHITE, WHITE, WHITE, WHITE, WHITE,   WHITE, WHITE, WHITE, WHITE, WHITE,
        ];

        assert_eq!(texture.get_color_data(), &expected_colors);
    }
}
