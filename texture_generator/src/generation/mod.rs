use crate::generation::component::Component;
use crate::generation::data::RuntimeData;
use crate::generation::process::PostProcess;
use crate::math::aabb::AABB;
use crate::math::color::Color;
use crate::math::size::Size;

pub mod component;
pub mod data;
pub mod process;

#[derive(Clone, Debug, PartialEq)]
pub struct TextureGenerator {
    pub name: String,
    pub size: Size,
    pub background: Color,
    pub component: Component,
    pub post_processes: Vec<PostProcess>,
}

impl TextureGenerator {
    pub fn new<S: Into<String>>(
        name: S,
        size: Size,
        background: Color,
        component: Component,
        post_processes: Vec<PostProcess>,
    ) -> TextureGenerator {
        TextureGenerator {
            name: name.into(),
            size,
            background,
            component,
            post_processes,
        }
    }

    /// Generates the texture with a specific size.
    pub fn generate(&self, width: u32, height: u32) -> RuntimeData {
        let size = Size::new(width, height);
        let aabb = AABB::with_size(size);
        let mut data = RuntimeData::new(size, self.background);

        self.component.generate(&mut data, &aabb);

        for post_process in self.post_processes.iter() {
            post_process.process(&mut data);
        }

        data
    }
}

#[cfg(test)]
mod tests {
    use crate::generation::component::rendering::RenderingComponent;
    use crate::generation::data::Data;
    use crate::math::color::{GREEN, RED};
    use crate::math::shape::Shape;

    use super::*;

    #[test]
    fn test_generate() {
        let rectangle = Shape::new_rectangle(2, 4).unwrap();
        let rendering = RenderingComponent::new_shape("test", rectangle, RED);
        let component = Component::Rendering(Box::new(rendering));
        let generator =
            TextureGenerator::new("test", Size::new(1, 1), GREEN, component, Vec::new());

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

        assert_eq!(data.get_color_data(), &result);
    }
}
