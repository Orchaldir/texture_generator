use crate::generation::component::Component;
use crate::generation::data::texture::Texture;
use crate::generation::data::Data;
use crate::math::aabb::AABB;
use crate::math::size::Size;
use anyhow::{bail, Result};

#[derive(Clone, Debug, PartialEq)]
pub enum SplitEntry<T> {
    Fixed(u32, Component),
    Proportional(T, Component),
}

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
    entries: Vec<SplitEntry<f32>>,
    total_fixed_length: u32,
}

impl SplitLayout {
    pub fn new(is_horizontal: bool, entries: Vec<SplitEntry<u32>>) -> Result<SplitLayout> {
        if entries.len() < 2 {
            bail!("Requires at least 2 entries");
        }

        let mut converted = Vec::with_capacity(entries.len());
        let total_proportion = entries
            .iter()
            .map(|entry| match entry {
                SplitEntry::Proportional(v, _) => *v,
                _ => 0,
            })
            .sum::<u32>() as f32;
        let mut total_fixed_length = 0;

        if total_proportion == 0.0 {
            bail!("Requires at least 1 proportional entry");
        }

        for (i, entry) in entries.into_iter().enumerate() {
            match entry {
                SplitEntry::Fixed(length, component) => {
                    if length == 0 {
                        bail!(format!("{}.length is 0", i + 1));
                    }

                    total_fixed_length += length;
                    converted.push(SplitEntry::Fixed(length, component));
                }
                SplitEntry::Proportional(proportion, component) => {
                    if proportion == 0 {
                        bail!(format!("{}.proportion is 0", i + 1));
                    }

                    let factor = proportion as f32 / total_proportion;
                    converted.push(SplitEntry::Proportional(factor, component));
                }
            }
        }

        Ok(SplitLayout {
            is_horizontal,
            entries: converted,
            total_fixed_length,
        })
    }

    pub fn new_proportional(
        is_horizontal: bool,
        entries: Vec<(u32, Component)>,
    ) -> Result<SplitLayout> {
        let converted = convert_proportional(entries, "entries")?
            .into_iter()
            .map(|(proportion, component)| SplitEntry::Proportional(proportion, component))
            .collect();

        Ok(SplitLayout {
            is_horizontal,
            entries: converted,
            total_fixed_length: 0,
        })
    }

    // Flips between horizontal & vertical mode.
    pub fn flip(&self) -> SplitLayout {
        SplitLayout {
            is_horizontal: !self.is_horizontal,
            entries: self
                .entries
                .iter()
                .map(|entry| match entry {
                    SplitEntry::Fixed(v, c) => SplitEntry::Fixed(*v, c.flip()),
                    SplitEntry::Proportional(v, c) => SplitEntry::Proportional(*v, c.flip()),
                })
                .collect(),
            total_fixed_length: self.total_fixed_length,
        }
    }

    /// Generates the component in the area defined by the [`AABB`].
    pub fn generate(&self, texture: &mut Texture, data: Data) {
        if self.is_horizontal {
            self.inner_generate(texture, data.make_horizontal())
        } else {
            self.inner_generate(texture, data.make_vertical())
        }
    }

    fn inner_generate(&self, texture: &mut Texture, mut data: Data) {
        let total_aabb = data.get_aabbs().get_inner();
        let total_width = total_aabb.size().width();
        let height = total_aabb.size().height();
        let mut start = total_aabb.start();

        if total_width < self.total_fixed_length {
            return;
        }

        let remaining_width = total_width - self.total_fixed_length;

        for entry in self.entries.iter() {
            let (width, component) = match entry {
                SplitEntry::Fixed(width, component) => (*width, component),
                SplitEntry::Proportional(factor, component) => {
                    let width = (remaining_width as f32 * *factor) as u32;
                    (width, component)
                }
            };

            let size = Size::new(width, height);
            let aabb = AABB::new(start, size);

            component.generate(texture, &data.next(aabb));
            start.x += width as i32;
        }
    }
}

pub fn convert_proportional<T>(entries: Vec<(u32, T)>, description: &str) -> Result<Vec<(f32, T)>> {
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
    use crate::math::color::{Color, BLUE, GREEN, RED, WHITE, YELLOW};
    use crate::math::size::Size;

    #[test]
    #[should_panic]
    fn test_new_with_too_few_entries() {
        SplitLayout::new(true, vec![proportional(1)]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_new_with_proportion_is_zero() {
        SplitLayout::new(true, vec![proportional(1), proportional(0)]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_new_with_no_proportional_entry() {
        SplitLayout::new(true, vec![fixed(1), fixed(1)]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_new_with_fixed_length_is_0() {
        SplitLayout::new(true, vec![proportional(1), fixed(0)]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_new_proportional_with_too_few_entries() {
        SplitLayout::new_proportional(true, vec![create(1, RED)]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_new_proportional_with_proportion_is_zero() {
        SplitLayout::new_proportional(true, vec![create(0, RED), create(3, GREEN)]).unwrap();
    }

    #[test]
    fn test_split_proportional_x() {
        let size = Size::new(6, 2);
        let aabb = AABB::with_size(size);
        let mut texture = Texture::new(size, WHITE);
        let layout = SplitLayout::new_proportional(
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
        let size = Size::new(2, 8);
        let aabb = AABB::with_size(size);
        let mut texture = Texture::new(size, WHITE);
        let layout = SplitLayout::new(
            false,
            vec![
                create_fixed(3, RED),
                create_proportional(GREEN),
                create_proportional(YELLOW),
                create_fixed(1, BLUE),
            ],
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
            YELLOW, YELLOW,
            YELLOW, YELLOW,
            BLUE, BLUE,
        ];

        assert_eq!(texture.get_color_data(), &expected_colors);
    }

    fn create(size: u32, color: Color) -> (u32, Component) {
        let renderer = RenderingComponent::new_fill_area(color, 200);
        (size, Component::Rendering(Box::new(renderer)))
    }

    fn create_fixed(size: u32, color: Color) -> SplitEntry<u32> {
        let renderer = RenderingComponent::new_fill_area(color, 200);
        SplitEntry::Fixed(size, Component::Rendering(Box::new(renderer)))
    }

    fn create_proportional(color: Color) -> SplitEntry<u32> {
        let renderer = RenderingComponent::new_fill_area(color, 200);
        SplitEntry::Proportional(1, Component::Rendering(Box::new(renderer)))
    }

    fn fixed(size: u32) -> SplitEntry<u32> {
        SplitEntry::Fixed(size, Component::Mock(2))
    }

    fn proportional(size: u32) -> SplitEntry<u32> {
        SplitEntry::Proportional(size, Component::Mock(2))
    }
}
