use crate::generation::data::Data;
use crate::math::aabb::AABB;
use crate::math::color::Color;
use crate::math::shape::Shape;

pub mod depth;

#[derive(Clone, Debug, Eq, PartialEq)]
/// Renders the texture.
pub enum RenderingComponent {
    /// Renders a [`Shape`].
    Shape {
        name: String,
        shape: Shape,
        color: Color,
    },
}

impl RenderingComponent {
    pub fn new_shape<S: Into<String>>(name: S, shape: Shape, color: Color) -> RenderingComponent {
        RenderingComponent::Shape {
            name: name.into(),
            shape,
            color,
        }
    }

    /// Renders the texture in the area defined by the [`AABB`].
    pub fn render(&self, data: &mut dyn Data, aabb: &AABB) {
        match self {
            RenderingComponent::Shape { shape, color, .. } => {
                let mut point = aabb.start();
                let center = aabb.center();

                while point.y < aabb.end().y {
                    point.x = aabb.start().x;

                    while point.x < aabb.end().x {
                        if shape.is_inside(&center, &point) {
                            data.set(&point, color, 255);
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
    use crate::generation::data::TestData;
    use crate::math::color::{RED, WHITE};
    use crate::math::point::Point;
    use crate::math::size::Size;

    #[test]
    fn test_render_shape() {
        let size = Size::new(4, 6);
        let data_size = Size::new(5, 8);
        let start = Point::new(1, 2);
        let rectangle = Shape::new_rectangle(2, 4).unwrap();
        let aabb = AABB::new(start, size);

        let mut data = TestData::new(data_size, WHITE);
        let renderer = RenderingComponent::new_shape("test", rectangle, RED);

        renderer.render(&mut data, &aabb);

        #[rustfmt::skip]
        let colors = vec![
            WHITE, WHITE, WHITE, WHITE, WHITE,
            WHITE, WHITE, WHITE, WHITE, WHITE,
            WHITE, WHITE, WHITE, WHITE, WHITE,
            WHITE, WHITE,   RED,   RED, WHITE,
            WHITE, WHITE,   RED,   RED, WHITE,
            WHITE, WHITE,   RED,   RED, WHITE,
            WHITE, WHITE,   RED,   RED, WHITE,
            WHITE, WHITE, WHITE, WHITE, WHITE
        ];

        assert_eq!(data.get_colors(), &colors);

        #[rustfmt::skip]
        let depth = vec![
            0, 0,   0,   0, 0,
            0, 0,   0,   0, 0,
            0, 0,   0,   0, 0,
            0, 0, 255, 255, 0,
            0, 0, 255, 255, 0,
            0, 0, 255, 255, 0,
            0, 0, 255, 255, 0,
            0, 0,   0,   0, 0
        ];

        assert_eq!(data.get_depth_data(), &depth);
    }
}
