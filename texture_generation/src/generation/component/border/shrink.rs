use crate::generation::component::Component;
use crate::generation::data::texture::Texture;
use crate::generation::data::Data;
use crate::generation::random::Random;
use crate::math::aabb::AABB;
use crate::math::point::Point;
use crate::math::size::Size;
use anyhow::{bail, Result};

#[svgbobdoc::transform]
/// Shrinks a [`Component`] along the x-axis or y-axis.
#[derive(Clone, Debug, PartialEq)]
pub struct ShrinkAxis {
    is_horizontal: bool,
    min_border: u32,
    border_diff: u32,
    component: Component,
    random: Random,
}

impl ShrinkAxis {
    pub fn new(
        is_horizontal: bool,
        desired_border: u32,
        component: Component,
    ) -> Result<ShrinkAxis> {
        if desired_border == 0 {
            bail!("Argument 'desired_border' needs to be greater than 0");
        }

        Ok(ShrinkAxis {
            is_horizontal,
            min_border: desired_border,
            border_diff: 0,
            component,
            random: Random::Hash,
        })
    }

    pub fn new_random(
        is_horizontal: bool,
        min_border: u32,
        max_border: u32,
        component: Component,
        random: Random,
    ) -> Result<ShrinkAxis> {
        if max_border <= min_border {
            bail!("Argument 'max_border' needs to be greater than 'min_border'");
        }

        Ok(ShrinkAxis {
            is_horizontal,
            min_border,
            border_diff: 1 + max_border - min_border,
            component,
            random,
        })
    }

    /// Generates the pattern in all the repeating areas intersected by the [`AABB`].
    pub fn generate(&self, texture: &mut Texture, data: &Data) {
        let border = self.calculate_random_border(&data);
        let old_start = data.get_aabbs().get_inner().start();
        let old_size = data.get_aabbs().get_inner().size();

        let (start, size) = if self.is_horizontal {
            (
                Point::new(old_start.x + border as i32, old_start.y),
                Size::new(old_size.width() - border * 2, old_size.height()),
            )
        } else {
            (
                Point::new(old_start.x, old_start.y + border as i32),
                Size::new(old_size.width(), old_size.height() - border * 2),
            )
        };

        let aabb = AABB::new(start, size);
        self.component.generate(texture, &data.transform(aabb));
    }

    fn calculate_random_border(&self, data: &Data) -> u32 {
        if self.border_diff == 0 {
            return self.min_border;
        }

        self.min_border
            + self
                .random
                .get_random_instance_u32(&data, self.border_diff, 42)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generation::component::rendering::RenderingComponent;
    use crate::generation::data::texture::Texture;
    use crate::math::color::{RED, WHITE};
    use crate::math::size::Size;

    #[test]
    #[should_panic]
    fn test_new() {
        ShrinkAxis::new(true, 0, Component::Mock(42)).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_new_random() {
        ShrinkAxis::new_random(true, 3, 3, Component::Mock(42), Random::Hash).unwrap();
    }

    #[test]
    fn test_shrink_x() {
        let size = Size::new(8, 2);
        let aabb = AABB::with_size(size);
        let mut texture = Texture::new(size, WHITE);

        let random = Random::Mock(vec![1, 2, 3]);
        let renderer = RenderingComponent::new_fill_area(RED, 1);
        let component = Component::Rendering(Box::new(renderer));
        let layout = ShrinkAxis::new_random(true, 1, 3, component, random).unwrap();

        layout.generate(&mut texture, &Data::for_texture(aabb));

        #[rustfmt::skip]
        let expected_colors = vec![
            WHITE, WHITE, RED, RED, RED, RED, WHITE, WHITE,
            WHITE, WHITE, RED, RED, RED, RED, WHITE, WHITE,
        ];

        assert_eq!(texture.get_color_data(), &expected_colors);
    }

    #[test]
    fn test_shrink_y() {
        let size = Size::new(2, 8);
        let aabb = AABB::with_size(size);
        let mut texture = Texture::new(size, WHITE);

        let random = Random::Mock(vec![4, 5, 6]);
        let renderer = RenderingComponent::new_fill_area(RED, 1);
        let component = Component::Rendering(Box::new(renderer));
        let layout = ShrinkAxis::new_random(false, 1, 3, component, random).unwrap();

        layout.generate(&mut texture, &Data::for_texture(aabb));

        #[rustfmt::skip]
        let expected_colors = vec![
            WHITE, WHITE,
            WHITE, WHITE,
            RED, RED,
            RED, RED,
            RED, RED,
            RED, RED,
            WHITE, WHITE,
            WHITE, WHITE,
        ];

        assert_eq!(texture.get_color_data(), &expected_colors);
    }
}
