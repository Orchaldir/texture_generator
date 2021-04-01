use crate::generation::component::layout::LayoutComponent;
use crate::generation::component::rendering::RenderingComponent;
use crate::generation::data::Data;
use crate::math::aabb::AABB;

pub mod layout;
pub mod rendering;

#[derive(Clone, Debug, PartialEq)]
/// A wrapper for different types of components.
pub enum Component {
    Layout(Box<LayoutComponent>),
    Mock(u8),
    Rendering(Box<RenderingComponent>),
}

impl Component {
    /// Generates the texture inside the [`AABB`].
    pub fn generate(&self, data: &mut dyn Data, aabb: &AABB) {
        match self {
            Component::Layout(component) => component.generate(data, aabb),
            Component::Mock(id) => info!("Generate mock {}", *id),
            Component::Rendering(component) => component.render(data, aabb),
        }
    }
}
