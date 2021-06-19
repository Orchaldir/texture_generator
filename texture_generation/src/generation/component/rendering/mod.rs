use crate::generation::component::rendering::color_factory::ColorFactory;
use crate::generation::component::rendering::depth_factory::DepthFactory;
use crate::generation::data::texture::Texture;
use crate::generation::data::Data;
use crate::math::color::{Color, PINK};
use crate::math::shape_factory::ShapeFactory;

pub mod color;
pub mod color_factory;
pub mod depth;
pub mod depth_factory;

#[derive(Clone, Debug, PartialEq)]
/// Renders the texture.
pub enum RenderingComponent {
    /// Fills the area with a color.
    FillArea {
        color: Color,
        depth: u8,
    },
    Mock,
    /// Renders a [`Shape`].
    Shape {
        shape_factory: ShapeFactory,
        color_selector: ColorFactory,
        depth_factory: DepthFactory,
    },
}

impl RenderingComponent {
    pub fn default() -> RenderingComponent {
        Self::new_fill_area(PINK, 0)
    }

    pub fn new_fill_area(color: Color, depth: u8) -> RenderingComponent {
        RenderingComponent::FillArea { color, depth }
    }

    pub fn new_shape(shape_factory: ShapeFactory, color: Color, depth: u8) -> RenderingComponent {
        RenderingComponent::new_shape_with_depth(
            shape_factory,
            ColorFactory::ConstantColor(color),
            DepthFactory::Uniform(depth),
        )
    }

    pub fn new_shape_with_depth(
        shape_factory: ShapeFactory,
        color_factory: ColorFactory,
        depth_factory: DepthFactory,
    ) -> RenderingComponent {
        RenderingComponent::Shape {
            shape_factory,
            color_selector: color_factory,
            depth_factory,
        }
    }

    /// Renders the texture in the area defined by the [`AABB`].
    pub fn render(&self, texture: &mut Texture, data: &Data) {
        let aabbs = data.get_aabbs_in_texture_space();
        let start = aabbs.get_start();
        let end = aabbs.get_end();

        match self {
            RenderingComponent::FillArea { color, depth, .. } => {
                let mut point = start;
                let depth = texture.get_base_depth() + *depth;

                while point.y < end.y {
                    point.x = start.x;

                    while point.x < end.x {
                        texture.set(&point, color, depth);

                        point.x += 1;
                    }

                    point.y += 1;
                }
            }
            RenderingComponent::Mock => {}
            RenderingComponent::Shape {
                shape_factory,
                color_selector: color_factory,
                depth_factory,
                ..
            } => {
                let mut point = start;
                let color_selector = color_factory.create(data);
                let depth_calculator = depth_factory.create(data);
                let base_depth = texture.get_base_depth();

                if let Ok(shape) = shape_factory.create_shape(aabbs.get_inner()) {
                    while point.y < end.y {
                        point.x = start.x;

                        while point.x < end.x {
                            let distance = shape.distance(&point);

                            if distance <= 1.0 {
                                let color = color_selector.select(&point);
                                let depth = depth_calculator.calculate(&point, distance);
                                let depth = base_depth.saturating_add(depth);
                                texture.set(&point, &color, depth);
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
    use crate::generation::data::texture::Texture;
    use crate::math::aabb::AABB;
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

        let mut texture = Texture::with_depth(data_size, WHITE, 3);
        let renderer = RenderingComponent::new_fill_area(RED, 42);

        renderer.render(&mut texture, &Data::for_two_aabb(0, outer, aabb));

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

        assert_eq!(texture.get_color_data(), &colors);

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

        assert_eq!(texture.get_depth_data(), &depth);
    }

    #[test]
    fn test_render_shape() {
        let size = Size::new(3, 4);
        let data_size = Size::new(5, 7);
        let start = Point::new(1, 2);
        let outer = AABB::with_size(data_size);
        let aabb = AABB::new(start, size);

        let mut texture = Texture::with_depth(data_size, WHITE, 3);
        let renderer = RenderingComponent::new_shape(Rectangle, RED, 42);

        renderer.render(&mut texture, &Data::for_two_aabb(0, outer, aabb));

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

        assert_eq!(texture.get_color_data(), &colors);

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

        assert_eq!(texture.get_depth_data(), &depth);
    }

    #[test]
    fn test_render_shape_partly_outside() {
        let size = Size::new(4, 2);
        let data_size = Size::square(4);
        let start = Point::new(-2, -1);
        let outer = AABB::with_size(data_size);
        let aabb = AABB::new(start, size);

        let mut texture = Texture::new(data_size, WHITE);
        let renderer = RenderingComponent::new_shape(Rectangle, RED, 200);

        renderer.render(&mut texture, &Data::for_two_aabb(0, outer, aabb));

        #[rustfmt::skip]
            let colors = vec![
              RED,   RED, WHITE, WHITE,
            WHITE, WHITE, WHITE, WHITE,
            WHITE, WHITE, WHITE, WHITE,
            WHITE, WHITE, WHITE, WHITE,
        ];

        assert_eq!(texture.get_color_data(), &colors);

        #[rustfmt::skip]
            let depth = vec![
            200, 200, 0, 0,
              0,   0, 0, 0,
              0,   0, 0, 0,
              0,   0, 0, 0,
        ];

        assert_eq!(texture.get_depth_data(), &depth);
    }
}
