use crate::generation::data::Data;
use crate::generation::layout::LayoutComponent;
use crate::generation::rendering::RenderingComponent;
use crate::math::aabb::AABB;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Component {
    Layout(LayoutComponent),
    Rendering(RenderingComponent),
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
