use crate::generation::component::Component;
use crate::generation::data::texture::Texture;
use crate::generation::data::Data;
use crate::math::aabb::AABB;
use crate::math::point::Point;
use crate::math::size::Size;
use crate::utils::error::ValueError;

#[svgbobdoc::transform]
/// A simple brick wall.
///
/// # Diagram
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
    ) -> Result<BrickPattern, ValueError> {
        if brick.width() < 1 {
            return Err(ValueError::value_too_small(
                name,
                "brick.width",
                brick.width(),
            ));
        } else if brick.height() < 1 {
            return Err(ValueError::value_too_small(
                name,
                "brick.height",
                brick.height(),
            ));
        } else if offset >= brick.width() {
            return Err(ValueError::value_too_big(name, "offset", brick.height()));
        }

        Ok(BrickPattern {
            name: name.into(),
            brick,
            offset,
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
