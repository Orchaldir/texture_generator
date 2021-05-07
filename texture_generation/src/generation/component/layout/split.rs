use crate::generation::component::Component;
use crate::generation::data::texture::Texture;
use crate::generation::data::Data;
use crate::math::aabb::AABB;
use crate::math::size::Size;

#[svgbobdoc::transform]
/// Splits an area into different components of different sizes.
///
/// # Diagram
///
/// ```svgbob
///   +--*-----*---*
///   |  |     |   |
///   |  |     |   |
///   |  |     |   |
///   |  |     |   |
///   |  |     |   |
///   *--*-----*---*
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct SplitLayout {
    is_horizontal: bool,
    components: Vec<(f32, Component)>,
}

impl SplitLayout {
    pub fn new(is_horizontal: bool, components: Vec<(u32, Component)>) -> SplitLayout {
        let total = components.iter().map(|(value, _c)| *value).sum::<u32>() as f32;

        SplitLayout {
            is_horizontal,
            components: components
                .into_iter()
                .map(|(v, c)| (v as f32 / total, c))
                .collect(),
        }
    }

    /// Generates the pattern in all the repeating areas intersected by the [`AABB`].
    pub fn generate(&self, texture: &mut Texture, data: Data) {
        if self.is_horizontal {
            self.generate_horizontal(texture, data)
        }
    }

    fn generate_horizontal(&self, texture: &mut Texture, mut data: Data) {
        let total_aabb = data.get_inner();
        let total_width = total_aabb.size().width();
        let height = total_aabb.size().height();
        let mut start = total_aabb.start();

        for (factor, component) in self.components.iter() {
            let width = (total_width as f32 * *factor) as u32;
            let size = Size::new(width, height);
            let aabb = AABB::new(start, size);

            component.generate(texture, &data.next(aabb));

            start.x += width as i32;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generation::component::rendering::RenderingComponent;
    use crate::generation::data::texture::Texture;
    use crate::math::color::{Color, BLUE, GREEN, RED, WHITE};
    use crate::math::size::Size;

    #[test]
    fn test_split_x() {
        let size = Size::new(6, 2);
        let aabb = AABB::with_size(size);
        let mut texture = Texture::new(size, WHITE);
        let layout = SplitLayout::new(
            true,
            vec![create(1, RED), create(3, GREEN), create(2, BLUE)],
        );

        layout.generate(&mut texture, Data::for_texture(aabb));

        #[rustfmt::skip]
        let expected_colors = vec![
            RED, GREEN, GREEN, GREEN, BLUE, BLUE,
            RED, GREEN, GREEN, GREEN, BLUE, BLUE,
        ];

        assert_eq!(texture.get_color_data(), &expected_colors);
    }

    fn create(size: u32, color: Color) -> (u32, Component) {
        let renderer = RenderingComponent::new_fill_area("area", color, 200);
        (size, Component::Rendering(Box::new(renderer)))
    }
}
