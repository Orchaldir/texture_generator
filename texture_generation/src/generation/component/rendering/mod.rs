use crate::generation::component::rendering::color::ColorSelector;
use crate::generation::component::rendering::depth::DepthCalculator;
use crate::generation::data::Data;
use crate::math::aabb::AABB;
use crate::math::color::Color;
use crate::math::point::Point;
use crate::math::shape::Shape;

pub mod color;
pub mod depth;

#[derive(Clone, Debug, PartialEq)]
/// Renders the texture.
pub enum RenderingComponent {
    /// Fills the area with a color.
    FillArea {
        name: String,
        color: Color,
        depth: u8,
    },
    /// Renders a [`Shape`].
    Shape {
        name: String,
        shape: Shape,
        color_selector: ColorSelector,
        depth_calculator: DepthCalculator,
    },
}

impl RenderingComponent {
    pub fn new_fill_area<S: Into<String>>(name: S, color: Color, depth: u8) -> RenderingComponent {
        RenderingComponent::FillArea {
            name: name.into(),
            color,
            depth,
        }
    }

    pub fn new_shape<S: Into<String>>(name: S, shape: Shape, color: Color) -> RenderingComponent {
        RenderingComponent::new_shape_with_depth(
            name,
            shape,
            ColorSelector::ConstantColor(color),
            DepthCalculator::Uniform(200),
        )
    }

    pub fn new_shape_with_depth<S: Into<String>>(
        name: S,
        shape: Shape,
        color_selector: ColorSelector,
        depth_calculator: DepthCalculator,
    ) -> RenderingComponent {
        RenderingComponent::Shape {
            name: name.into(),
            shape,
            color_selector,
            depth_calculator,
        }
    }

    /// Renders the texture in the area defined by the [`AABB`].
    pub fn render(&self, data: &mut dyn Data, aabb: &AABB) {
        match self {
            RenderingComponent::FillArea { color, depth, .. } => {
                let start = aabb.start().max(&Point::default());
                let end = aabb.end().limit_to(data.get_size());
                let mut point = start;
                let depth = data.get_base_depth() + *depth;

                while point.y < end.y {
                    point.x = start.x;

                    while point.x < end.x {
                        data.set(&point, color, depth);

                        point.x += 1;
                    }

                    point.y += 1;
                }
            }
            RenderingComponent::Shape {
                shape,
                color_selector,
                depth_calculator,
                ..
            } => {
                let start = aabb.start().max(&Point::default());
                let end = aabb.end().limit_to(data.get_size());
                let mut point = start;
                let center = aabb.center();
                let color = color_selector.select();
                let base_depth = data.get_base_depth();

                while point.y < end.y {
                    point.x = start.x;

                    while point.x < end.x {
                        let distance = shape.distance(&center, &point);

                        if distance <= 1.0 {
                            let depth =
                                base_depth.saturating_add(depth_calculator.calculate(distance));
                            data.set(&point, &color, depth);
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
    use crate::generation::data::RuntimeData;
    use crate::math::color::{RED, WHITE};
    use crate::math::point::Point;
    use crate::math::size::Size;

    #[test]
    fn test_render_fill_area() {
        let size = Size::new(3, 4);
        let data_size = Size::new(5, 7);
        let start = Point::new(1, 2);
        let aabb = AABB::new(start, size);

        let mut data = RuntimeData::with_base_depth(data_size, WHITE, 3);
        let renderer = RenderingComponent::new_fill_area("test", RED, 42);

        renderer.render(&mut data, &aabb);

        #[rustfmt::skip]
        let colors = vec![
            WHITE, WHITE, WHITE, WHITE, WHITE,
            WHITE, WHITE, WHITE, WHITE, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,
            WHITE,   RED,   RED,   RED, WHITE,
            WHITE, WHITE, WHITE, WHITE, WHITE,
        ];

        assert_eq!(data.get_color_data(), &colors);

        #[rustfmt::skip]
        let depth = vec![
            0,  0,  0,  0, 0,
            0,  0,  0,  0, 0,
            0, 45, 45, 45, 0,
            0, 45, 45, 45, 0,
            0, 45, 45, 45, 0,
            0, 45, 45, 45, 0,
            0,  0,  0,  0, 0,
        ];

        assert_eq!(data.get_depth_data(), &depth);
    }

    #[test]
    fn test_render_shape() {
        let size = Size::new(4, 6);
        let data_size = Size::new(6, 9);
        let start = Point::new(1, 2);
        let rectangle = Shape::new_rectangle(2, 4).unwrap();
        let aabb = AABB::new(start, size);

        let mut data = RuntimeData::with_base_depth(data_size, WHITE, 10);
        let renderer = RenderingComponent::new_shape("test", rectangle, RED);

        renderer.render(&mut data, &aabb);

        #[rustfmt::skip]
        let colors = vec![
            WHITE, WHITE, WHITE, WHITE, WHITE, WHITE,
            WHITE, WHITE, WHITE, WHITE, WHITE, WHITE,
            WHITE, WHITE, WHITE, WHITE, WHITE, WHITE,
            WHITE, WHITE,   RED,   RED,   RED, WHITE,
            WHITE, WHITE,   RED,   RED,   RED, WHITE,
            WHITE, WHITE,   RED,   RED,   RED, WHITE,
            WHITE, WHITE,   RED,   RED,   RED, WHITE,
            WHITE, WHITE,   RED,   RED,   RED, WHITE,
            WHITE, WHITE, WHITE, WHITE, WHITE, WHITE,
        ];

        assert_eq!(data.get_color_data(), &colors);

        #[rustfmt::skip]
        let depth = vec![
            0, 0,   0,   0,   0, 0,
            0, 0,   0,   0,   0, 0,
            0, 0,   0,   0,   0, 0,
            0, 0, 210, 210, 210, 0,
            0, 0, 210, 210, 210, 0,
            0, 0, 210, 210, 210, 0,
            0, 0, 210, 210, 210, 0,
            0, 0, 210, 210, 210, 0,
            0, 0,   0,   0,   0, 0,
        ];

        assert_eq!(data.get_depth_data(), &depth);
    }

    #[test]
    fn test_render_shape_partly_outside() {
        let size = Size::new(4, 2);
        let data_size = Size::square(4);
        let start = Point::new(-2, -1);
        let rectangle = Shape::new_rectangle(4, 2).unwrap();
        let aabb = AABB::new(start, size);

        let mut data = RuntimeData::new(data_size, WHITE);
        let renderer = RenderingComponent::new_shape("test", rectangle, RED);

        renderer.render(&mut data, &aabb);

        #[rustfmt::skip]
            let colors = vec![
              RED,   RED, WHITE, WHITE,
            WHITE, WHITE, WHITE, WHITE,
            WHITE, WHITE, WHITE, WHITE,
            WHITE, WHITE, WHITE, WHITE,
        ];

        assert_eq!(data.get_color_data(), &colors);

        #[rustfmt::skip]
            let depth = vec![
            200, 200, 0, 0,
              0,   0, 0, 0,
              0,   0, 0, 0,
              0,   0, 0, 0,
        ];

        assert_eq!(data.get_depth_data(), &depth);
    }
}
