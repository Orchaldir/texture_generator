use crate::generation::component::layout::herringbone::HerringbonePattern;
use crate::generation::component::layout::random_ashlar::RandomAshlarPattern;
use crate::generation::component::Component;
use crate::generation::data::texture::Texture;
use crate::generation::data::Data;
use crate::math::aabb::AABB;
use crate::math::size::Size;
use crate::utils::error::ValueError;

pub mod herringbone;
pub mod random_ashlar;

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
    Herringbone(HerringbonePattern),
    Mock(u32),
    RandomAshlar(RandomAshlarPattern),
    /// Repeats a component along the x-axis.
    ///
    /// # Diagram
    ///
    /// ```svgbob
    ///   +--*--*--*
    ///   |  |  |  |
    ///   *  *  *  *
    ///   |  |  |  |
    ///   *  *  *  *
    ///   |  |  |  |
    ///   *--*--*--*
    /// ```
    RepeatX {
        size: u32,
        component: Component,
    },
    /// Repeats a component along the y-axis.
    ///
    /// # Diagram
    ///
    /// ```svgbob
    ///   +--*--*--*
    ///   |        |
    ///   *--*--*--*
    ///   |        |
    ///   *--*--*--*
    ///   |        |
    ///   *--*--*--*
    /// ```
    RepeatY {
        size: u32,
        component: Component,
    },
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
            LayoutComponent::Herringbone(..) => self.clone(),
            LayoutComponent::Mock(_id) => self.clone(),
            LayoutComponent::RandomAshlar(..) => self.clone(),
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
    pub fn generate(&self, texture: &mut Texture, data: &Data) {
        let inner = data.get_inner();
        let mut combined = data.combine();

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

                        component.generate(texture, &combined.next(aabb));

                        point.x += brick.width() as i32;
                    }

                    point.y += brick.height() as i32;
                    is_offset_row = !is_offset_row;
                }
            }
            LayoutComponent::Herringbone(pattern) => pattern.generate(texture, &combined),
            LayoutComponent::Mock(id) => info!("Generate layout mock {}", *id),
            LayoutComponent::RandomAshlar(pattern) => pattern.generate(texture, combined),
            LayoutComponent::RepeatX { size, component } => {
                let mut point = inner.start();
                let mut i = 0;
                let end = inner.end().x;
                let (step, remain) = calculate_repeat_step(inner.size().width(), *size);
                let bigger_step = step + 1;
                let aabb_size = Size::new(step as u32, inner.size().height());
                let bigger_size = Size::new(bigger_step as u32, inner.size().height());

                while point.x < end {
                    let (size, step) = if i < remain {
                        (bigger_size, bigger_step)
                    } else {
                        (aabb_size, step)
                    };
                    let aabb = AABB::new(point, size);

                    component.generate(texture, &combined.next(aabb));

                    point.x += step;
                    i += 1;
                }
            }
            LayoutComponent::RepeatY { size, component } => {
                let mut point = inner.start();
                let mut i = 0;
                let end = inner.end().y;
                let (step, remain) = calculate_repeat_step(inner.size().height(), *size);
                let bigger_step = step + 1;
                let aabb_size = Size::new(inner.size().width(), step as u32);
                let bigger_size = Size::new(inner.size().width(), bigger_step as u32);

                while point.y < end {
                    let (size, step) = if i < remain {
                        (bigger_size, bigger_step)
                    } else {
                        (aabb_size, step)
                    };
                    let aabb = AABB::new(point, size);

                    component.generate(texture, &combined.next(aabb));

                    point.y += step;
                    i += 1;
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

                        component.generate(texture, &combined.next(aabb));

                        point.x += step;
                    }

                    point.y += step;
                }
            }
        }
    }
}

fn calculate_repeat_step(size: u32, side: u32) -> (i32, u32) {
    let factor = (size % side) as f32 / side as f32;
    let n = size / side;

    let n_step = if factor < 0.5 { n } else { n + 1 };
    let step = (size / n_step) as i32;
    let remain = size - n_step * step as u32;
    (step, remain)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generation::component::border::BorderComponent;
    use crate::generation::component::rendering::RenderingComponent;
    use crate::generation::data::texture::Texture;
    use crate::math::color::{RED, WHITE};
    use crate::math::shape_factory::ShapeFactory::Rectangle;
    use crate::math::size::Size;

    #[test]
    fn test_brick_wall() {
        let size = Size::new(10, 15);
        let aabb = AABB::with_size(size);
        let mut texture = Texture::new(size, WHITE);
        let layout =
            LayoutComponent::new_brick_wall("test", Size::square(5), 2, create_component())
                .unwrap();

        layout.generate(&mut texture, &Data::for_texture(aabb));

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

        assert_eq!(texture.get_color_data(), &expected_colors);
    }

    #[test]
    fn test_repeat_x() {
        let size = Size::new(15, 5);
        let aabb = AABB::with_size(size);
        let mut texture = Texture::new(size, WHITE);
        let layout = LayoutComponent::new_repeat_x(5, create_component()).unwrap();

        layout.generate(&mut texture, &Data::for_texture(aabb));

        #[rustfmt::skip]
            let expected_colors = vec![
            WHITE, WHITE, WHITE, WHITE, WHITE,  WHITE, WHITE, WHITE, WHITE, WHITE,  WHITE, WHITE, WHITE, WHITE, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,  WHITE,   RED,   RED,   RED, WHITE,  WHITE,   RED,   RED,   RED, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,  WHITE,   RED,   RED,   RED, WHITE,  WHITE,   RED,   RED,   RED, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,  WHITE,   RED,   RED,   RED, WHITE,  WHITE,   RED,   RED,   RED, WHITE,
            WHITE, WHITE, WHITE, WHITE, WHITE,  WHITE, WHITE, WHITE, WHITE, WHITE,  WHITE, WHITE, WHITE, WHITE, WHITE,
        ];

        assert_eq!(texture.get_color_data(), &expected_colors);
    }

    #[test]
    fn test_repeat_y() {
        let size = Size::new(5, 15);
        let aabb = AABB::with_size(size);
        let mut textzre = Texture::new(size, WHITE);
        let layout = LayoutComponent::new_repeat_x(5, create_component())
            .unwrap()
            .flip();

        layout.generate(&mut textzre, &Data::for_texture(aabb));

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

        assert_eq!(textzre.get_color_data(), &expected_colors);
    }

    #[test]
    fn test_square_layout() {
        let size = Size::new(10, 15);
        let aabb = AABB::with_size(size);
        let mut texture = Texture::new(size, WHITE);
        let layout = LayoutComponent::new_square("test", 5, create_component()).unwrap();

        layout.generate(&mut texture, &Data::for_texture(aabb));

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

    fn create_component() -> Component {
        let renderer = RenderingComponent::new_shape("tile", Rectangle, RED, 200);
        let rendering_component = Component::Rendering(Box::new(renderer));
        let border = BorderComponent::new_uniform(1, rendering_component);
        Component::Border(Box::new(border))
    }
}
