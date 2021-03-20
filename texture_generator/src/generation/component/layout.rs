use crate::generation::component::Component;
use crate::generation::data::Data;
use crate::math::aabb::AABB;
use crate::math::size::Size;
use crate::utils::error::GenerationError;

#[derive(Clone, Debug, PartialEq)]
/// Generates a layout,
pub enum LayoutComponent {
    /// A simple brick wall.
    ///
    /// # Diagram
    ///
    /// ```svgbob
    ///   +-----*-----*-----*-----*----> x-axis
    ///   |     |     |     |     |
    ///   *--*--*--*--*--*--*--*--*--*
    ///   |  |     |     |     |     |
    ///   *--*--*--*--*--*--*--*--*--*
    ///   |     |     |     |     |
    ///   *--*--*--*--*--*--*--*--*
    ///   |
    ///   v
    /// y-axis
    /// ```
    BrickWall {
        name: String,
        brick: Size,
        offset: i32,
        component: Component,
    },
    /// A grid of squares that have the same size.
    ///
    /// # Diagram
    ///
    /// ```svgbob
    ///   +--*--*--*----> x-axis
    ///   |  |  |  |
    ///   *--*--*--*
    ///   |  |  |  |
    ///   *--*--*--*
    ///   |  |  |  |
    ///   *--*--*--*
    ///   |
    ///   v
    /// y-axis
    /// ```
    Square {
        name: String,
        side: u32,
        component: Component,
    },
}

impl LayoutComponent {
    pub fn new_brick_wall<S: Into<String>>(
        name: S,
        brick: Size,
        offset: u32,
        component: Component,
    ) -> Result<LayoutComponent, GenerationError> {
        if brick.width() < 1 {
            return Err(GenerationError::value_too_small(
                name,
                "brick.width",
                brick.width(),
            ));
        } else if brick.height() < 1 {
            return Err(GenerationError::value_too_small(
                name,
                "brick.height",
                brick.height(),
            ));
        } else if offset >= brick.width() {
            return Err(GenerationError::value_too_big(
                name,
                "offset",
                brick.height(),
            ));
        }

        let offset = offset as i32 - brick.width() as i32;

        Ok(LayoutComponent::BrickWall {
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
    ) -> Result<LayoutComponent, GenerationError> {
        if side < 1 {
            return Err(GenerationError::value_too_small(name, "side", side));
        }

        Ok(LayoutComponent::Square {
            name: name.into(),
            side,
            component,
        })
    }

    /// Generates the layout in the area defined by the [`AABB`].
    pub fn generate(&self, data: &mut dyn Data, aabb: &AABB) {
        match self {
            LayoutComponent::BrickWall {
                brick,
                offset,
                component,
                ..
            } => {
                let mut point = aabb.start();
                let mut is_offset_row = false;

                while point.y < aabb.end().y {
                    point.x = aabb.start().x + if is_offset_row { *offset } else { 0 };

                    while point.x < aabb.end().x {
                        let square_aabb = AABB::new(point, *brick);

                        component.generate(data, &square_aabb);

                        point.x += brick.width() as i32;
                    }

                    point.y += brick.height() as i32;
                    is_offset_row = !is_offset_row;
                }
            }
            LayoutComponent::Square {
                side, component, ..
            } => {
                let mut point = aabb.start();
                let square_size = Size::new(*side, *side);
                let end = aabb.end() - square_size;
                let step = *side as i32;

                while point.y <= end.y {
                    point.x = aabb.start().x;

                    while point.x <= end.x {
                        let square_aabb = AABB::new(point, square_size);

                        component.generate(data, &square_aabb);

                        point.x += step;
                    }

                    point.y += step;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generation::component::rendering::RenderingComponent;
    use crate::generation::data::RuntimeData;
    use crate::math::color::{RED, WHITE};
    use crate::math::shape::Shape;
    use crate::math::size::Size;

    #[test]
    fn test_brick_wall() {
        let size = Size::new(8, 12);
        let aabb = AABB::with_size(size);

        let mut data = RuntimeData::new(size, WHITE);

        let rectangle = Shape::new_rectangle(2, 2).unwrap();
        let renderer = RenderingComponent::new_shape("tile", rectangle, RED);
        let component = Component::Rendering(Box::new(renderer));
        let layout =
            LayoutComponent::new_brick_wall("test", Size::new(4, 4), 2, component).unwrap();

        layout.generate(&mut data, &aabb);

        #[rustfmt::skip]
            let expected_colors = vec![
            WHITE, WHITE, WHITE, WHITE,   WHITE, WHITE, WHITE, WHITE,
            WHITE,   RED,   RED, WHITE,   WHITE,   RED,   RED, WHITE,
            WHITE,   RED,   RED, WHITE,   WHITE,   RED,   RED, WHITE,
            WHITE, WHITE, WHITE, WHITE,   WHITE, WHITE, WHITE, WHITE,

            WHITE, WHITE,   WHITE, WHITE, WHITE, WHITE,   WHITE, WHITE,
              RED, WHITE,   WHITE,   RED,   RED, WHITE,   WHITE,   RED,
              RED, WHITE,   WHITE,   RED,   RED, WHITE,   WHITE,   RED,
            WHITE, WHITE,   WHITE, WHITE, WHITE, WHITE,   WHITE, WHITE,

            WHITE, WHITE, WHITE, WHITE,   WHITE, WHITE, WHITE, WHITE,
            WHITE,   RED,   RED, WHITE,   WHITE,   RED,   RED, WHITE,
            WHITE,   RED,   RED, WHITE,   WHITE,   RED,   RED, WHITE,
            WHITE, WHITE, WHITE, WHITE,   WHITE, WHITE, WHITE, WHITE,
        ];

        assert_eq!(data.get_color_data(), &expected_colors);
    }

    #[test]
    fn test_square_layout() {
        let size = Size::new(8, 12);
        let aabb = AABB::with_size(size);

        let mut data = RuntimeData::new(size, WHITE);

        let rectangle = Shape::new_rectangle(2, 2).unwrap();
        let renderer = RenderingComponent::new_shape("tile", rectangle, RED);
        let component = Component::Rendering(Box::new(renderer));
        let layout = LayoutComponent::new_square("test", 4, component).unwrap();

        layout.generate(&mut data, &aabb);

        #[rustfmt::skip]
        let expected_colors = vec![
            WHITE, WHITE, WHITE, WHITE,   WHITE, WHITE, WHITE, WHITE,
            WHITE,   RED,   RED, WHITE,   WHITE,   RED,   RED, WHITE,
            WHITE,   RED,   RED, WHITE,   WHITE,   RED,   RED, WHITE,
            WHITE, WHITE, WHITE, WHITE,   WHITE, WHITE, WHITE, WHITE,

            WHITE, WHITE, WHITE, WHITE,   WHITE, WHITE, WHITE, WHITE,
            WHITE,   RED,   RED, WHITE,   WHITE,   RED,   RED, WHITE,
            WHITE,   RED,   RED, WHITE,   WHITE,   RED,   RED, WHITE,
            WHITE, WHITE, WHITE, WHITE,   WHITE, WHITE, WHITE, WHITE,

            WHITE, WHITE, WHITE, WHITE,   WHITE, WHITE, WHITE, WHITE,
            WHITE,   RED,   RED, WHITE,   WHITE,   RED,   RED, WHITE,
            WHITE,   RED,   RED, WHITE,   WHITE,   RED,   RED, WHITE,
            WHITE, WHITE, WHITE, WHITE,   WHITE, WHITE, WHITE, WHITE,
        ];

        assert_eq!(data.get_color_data(), &expected_colors);
    }
}
