use crate::generation::component::Component;
use crate::generation::data::RuntimeData;
use crate::math::aabb::AABB;
use crate::math::color::Color;
use crate::math::size::Size;

pub mod component;
pub mod data;
pub mod layout;
pub mod rendering;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextureGenerator {
    pub name: String,
    pub background: Color,
    pub component: Component,
}

impl TextureGenerator {
    pub fn new<S: Into<String>>(
        name: S,
        background: Color,
        component: Component,
    ) -> TextureGenerator {
        TextureGenerator {
            name: name.into(),
            background,
            component,
        }
    }

    /// Generates the texture with a specific size.
    pub fn generate(&self, width: u32, height: u32) -> RuntimeData {
        let size = Size::new(width, height);
        let aabb = AABB::with_size(size);
        let mut data = RuntimeData::new(size, self.background);

        self.component.generate(&mut data, &aabb);

        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generation::data::Data;
    use crate::generation::rendering::RenderingComponent;
    use crate::math::color::{GREEN, RED};
    use crate::math::shape::Shape;

    #[test]
    fn test_generate() {
        let rectangle = Shape::new_rectangle(2, 4).unwrap();
        let rendering = RenderingComponent::new_shape("test", rectangle, RED);
        let component = Component::Rendering(Box::new(rendering));
        let generator = TextureGenerator::new("test", GREEN, component);

        let data = generator.generate(4, 6);

        #[rustfmt::skip]
        let result = vec![
            GREEN, GREEN, GREEN, GREEN,
            GREEN,   RED,   RED, GREEN,
            GREEN,   RED,   RED, GREEN,
            GREEN,   RED,   RED, GREEN,
            GREEN,   RED,   RED, GREEN,
            GREEN, GREEN, GREEN, GREEN
        ];

        let color_data = data.get_color_data();

        for (index, color) in result.iter().enumerate() {
            let data_index = index * 3;
            assert_eq!(color_data[data_index], color.r());
            assert_eq!(color_data[data_index + 1], color.g());
            assert_eq!(color_data[data_index + 2], color.b());
        }
    }
}
