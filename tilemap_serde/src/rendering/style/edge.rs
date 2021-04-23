use texture_generation::definition::generation::component::layout::LayoutDefinition;
use texture_generation::definition::generation::component::rendering::RenderingDefinition;

#[derive(Clone, Debug, PartialEq)]
pub enum EdgeDefinition {
    Layout {
        thickness: u32,
        layout: LayoutDefinition,
    },
    Mock(u32),
    Solid {
        thickness: u32,
        component: RenderingDefinition,
    },
}
