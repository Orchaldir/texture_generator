use crate::generation::RuntimeData;
use crate::math::aabb::AABB;
use crate::math::color::Color;
use crate::math::shape::Shape;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RenderComponent {
    /// Renders a [`Shape`] at the center of the [`AABB`].
    Shape { shape: Shape, color: Color },
}

impl RenderComponent {
    pub fn new_shape(shape: Shape, color: Color) -> RenderComponent {
        RenderComponent::Shape { shape, color }
    }

    /// Renders the component in the area defined by the [`AABB`].
    pub fn render(&self, data: &mut dyn RuntimeData, aabb: &AABB) {
        match self {
            RenderComponent::Shape { shape, color } => {
                let mut point = aabb.start();
                let center = aabb.center();

                while point.y < aabb.end().y {
                    point.x = aabb.start().x;

                    while point.x < aabb.end().x {
                        if shape.is_inside(&center, &point) {
                            data.set(&point, color);
                        }

                        point.x += 1;
                    }

                    point.y += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generation::TestData;
    use crate::math::color::{RED, WHITE};
    use crate::math::point::Point;
    use crate::math::size::Size;

    #[test]
    fn test_render_shape() {
        let size = Size::new(4, 6);
        let data_size = Size::new(5, 8);
        let start = Point::new(1, 2);
        let rectangle = Shape::new_rectangle(2, 4);
        let aabb = AABB::new(start, size);

        let mut data = TestData::new(data_size, WHITE);
        let renderer = RenderComponent::new_shape(rectangle, RED);

        renderer.render(&mut data, &aabb);

        #[rustfmt::skip]
        let result = vec![
            WHITE, WHITE, WHITE, WHITE, WHITE,
            WHITE, WHITE, WHITE, WHITE, WHITE,
            WHITE, WHITE, WHITE, WHITE, WHITE,
            WHITE, WHITE,   RED,   RED, WHITE,
            WHITE, WHITE,   RED,   RED, WHITE,
            WHITE, WHITE,   RED,   RED, WHITE,
            WHITE, WHITE,   RED,   RED, WHITE,
            WHITE, WHITE, WHITE, WHITE, WHITE
        ];

        assert_eq!(data.get_colors(), &result);
    }
}
