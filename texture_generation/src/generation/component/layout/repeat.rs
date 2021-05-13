use crate::generation::component::Component;
use crate::generation::data::texture::Texture;
use crate::generation::data::Data;
use crate::generation::random::Random;
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
    min_step: u32,
    max_step: u32,
    component: Component,
    random: Random,
}

impl RepeatLayout {
    pub fn new(is_horizontal: bool, desired_step: u32, component: Component) -> RepeatLayout {
        RepeatLayout {
            is_horizontal,
            min_step: desired_step,
            max_step: desired_step,
            component,
            random: Random::Hash,
        }
    }

    pub fn new_random(
        is_horizontal: bool,
        min_step: u32,
        max_step: u32,
        component: Component,
        random: Random,
    ) -> RepeatLayout {
        RepeatLayout {
            is_horizontal,
            min_step,
            max_step,
            component,
            random,
        }
    }

    // Flips between horizontal & vertical mode.
    pub fn flip(&self) -> RepeatLayout {
        RepeatLayout {
            is_horizontal: !self.is_horizontal,
            min_step: self.min_step,
            max_step: self.max_step,
            component: self.component.flip(),
            random: self.random.clone(),
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

        for step in self.calculate_steps(&data, inner.size().width()) {
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

        for step in self.calculate_steps(&data, inner.size().height()) {
            let size = Size::new(width, step);
            let aabb = AABB::new(point, size);

            self.component.generate(texture, &data.next(aabb));

            point.y += step as i32;
        }
    }

    fn calculate_steps(&self, data: &Data, distance: u32) -> Vec<u32> {
        if self.min_step == self.max_step {
            let desired_step = self.min_step;
            let factor = (distance % desired_step) as f32 / desired_step as f32;
            let n = distance / desired_step;
            let n = if factor < 0.5 { n } else { n + 1 };
            let step = distance / n;
            let n_big = distance - n * step;

            let mut big_steps = vec![step + 1; n_big as usize];
            let mut steps = vec![step; (n - n_big) as usize];
            big_steps.append(&mut steps);
            big_steps
        } else {
            let step_diff = 1 + self.max_step - self.min_step;
            let mut pos = 0;
            let mut index = 0;
            let mut steps = Vec::new();

            while pos < distance {
                let diff = distance - pos;

                if diff < self.min_step {
                    let n = steps.len() as u32;
                    let bonus = diff / n;
                    let n_big = diff - n * bonus;

                    return steps
                        .into_iter()
                        .enumerate()
                        .map(|(i, step)| step + if (i as u32) < n_big { bonus + 1 } else { bonus })
                        .collect();
                } else if diff < self.max_step {
                    steps.push(diff);
                    break;
                }

                let step =
                    self.min_step + self.random.get_random_instance_u32(data, step_diff, index);
                steps.push(step);

                pos += step;
                index += 1;
            }

            steps
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generation::component::layout::tests::create_component;
    use crate::generation::component::rendering::color::ColorSelector;
    use crate::generation::component::rendering::depth::DepthCalculator;
    use crate::generation::component::rendering::RenderingComponent;
    use crate::generation::data::texture::Texture;
    use crate::math::color::{BLUE, GREEN, RED, WHITE};
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

    #[test]
    fn test_random_x() {
        let size = Size::new(12, 5);
        let aabb = AABB::with_size(size);
        let mut texture = Texture::new(size, WHITE);
        let random = Random::Mock(vec![0, 2, 1]);
        let layout = RepeatLayout::new_random(true, 3, 5, create_random_component(), random);

        layout.generate(&mut texture, Data::for_texture(aabb));

        #[rustfmt::skip]
        let expected_colors = vec![
            RED, RED, RED,  GREEN, GREEN, GREEN, GREEN, GREEN,  BLUE, BLUE, BLUE, BLUE,
            RED, RED, RED,  GREEN, GREEN, GREEN, GREEN, GREEN,  BLUE, BLUE, BLUE, BLUE,
            RED, RED, RED,  GREEN, GREEN, GREEN, GREEN, GREEN,  BLUE, BLUE, BLUE, BLUE,
            RED, RED, RED,  GREEN, GREEN, GREEN, GREEN, GREEN,  BLUE, BLUE, BLUE, BLUE,
            RED, RED, RED,  GREEN, GREEN, GREEN, GREEN, GREEN,  BLUE, BLUE, BLUE, BLUE,
        ];

        assert_eq!(texture.get_color_data(), &expected_colors);
    }

    pub fn create_random_component() -> Component {
        let color = ColorSelector::Sequence(vec![RED, GREEN, BLUE]);
        let depth = DepthCalculator::Uniform(255);
        let renderer = RenderingComponent::new_shape_with_depth("tile", Rectangle, color, depth);
        Component::Rendering(Box::new(renderer))
    }
}