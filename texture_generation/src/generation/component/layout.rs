use crate::generation::component::Component;
use crate::generation::data::Data;
use crate::math::aabb::AABB;
use crate::math::size::Size;
use crate::utils::error::ValueError;

#[svgbobdoc::transform]
#[derive(Clone, Debug, PartialEq)]
/// Generates a layout,
pub enum LayoutComponent {
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
    BrickWall {
        name: String,
        brick: Size,
        offset: i32,
        component: Component,
    },
    /// Repeats a component along the x-axis.
    RepeatX { size: u32, component: Component },
    /// Repeats a component along the y-axis.
    RepeatY { size: u32, component: Component },
    /// A grid of squares that have the same size.
    ///
    /// # Diagram
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
    ) -> Result<LayoutComponent, ValueError> {
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

        let offset = offset as i32 - brick.width() as i32;

        Ok(LayoutComponent::BrickWall {
            name: name.into(),
            brick,
            offset,
            component,
        })
    }

    pub fn new_repeat_x(size: u32, component: Component) -> Result<LayoutComponent, ValueError> {
        if size < 1 {
            return Err(ValueError::value_too_small("repeat_x", "size", size));
        }

        Ok(LayoutComponent::RepeatX { size, component })
    }

    pub fn new_repeat_y(size: u32, component: Component) -> Result<LayoutComponent, ValueError> {
        if size < 1 {
            return Err(ValueError::value_too_small("repeat_y", "size", size));
        }

        Ok(LayoutComponent::RepeatY { size, component })
    }

    pub fn new_square<S: Into<String>>(
        name: S,
        side: u32,
        component: Component,
    ) -> Result<LayoutComponent, ValueError> {
        if side < 1 {
            return Err(ValueError::value_too_small(name, "side", side));
        }

        Ok(LayoutComponent::Square {
            name: name.into(),
            side,
            component,
        })
    }

    /// Flips between horizontal & vertical mode.
    pub fn flip(&self) -> LayoutComponent {
        match self.clone() {
            LayoutComponent::BrickWall {
                name,
                brick,
                offset,
                component,
            } => LayoutComponent::BrickWall {
                name,
                brick,
                offset,
                component: component.flip(),
            },
            LayoutComponent::RepeatX { size, component } => LayoutComponent::RepeatY {
                size,
                component: component.flip(),
            },
            LayoutComponent::RepeatY { size, component } => LayoutComponent::RepeatX {
                size,
                component: component.flip(),
            },
            LayoutComponent::Square {
                name,
                side,
                component,
            } => LayoutComponent::Square {
                name,
                side,
                component: component.flip(),
            },
        }
    }

    /// Generates the layout in the area defined by the [`AABB`].
    pub fn generate(&self, data: &mut dyn Data, outer: &AABB, inner: &AABB) {
        let limited = outer.limit(inner);

        match self {
            LayoutComponent::BrickWall {
                brick,
                offset,
                component,
                ..
            } => {
                let mut point = inner.start();
                let mut is_offset_row = false;

                while point.y < inner.end().y {
                    point.x = inner.start().x + if is_offset_row { *offset } else { 0 };

                    while point.x < inner.end().x {
                        let aabb = AABB::new(point, *brick);

                        component.generate(data, &limited, &aabb);

                        point.x += brick.width() as i32;
                    }

                    point.y += brick.height() as i32;
                    is_offset_row = !is_offset_row;
                }
            }
            LayoutComponent::RepeatX { size, component } => {
                let mut point = inner.start();
                let inner_size = Size::new(*size, inner.size().height());
                let end = inner.end().x;
                let step = *size as i32;

                while point.x < end {
                    let aabb = AABB::new(point, inner_size);

                    component.generate(data, &limited, &aabb);

                    point.x += step;
                }
            }
            LayoutComponent::RepeatY { size, component } => {
                let mut point = inner.start();
                let inner_size = Size::new(inner.size().width(), *size);
                let end = inner.end().y;
                let step = *size as i32;

                while point.y < end {
                    let aabb = AABB::new(point, inner_size);

                    component.generate(data, &limited, &aabb);

                    point.y += step;
                }
            }
            LayoutComponent::Square {
                side, component, ..
            } => {
                let mut point = inner.start();
                let square_size = Size::square(*side);
                let end = inner.end() - square_size;
                let step = *side as i32;

                while point.y <= end.y {
                    point.x = inner.start().x;

                    while point.x <= end.x {
                        let aabb = AABB::new(point, square_size);

                        component.generate(data, &limited, &aabb);

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
    use crate::generation::component::border::BorderComponent;
    use crate::generation::component::rendering::RenderingComponent;
    use crate::generation::data::RuntimeData;
    use crate::math::color::{RED, WHITE};
    use crate::math::shape_factory::ShapeFactory::Rectangle;
    use crate::math::size::Size;

    #[test]
    fn test_brick_wall() {
        let size = Size::new(10, 15);
        let aabb = AABB::with_size(size);
        let mut data = RuntimeData::new(size, WHITE);
        let layout =
            LayoutComponent::new_brick_wall("test", Size::square(5), 2, create_component())
                .unwrap();

        layout.generate(&mut data, &aabb, &aabb);

        #[rustfmt::skip]
        let expected_colors = vec![
            WHITE, WHITE, WHITE, WHITE, WHITE,   WHITE, WHITE, WHITE, WHITE, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,   WHITE,   RED,   RED,   RED, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,   WHITE,   RED,   RED,   RED, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,   WHITE,   RED,   RED,   RED, WHITE,
            WHITE, WHITE, WHITE, WHITE, WHITE,   WHITE, WHITE, WHITE, WHITE, WHITE,

            WHITE, WHITE,   WHITE, WHITE, WHITE, WHITE, WHITE,   WHITE, WHITE, WHITE,
              RED, WHITE,   WHITE,   RED,   RED,   RED, WHITE,   WHITE,   RED,   RED,
              RED, WHITE,   WHITE,   RED,   RED,   RED, WHITE,   WHITE,   RED,   RED,
              RED, WHITE,   WHITE,   RED,   RED,   RED, WHITE,   WHITE,   RED,   RED,
            WHITE, WHITE,   WHITE, WHITE, WHITE, WHITE, WHITE,   WHITE, WHITE, WHITE,

            WHITE, WHITE, WHITE, WHITE, WHITE,   WHITE, WHITE, WHITE, WHITE, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,   WHITE,   RED,   RED,   RED, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,   WHITE,   RED,   RED,   RED, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,   WHITE,   RED,   RED,   RED, WHITE,
            WHITE, WHITE, WHITE, WHITE, WHITE,   WHITE, WHITE, WHITE, WHITE, WHITE,
        ];

        assert_eq!(data.get_color_data(), &expected_colors);
    }

    #[test]
    fn test_repeat_x() {
        let size = Size::new(15, 5);
        let aabb = AABB::with_size(size);
        let mut data = RuntimeData::new(size, WHITE);
        let layout = LayoutComponent::new_repeat_x(5, create_component()).unwrap();

        layout.generate(&mut data, &aabb, &aabb);

        #[rustfmt::skip]
            let expected_colors = vec![
            WHITE, WHITE, WHITE, WHITE, WHITE,  WHITE, WHITE, WHITE, WHITE, WHITE,  WHITE, WHITE, WHITE, WHITE, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,  WHITE,   RED,   RED,   RED, WHITE,  WHITE,   RED,   RED,   RED, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,  WHITE,   RED,   RED,   RED, WHITE,  WHITE,   RED,   RED,   RED, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,  WHITE,   RED,   RED,   RED, WHITE,  WHITE,   RED,   RED,   RED, WHITE,
            WHITE, WHITE, WHITE, WHITE, WHITE,  WHITE, WHITE, WHITE, WHITE, WHITE,  WHITE, WHITE, WHITE, WHITE, WHITE,
        ];

        assert_eq!(data.get_color_data(), &expected_colors);
    }

    #[test]
    fn test_repeat_y() {
        let size = Size::new(5, 15);
        let aabb = AABB::with_size(size);
        let mut data = RuntimeData::new(size, WHITE);
        let layout = LayoutComponent::new_repeat_x(5, create_component())
            .unwrap()
            .flip();

        layout.generate(&mut data, &aabb, &aabb);

        #[rustfmt::skip]
            let expected_colors = vec![
            WHITE, WHITE, WHITE, WHITE, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,
            WHITE, WHITE, WHITE, WHITE, WHITE,

            WHITE, WHITE, WHITE, WHITE, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,
            WHITE, WHITE, WHITE, WHITE, WHITE,

            WHITE, WHITE, WHITE, WHITE, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,
            WHITE, WHITE, WHITE, WHITE, WHITE,
        ];

        assert_eq!(data.get_color_data(), &expected_colors);
    }

    #[test]
    fn test_square_layout() {
        let size = Size::new(10, 15);
        let aabb = AABB::with_size(size);
        let mut data = RuntimeData::new(size, WHITE);
        let layout = LayoutComponent::new_square("test", 5, create_component()).unwrap();

        layout.generate(&mut data, &aabb, &aabb);

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

        assert_eq!(data.get_color_data(), &expected_colors);
    }

    fn create_component() -> Component {
        let renderer = RenderingComponent::new_shape("tile", Rectangle, RED, 200);
        let rendering_component = Component::Rendering(Box::new(renderer));
        let border = BorderComponent::new_uniform(1, rendering_component);
        Component::Border(Box::new(border))
    }
}
