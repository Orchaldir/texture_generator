use texture_generation::generation::component::Component;
use texture_generation::generation::data::RuntimeData;
use texture_generation::generation::process::PostProcess;
use texture_generation::math::aabb::AABB;
use texture_generation::math::color::Color;
use texture_generation::math::size::Size;

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
    pub fn generate(&self) -> RuntimeData {
        let aabb = AABB::with_size(self.size);
        let mut data = RuntimeData::new(self.size, self.background);

        self.component.generate(&mut data, &aabb);

        for post_process in self.post_processes.iter() {
            post_process.process(&mut data);
        }

        data
    }
}

#[cfg(test)]
mod tests {
    use texture_generation::generation::component::rendering::RenderingComponent;
    use texture_generation::generation::data::Data;
    use texture_generation::math::color::{GREEN, RED};
    use texture_generation::math::shape::Shape;

    use super::*;

    #[test]
    fn test_generate() {
        let rectangle = Shape::new_rectangle(2, 4).unwrap();
        let rendering = RenderingComponent::new_shape("test", rectangle, RED);
        let component = Component::Rendering(Box::new(rendering));
        let generator =
            TextureGenerator::new("test", Size::new(5, 7), GREEN, component, Vec::new());

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
