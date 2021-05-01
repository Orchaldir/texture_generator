use crate::generation::component::rendering::RenderingComponent;
use crate::generation::component::Component;
use crate::generation::data::{Data, RuntimeData};
use crate::math::aabb::AABB;
use crate::math::color::{Color, PINK};
use crate::math::size::Size;

pub mod component;
pub mod data;
pub mod process;

#[derive(Clone, Debug, PartialEq)]
pub struct TextureGenerator {
    name: String,
    size: Size,
    background: Color,
    component: Component,
}

impl TextureGenerator {
    pub fn new<S: Into<String>>(
        name: S,
        size: Size,
        background: Color,
        component: Component,
    ) -> TextureGenerator {
        TextureGenerator {
            name: name.into(),
            size,
            background,
            component,
        }
    }

    /// Generates the texture with a specific size.
    pub fn generate(&self) -> RuntimeData {
        let aabb = AABB::with_size(self.size);
        let mut data = RuntimeData::new(self.size, self.background);

        self.component.generate(&mut data, &aabb, &aabb);

        data
    }

    /// Generates the texture to a specific part of [`Data`].
    pub fn render(&self, data: &mut dyn Data, aabb: &AABB) {
        let background = RenderingComponent::new_fill_area("background", self.background, 0);

        background.render(data, aabb, aabb);
        self.component.generate(data, aabb, aabb);
    }
}

impl Default for TextureGenerator {
    fn default() -> Self {
        TextureGenerator::new("default", Size::square(1), PINK, Component::Mock(42))
    }
}

#[cfg(test)]
mod tests {
    use crate::generation::component::border::BorderComponent;
    use crate::generation::component::rendering::RenderingComponent;
    use crate::generation::data::Data;
    use crate::math::color::{GREEN, RED};

    use super::*;

    #[test]
    fn test_generate() {
        let rendering = RenderingComponent::new_fill_area("test", RED, 200);
        let rendering_component = Component::Rendering(Box::new(rendering));
        let border = BorderComponent::new_uniform(1, rendering_component);
        let border_component = Component::Border(Box::new(border));
        let generator = TextureGenerator::new("test", Size::new(5, 7), GREEN, border_component);

        let data = generator.generate();

        #[rustfmt::skip]
            let result = vec![
            GREEN, GREEN, GREEN, GREEN, GREEN,
            GREEN,   RED,   RED,   RED, GREEN,
            GREEN,   RED,   RED,   RED, GREEN,
            GREEN,   RED,   RED,   RED, GREEN,
            GREEN,   RED,   RED,   RED, GREEN,
            GREEN,   RED,   RED,   RED, GREEN,
            GREEN, GREEN, GREEN, GREEN, GREEN
        ];

        assert_eq!(data.get_color_data(), &result);
    }
}
