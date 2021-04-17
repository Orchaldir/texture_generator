use crate::generation::component::rendering::color::ColorSelector;
use crate::generation::component::rendering::depth::DepthCalculator;
use crate::generation::data::Data;
use crate::math::aabb::AABB;
use crate::math::color::{Color, PINK};
use crate::math::shape_factory::ShapeFactory;

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
        shape_factory: ShapeFactory,
        color_selector: ColorSelector,
        depth_calculator: DepthCalculator,
    },
}

impl RenderingComponent {
    pub fn default() -> RenderingComponent {
        Self::new_fill_area("default", PINK, 0)
    }

    pub fn new_fill_area<S: Into<String>>(name: S, color: Color, depth: u8) -> RenderingComponent {
        RenderingComponent::FillArea {
            name: name.into(),
            color,
            depth,
        }
    }

    pub fn new_shape<S: Into<String>>(
        name: S,
        shape_factory: ShapeFactory,
        color: Color,
        depth: u8,
    ) -> RenderingComponent {
        RenderingComponent::new_shape_with_depth(
            name,
            shape_factory,
            ColorSelector::ConstantColor(color),
            DepthCalculator::Uniform(depth),
        )
    }

    pub fn new_shape_with_depth<S: Into<String>>(
        name: S,
        shape_factory: ShapeFactory,
        color_selector: ColorSelector,
        depth_calculator: DepthCalculator,
    ) -> RenderingComponent {
        RenderingComponent::Shape {
            name: name.into(),
            shape_factory,
            color_selector,
            depth_calculator,
        }
    }

    /// Flips between horizontal & vertical mode.
    pub fn flip(&self) -> RenderingComponent {
        self.clone()
    }

    /// Renders the texture in the area defined by the [`AABB`].
    pub fn render(&self, data: &mut dyn Data, outer: &AABB, inner: &AABB) {
        match self {
            RenderingComponent::FillArea { color, depth, .. } => {
                let start = outer.start().max(&inner.start());
                let end = outer.end().min(&inner.end());
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
                shape_factory,
                color_selector,
                depth_calculator,
                ..
            } => {
                let start = outer.start().max(&inner.start());
                let end = outer.end().min(&inner.end());
                let mut point = start;
                let center = inner.center();
                let color = color_selector.select();
                let base_depth = data.get_base_depth();
                if let Ok(shape) = shape_factory.create_shape(inner) {
                    while point.y < end.y {
                        point.x = start.x;

                        while point.x < end.x {
                            let distance = shape.distance(&center, &point);

                            if distance <= 1.0 {
                                let depth = depth_calculator.calculate(distance);
                                let depth = base_depth.saturating_add(depth);
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generation::data::RuntimeData;
    use crate::math::color::{RED, WHITE};
    use crate::math::point::Point;
    use crate::math::size::Size;
    use ShapeFactory::Rectangle;

    #[test]
    fn test_render_fill_area() {
        let size = Size::new(3, 4);
        let data_size = Size::new(5, 7);
        let start = Point::new(1, 2);
        let outer = AABB::with_size(data_size);
        let aabb = AABB::new(start, size);

        let mut data = RuntimeData::with_base_depth(data_size, WHITE, 3);
        let renderer = RenderingComponent::new_fill_area("test", RED, 42);

        renderer.render(&mut data, &outer, &aabb);

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
        let size = Size::new(3, 4);
        let data_size = Size::new(5, 7);
        let start = Point::new(1, 2);
        let outer = AABB::with_size(data_size);
        let aabb = AABB::new(start, size);

        let mut data = RuntimeData::with_base_depth(data_size, WHITE, 3);
        let renderer = RenderingComponent::new_shape("test", Rectangle, RED, 42);

        renderer.render(&mut data, &outer, &aabb);

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
    fn test_render_shape_partly_outside() {
        let size = Size::new(4, 2);
        let data_size = Size::square(4);
        let start = Point::new(-2, -1);
        let outer = AABB::with_size(data_size);
        let aabb = AABB::new(start, size);

        let mut data = RuntimeData::new(data_size, WHITE);
        let renderer = RenderingComponent::new_shape("test", Rectangle, RED, 200);

        renderer.render(&mut data, &outer, &aabb);

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
