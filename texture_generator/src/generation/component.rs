use crate::generation::data::Data;
use crate::generation::layout::LayoutComponent;
use crate::generation::rendering::RenderingComponent;
use crate::math::aabb::AABB;

#[derive(Clone, Debug, Eq, PartialEq)]
/// A wrapper for different types of components.
pub enum Component {
    Layout(Box<LayoutComponent>),
    Rendering(Box<RenderingComponent>),
}

impl Component {
    /// Generates the texture inside the [`AABB`].
    pub fn generate(&self, data: &mut dyn Data, aabb: &AABB) {
        match self {
            Component::Layout(component) => component.generate(data, aabb),
            Component::Rendering(component) => component.render(data, aabb),
        }
    }
}
