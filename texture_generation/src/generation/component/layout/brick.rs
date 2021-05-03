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
        let inner = data.get_inner();
        let start = self.calculate_column_row(inner.start());
        let end = self.calculate_column_row(inner.end()) + 1i32;
        let total_bricks = self.calculate_total_bricks(texture);

        for y in start.y..end.y {
            let mut point = self.calculate_brick_start(start, y);
            let mut index = total_bricks.convert_x_y(start.x as u32, y as u32); // + (y as f32 / 2.0).ceil() as usize;

            for _x in start.x..end.x {
                let aabb = AABB::new(point, self.brick);
                let brick_data = data.set(index, aabb);

                self.component.generate(texture, &brick_data);

                index += 1;
                point.x += self.brick.width() as i32;
            }
        }
    }

    fn calculate_brick_start(&self, start: Point, y: i32) -> Point {
        let mut x = start.x * self.brick.width() as i32;

        if is_offset_row(y) {
            x -= self.offset as i32;
        }

        Point::new(x, y * self.brick.height() as i32)
    }

    fn calculate_total_bricks(&self, texture: &mut Texture) -> Size {
        Size::new(
            (texture.get_size().width() as f32 / self.brick.width() as f32).ceil() as u32,
            (texture.get_size().height() as f32 / self.brick.height() as f32).ceil() as u32,
        )
    }

    /// In which row of bricks is this point?
    fn calculate_column_row(&self, point: Point) -> Point {
        let row = point.y / self.brick.height() as i32;
        let mut x = point.x;

        if is_offset_row(row) {
            x += self.offset as i32;
        }

        let column = x / self.brick.width() as i32;
        Point::new(column, row)
    }
}

/// Is the offset applied to this row?
fn is_offset_row(row: i32) -> bool {
    row % 2 == 1
}
