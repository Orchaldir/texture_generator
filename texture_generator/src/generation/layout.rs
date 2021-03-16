use crate::generation::component::Component;
use crate::generation::data::Data;
use crate::math::aabb::AABB;
use crate::math::size::Size;
use crate::utils::error::GenerationError;

#[derive(Clone, Debug, PartialEq)]
/// Generates a layout,
pub enum LayoutComponent {
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
        size: u32,
        component: Component,
    },
}

impl LayoutComponent {
    pub fn new_square<S: Into<String>>(
        name: S,
        size: u32,
        component: Component,
    ) -> Result<LayoutComponent, GenerationError> {
        if size < 1 {
            return Err(GenerationError::value_too_small(name, "size", size));
        }

        Ok(LayoutComponent::Square {
            name: name.into(),
            size,
            component,
        })
    }

    /// Generates the layout in the area defined by the [`AABB`].
    pub fn generate(&self, data: &mut dyn Data, aabb: &AABB) {
        match self {
            LayoutComponent::Square {
                size, component, ..
            } => {
                let mut point = aabb.start();
                let square_size = Size::new(*size, *size);
                let end = aabb.end() - square_size;
                let step = *size as i32;

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
    use crate::generation::data::RuntimeData;
    use crate::generation::rendering::RenderingComponent;
    use crate::math::color::{RED, WHITE};
    use crate::math::shape::Shape;
    use crate::math::size::Size;

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
