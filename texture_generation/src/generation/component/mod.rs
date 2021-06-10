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
    /// Flips between horizontal & vertical mode.
    pub fn flip(&self) -> Component {
        match self {
            Component::Border(component) => Component::Border(Box::new(component.flip())),
            Component::Empty => Component::Empty,
            Component::Layers(layers) => {
                Component::Layers(layers.iter().map(|component| component.flip()).collect())
            }
            Component::Layout(component) => Component::Layout(Box::new(component.flip())),
            Component::Mock(id) => Component::Mock(*id),
            Component::Rendering(component) => Component::Rendering(Box::new(component.flip())),
        }
    }

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
