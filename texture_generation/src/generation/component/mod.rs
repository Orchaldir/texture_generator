use crate::generation::component::border::BorderComponent;
use crate::generation::component::layout::LayoutComponent;
use crate::generation::component::rendering::RenderingComponent;
use crate::generation::data::texture::Data;
use crate::math::aabb::AABB;

pub mod border;
pub mod layout;
pub mod rendering;

#[derive(Clone, Debug, PartialEq)]
/// A wrapper for different types of components.
pub enum Component {
    Border(Box<BorderComponent>),
    Layout(Box<LayoutComponent>),
    Mock(u8),
    Rendering(Box<RenderingComponent>),
}

impl Component {
    /// Flips between horizontal & vertical mode.
    pub fn flip(&self) -> Component {
        match self {
            Component::Border(component) => Component::Border(Box::new(component.flip())),
            Component::Layout(component) => Component::Layout(Box::new(component.flip())),
            Component::Mock(id) => Component::Mock(*id),
            Component::Rendering(component) => Component::Rendering(Box::new(component.flip())),
        }
    }

    /// Generates the texture inside the [`AABB`].
    pub fn generate(&self, data: &mut dyn Data, outer: &AABB, inner: &AABB) {
        match self {
            Component::Border(component) => component.generate(data, outer, inner),
            Component::Layout(component) => component.generate(data, outer, inner),
            Component::Mock(id) => info!("Generate mock {}", *id),
            Component::Rendering(component) => component.render(data, outer, inner),
        }
    }
}
