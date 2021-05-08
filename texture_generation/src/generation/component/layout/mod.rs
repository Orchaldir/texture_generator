use crate::generation::component::layout::brick::BrickPattern;
use crate::generation::component::layout::herringbone::HerringbonePattern;
use crate::generation::component::layout::random_ashlar::RandomAshlarPattern;
use crate::generation::component::layout::split::SplitLayout;
use crate::generation::component::Component;
use crate::generation::data::texture::Texture;
use crate::generation::data::Data;
use crate::math::aabb::AABB;
use crate::math::size::Size;
use crate::utils::error::ValueError;

pub mod brick;
pub mod herringbone;
pub mod random_ashlar;
pub mod split;

#[svgbobdoc::transform]
#[derive(Clone, Debug, PartialEq)]
/// Generates a layout,
pub enum LayoutComponent {
    BrickWall(BrickPattern),
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
    Split(SplitLayout),
}

impl LayoutComponent {
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

    /// Flips between horizontal & vertical mode.
    pub fn flip(&self) -> LayoutComponent {
        match self {
            LayoutComponent::BrickWall(..) => self.clone(),
            LayoutComponent::Herringbone(..) => self.clone(),
            LayoutComponent::Mock(_id) => self.clone(),
            LayoutComponent::RandomAshlar(..) => self.clone(),
            LayoutComponent::RepeatX { size, component } => LayoutComponent::RepeatY {
                size: *size,
                component: component.flip(),
            },
            LayoutComponent::RepeatY { size, component } => LayoutComponent::RepeatX {
                size: *size,
                component: component.flip(),
            },
            LayoutComponent::Split(split) => LayoutComponent::Split(split.flip()),
        }
    }

    /// Generates the layout in the area defined by the [`AABB`].
    pub fn generate(&self, texture: &mut Texture, data: &Data) {
        let inner = data.get_inner();
        let mut combined = data.combine();

        match self {
            LayoutComponent::BrickWall(pattern) => pattern.generate(texture, combined),
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
            LayoutComponent::Split(split) => split.generate(texture, combined),
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

    pub fn create_component() -> Component {
        let renderer = RenderingComponent::new_shape("tile", Rectangle, RED, 200);
        let rendering_component = Component::Rendering(Box::new(renderer));
        let border = BorderComponent::new_uniform(1, rendering_component);
        Component::Border(Box::new(border))
    }
}
