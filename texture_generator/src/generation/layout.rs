use crate::definition::generation::component::ComponentError;
use crate::generation::component::GenerationComponent;
use crate::generation::data::Data;
use crate::math::aabb::AABB;
use crate::math::size::Size;

#[derive(Debug, Eq, PartialEq)]
pub enum LayoutError {
    ComponentError(ComponentError),
    SizeTooSmall(u32),
}

impl From<ComponentError> for LayoutError {
    fn from(error: ComponentError) -> Self {
        LayoutError::ComponentError(error)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
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
        size: u32,
        component: Box<GenerationComponent>,
    },
}

impl LayoutComponent {
    pub fn new_square(
        size: u32,
        component: GenerationComponent,
    ) -> Result<LayoutComponent, LayoutError> {
        LayoutComponent::new_square_box(size, Box::new(component))
    }
    pub fn new_square_box(
        size: u32,
        component: Box<GenerationComponent>,
    ) -> Result<LayoutComponent, LayoutError> {
        if size < 1 {
            return Err(LayoutError::SizeTooSmall(size));
        }

        Ok(LayoutComponent::Square { size, component })
    }

    /// Generates the layout in the area defined by the [`AABB`].
    pub fn generate(&self, data: &mut dyn Data, aabb: &AABB) {
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
    use crate::generation::data::TestData;
    use crate::generation::rendering::RenderComponent;
    use crate::math::color::{RED, WHITE};
    use crate::math::shape::Shape;
    use crate::math::size::Size;

    #[test]
    fn test_square_layout() {
        let size = Size::new(8, 12);
        let aabb = AABB::with_size(size);

        let mut data = TestData::new(size, WHITE);

        let rectangle = Shape::new_rectangle(2, 2).unwrap();
        let renderer = RenderComponent::new_shape(rectangle, RED);
        let component = GenerationComponent::Rendering(renderer);
        let layout = LayoutComponent::new_square(4, component).unwrap();

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

        assert_eq!(data.get_colors(), &expected_colors);
    }
}
