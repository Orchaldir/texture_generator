use crate::generation::component::Component;
use crate::generation::data::RuntimeData;
use crate::math::aabb::AABB;
use crate::math::color::Color;
use crate::math::size::Size;
use crate::process::PostProcess;

pub mod component;
pub mod data;

#[derive(Clone, Debug, PartialEq)]
pub struct TextureGenerator {
    pub name: String,
    pub background: Color,
    pub component: Component,
    pub post_processes: Vec<PostProcess>,
}

impl TextureGenerator {
    pub fn new<S: Into<String>>(
        name: S,
        background: Color,
        component: Component,
        post_processes: Vec<PostProcess>,
    ) -> TextureGenerator {
        TextureGenerator {
            name: name.into(),
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

        //let lighting = Lighting::new(Vector3::new(1.0, 0.0, 0.0), 10, 32);

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
        let generator = TextureGenerator::new("test", GREEN, component, Vec::new());

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
