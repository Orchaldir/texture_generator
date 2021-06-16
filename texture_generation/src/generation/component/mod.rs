use crate::generation::component::border::BorderComponent;
use crate::generation::component::layout::LayoutComponent;
use crate::generation::component::rendering::RenderingComponent;
use crate::generation::data::texture::Texture;
use crate::generation::data::Data;

pub mod border;
pub mod layout;
pub mod rendering;

#[derive(Clone, Debug, PartialEq)]
/// A wrapper for different types of components.
pub enum Component {
    Border(Box<BorderComponent>),
    Empty,
    Layers(Vec<Component>),
    Layout(Box<LayoutComponent>),
    Mock(u8),
    Rendering(Box<RenderingComponent>),
}

impl Component {
    /// Generates the texture inside the [`AABB`].
    pub fn generate(&self, texture: &mut Texture, data: &Data) {
        match self {
            Component::Border(component) => component.generate(texture, data),
            Component::Layers(layers) => layers
                .iter()
                .for_each(|component| component.generate(texture, data)),
            Component::Layout(component) => component.generate(texture, data),
            Component::Rendering(component) => component.render(texture, data),
            _ => {}
        }
    }
}
