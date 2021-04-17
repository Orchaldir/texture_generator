use crate::generation::component::Component;
use crate::generation::data::Data;
use crate::math::aabb::AABB;
use crate::math::point::Point;
use crate::math::size::Size;
use std::ops::Sub;

#[svgbobdoc::transform]
#[derive(Clone, Debug, PartialEq)]
/// Generates a border around an inner [`Component`].
pub enum BorderComponent {
    /// For better previews.
    MinBorder(Component),
    /// The border is equally big on all sides.
    ///
    /// # Diagram
    ///
    /// ```svgbob
    ///   +--*--*--*
    ///   |        |
    ///   *  *--*  *
    ///   |  |  |  |
    ///   *  *--*  *
    ///   |        |
    ///   *--*--*--*
    /// ```
    UniformBorder { border: u32, component: Component },
}

impl BorderComponent {
    pub fn new_uniform(border: u32, component: Component) -> BorderComponent {
        if border == 0 {
            return BorderComponent::MinBorder(component);
        }

        BorderComponent::UniformBorder { border, component }
    }

    /// Generates the border in the area defined by the [`AABB`].
    pub fn generate(&self, data: &mut dyn Data, outer: &AABB, inner: &AABB) {
        let size = inner.size();

        match self {
            BorderComponent::MinBorder(component) => {
                let aabb = BorderComponent::calculate_aabb(inner, size, 1, 1);
                component.generate(data, outer, &aabb);
            }
            BorderComponent::UniformBorder { border, component } => {
                let min_side = border * 2;

                if size.width() <= min_side || size.height() <= min_side {
                    info!("{:?} smaller than {}", size, min_side);
                    return;
                }

                let aabb = BorderComponent::calculate_aabb(inner, size, *border, min_side);
                component.generate(data, outer, &aabb);
            }
        }
    }

    fn calculate_aabb(inner: &AABB, size: Size, border: u32, min_side: u32) -> AABB {
        let start = inner.start();
        let border = border as i32;
        let new_start = Point::new(start.x + border, start.y + border);
        let new_size = Size::new(size.width().sub(min_side), size.height().sub(min_side));
        AABB::new(new_start, new_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generation::component::rendering::RenderingComponent;
    use crate::generation::data::RuntimeData;
    use crate::math::color::{RED, WHITE};
    use crate::math::size::Size;

    #[test]
    fn test_uniform() {
        let size = Size::new(5, 5);
        let aabb = AABB::with_size(size);

        let mut data = RuntimeData::new(size, WHITE);

        let renderer = RenderingComponent::new_fill_area("red", RED, 0);
        let component = Component::Rendering(Box::new(renderer));
        let layout = BorderComponent::new_uniform(1, component);

        layout.generate(&mut data, &aabb, &aabb);

        #[rustfmt::skip]
            let expected_colors = vec![
            WHITE, WHITE, WHITE, WHITE, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,
            WHITE, WHITE, WHITE, WHITE, WHITE,
        ];

        assert_eq!(data.get_color_data(), &expected_colors);
    }
}
