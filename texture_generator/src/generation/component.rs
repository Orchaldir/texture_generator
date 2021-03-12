use crate::generation::data::Data;
use crate::generation::layout::LayoutComponent;
use crate::generation::rendering::RenderingComponent;
use crate::math::aabb::AABB;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GenerationComponent {
    Layout(LayoutComponent),
    Rendering(RenderingComponent),
}

impl GenerationComponent {
    /// Generates the texture inside the [`AABB`].
    pub fn generate(&self, data: &mut dyn Data, aabb: &AABB) {
        match self {
            GenerationComponent::Layout(component) => component.generate(data, aabb),
            GenerationComponent::Rendering(component) => component.render(data, aabb),
        }
    }
}
