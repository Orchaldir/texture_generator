use crate::generation::data::Data;
use crate::generation::layout::LayoutComponent;
use crate::generation::rendering::RenderComponent;
use crate::math::aabb::AABB;

#[derive(Debug, Eq, PartialEq)]
pub enum GenerationComponent {
    Layout(LayoutComponent),
    Rendering(RenderComponent),
}

impl GenerationComponent {
    /// Generates the texture inside the [`AABB`].
    pub fn render(&self, data: &mut dyn Data, aabb: &AABB) {
        match self {
            GenerationComponent::Layout(component) => component.render(data, aabb),
            GenerationComponent::Rendering(component) => component.render(data, aabb),
        }
    }
}
