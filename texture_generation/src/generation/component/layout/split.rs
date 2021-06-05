use crate::generation::component::Component;
use crate::generation::data::texture::Texture;
use crate::generation::data::Data;
use crate::math::aabb::AABB;
use crate::math::size::Size;
use anyhow::{bail, Result};

#[svgbobdoc::transform]
/// Splits an area into different components of different sizes.
///
/// # Diagram
///
/// If `is_horizontal` is true:
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
///
/// else:
///
/// ```svgbob
///   +--------*
///   |        |
///   |        |
///   *--------*
///   |        |
///   *--------*
///   |        |
///   |        |
///   |        |
///   *--------*
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct SplitLayout {
    is_horizontal: bool,
    components: Vec<(f32, Component)>,
}

impl SplitLayout {
    pub fn new(is_horizontal: bool, components: Vec<(u32, Component)>) -> Result<SplitLayout> {
        let converted = convert(components, "components")?;

        Ok(SplitLayout {
            is_horizontal,
            components: converted,
        })
    }

    // Flips between horizontal & vertical mode.
    pub fn flip(&self) -> SplitLayout {
        SplitLayout {
            is_horizontal: !self.is_horizontal,
            components: self
                .components
                .iter()
                .map(|(v, c)| (*v, c.flip()))
                .collect(),
        }
    }

    /// Generates the component in the area defined by the [`AABB`].
    pub fn generate(&self, texture: &mut Texture, data: Data) {
        if self.is_horizontal {
            self.generate_horizontal(texture, data)
        } else {
            self.generate_vertical(texture, data)
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

    fn generate_vertical(&self, texture: &mut Texture, mut data: Data) {
        let total_aabb = data.get_inner();
        let width = total_aabb.size().width();
        let total_height = total_aabb.size().height();
        let mut start = total_aabb.start();

        for (factor, component) in self.components.iter() {
            let height = (total_height as f32 * *factor) as u32;
            let size = Size::new(width, height);
            let aabb = AABB::new(start, size);

            component.generate(texture, &data.next(aabb));

            start.y += height as i32;
        }
    }
}

pub fn convert<T>(entries: Vec<(u32, T)>, description: &str) -> Result<Vec<(f32, T)>> {
    if entries.len() < 2 {
        bail!("Requires at least 2 '{}'", description);
    }

    let mut converted = Vec::with_capacity(entries.len());
    let total = entries.iter().map(|(value, _c)| *value).sum::<u32>() as f32;

    for (i, (proportion, value)) in entries.into_iter().enumerate() {
        if proportion == 0 {
            bail!(format!("{}.proportion is 0", i + 1));
        }

        converted.push((proportion as f32 / total, value))
    }

    Ok(converted)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generation::component::rendering::RenderingComponent;
    use crate::generation::data::texture::Texture;
    use crate::math::color::{Color, BLUE, GREEN, RED, WHITE};
    use crate::math::size::Size;

    #[test]
    #[should_panic]
    fn test_new_with_too_few_components() {
        SplitLayout::new(true, vec![create(1, RED)]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_new_with_proportion_is_zero() {
        SplitLayout::new(true, vec![create(0, RED), create(3, GREEN)]).unwrap();
    }

    #[test]
    fn test_split_x() {
        let size = Size::new(6, 2);
        let aabb = AABB::with_size(size);
        let mut texture = Texture::new(size, WHITE);
        let layout = SplitLayout::new(
            true,
            vec![create(1, RED), create(3, GREEN), create(2, BLUE)],
        )
        .unwrap();

        layout.generate(&mut texture, Data::for_texture(aabb));

        #[rustfmt::skip]
        let expected_colors = vec![
            RED, GREEN, GREEN, GREEN, BLUE, BLUE,
            RED, GREEN, GREEN, GREEN, BLUE, BLUE,
        ];

        assert_eq!(texture.get_color_data(), &expected_colors);
    }

    #[test]
    fn test_split_y() {
        let size = Size::new(2, 6);
        let aabb = AABB::with_size(size);
        let mut texture = Texture::new(size, WHITE);
        let layout = SplitLayout::new(
            false,
            vec![create(3, RED), create(2, GREEN), create(1, BLUE)],
        )
        .unwrap();

        layout.generate(&mut texture, Data::for_texture(aabb));

        #[rustfmt::skip]
        let expected_colors = vec![
            RED, RED,
            RED, RED,
            RED, RED,
            GREEN, GREEN,
            GREEN, GREEN,
            BLUE, BLUE,
        ];

        assert_eq!(texture.get_color_data(), &expected_colors);
    }

    fn create(size: u32, color: Color) -> (u32, Component) {
        let renderer = RenderingComponent::new_fill_area(color, 200);
        (size, Component::Rendering(Box::new(renderer)))
    }
}
