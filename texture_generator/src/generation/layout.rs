use crate::generation::component::GenerationComponent;
use crate::generation::RuntimeData;
use crate::math::aabb::AABB;
use crate::math::size::Size;

#[derive(Debug, Eq, PartialEq)]
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
        size: u32,
        component: Box<GenerationComponent>,
    },
}

impl LayoutComponent {
    pub fn new_square(size: u32, component: GenerationComponent) -> LayoutComponent {
        LayoutComponent::Square {
            size,
            component: Box::new(component),
        }
    }

    /// Generates the layout in the area defined by the [`AABB`].
    pub fn render(&self, data: &mut dyn RuntimeData, aabb: &AABB) {
        match self {
            LayoutComponent::Square { size, component } => {
                let mut point = aabb.start();
                let square_size = Size::new(*size, *size);
                let end = aabb.end() - square_size;
                let step = *size as i32;

                while point.y <= end.y {
                    point.x = aabb.start().x;

                    while point.x <= end.x {
                        let square_aabb = AABB::new(point, square_size);

                        component.render(data, &square_aabb);

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
    use crate::generation::rendering::RenderComponent;
    use crate::generation::TestData;
    use crate::math::color::{RED, WHITE};
    use crate::math::shape::Shape;
    use crate::math::size::Size;

    #[test]
    fn test_square_layout() {
        let size = Size::new(8, 12);
        let aabb = AABB::with_size(size);

        let mut data = TestData::new(size, WHITE);

        let rectangle = Shape::new_rectangle(2, 2);
        let renderer = RenderComponent::new_shape(rectangle, RED);
        let component = GenerationComponent::Rendering(renderer);
        let layout = LayoutComponent::new_square(4, component);

        layout.render(&mut data, &aabb);

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

        assert_eq!(data.get_colors(), &expected_colors);
    }
}
