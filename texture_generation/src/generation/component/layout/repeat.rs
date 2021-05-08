use crate::generation::component::Component;
use crate::generation::data::texture::Texture;
use crate::generation::data::Data;
use crate::math::aabb::AABB;
use crate::math::size::Size;

#[svgbobdoc::transform]
/// Repeats a component along the x-axis or y-axis.
///
/// # Diagram
///
/// If `is_horizontal` is true:
///
/// ```svgbob
///   +---*---*---*---*
///   |   |   |   |   |
///   |   |   |   |   |
///   |   |   |   |   |
///   |   |   |   |   |
///   |   |   |   |   |
///   *---*---*---*---*
/// ```
///
/// else:
///
/// ```svgbob
///   +--------*
///   |        |
///   *--------*
///   |        |
///   *--------*
///   |        |
///   *--------*
///   |        |
///   *--------*
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct RepeatLayout {
    is_horizontal: bool,
    desired_step: u32,
    component: Component,
}

impl RepeatLayout {
    pub fn new(is_horizontal: bool, desired_step: u32, component: Component) -> RepeatLayout {
        RepeatLayout {
            is_horizontal,
            desired_step,
            component,
        }
    }

    // Flips between horizontal & vertical mode.
    pub fn flip(&self) -> RepeatLayout {
        RepeatLayout {
            is_horizontal: !self.is_horizontal,
            desired_step: self.desired_step,
            component: self.component.flip(),
        }
    }

    /// Generates the pattern in all the repeating areas intersected by the [`AABB`].
    pub fn generate(&self, texture: &mut Texture, data: Data) {
        if self.is_horizontal {
            self.generate_horizontal(texture, data)
        } else {
            self.generate_vertical(texture, data)
        }
    }

    fn generate_horizontal(&self, texture: &mut Texture, mut data: Data) {
        let inner = data.get_inner();
        let height = inner.size().height();
        let mut point = inner.start();

        for step in self.calculate_steps(inner.size().width()) {
            let size = Size::new(step, height);
            let aabb = AABB::new(point, size);

            self.component.generate(texture, &data.next(aabb));

            point.x += step as i32;
        }
    }

    fn generate_vertical(&self, texture: &mut Texture, mut data: Data) {
        let inner = data.get_inner();
        let width = inner.size().width();
        let mut point = inner.start();

        for step in self.calculate_steps(inner.size().height()) {
            let size = Size::new(width, step);
            let aabb = AABB::new(point, size);

            self.component.generate(texture, &data.next(aabb));

            point.y += step as i32;
        }
    }

    fn calculate_steps(&self, distance: u32) -> Vec<u32> {
        let factor = (distance % self.desired_step) as f32 / self.desired_step as f32;
        let n = distance / self.desired_step;
        let n = if factor < 0.5 { n } else { n + 1 };
        let step = distance / n;
        let n_big = distance - n * step;

        let mut big_steps = vec![step + 1; n_big as usize];
        let mut steps = vec![step; (n - n_big) as usize];
        big_steps.append(&mut steps);
        big_steps
    }
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
        let layout = RepeatLayout::new(true, 5, create_component());

        layout.generate(&mut texture, Data::for_texture(aabb));

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
        let layout = RepeatLayout::new(false, 5, create_component());

        layout.generate(&mut textzre, Data::for_texture(aabb));

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
