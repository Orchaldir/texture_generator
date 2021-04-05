use texture_generation::generation::component::rendering::RenderingComponent;
use texture_generation::generation::component::Component;
use texture_generation::generation::TextureGenerator;

#[derive(Clone, Debug, PartialEq)]
pub struct WallStyle {
    name: String,
    wall_generator: WallGenerator,
    corner_generator: Option<TextureGenerator>,
    node_generator: Option<TextureGenerator>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WallGenerator {
    Solid {
        thickness: u32,
        component: RenderingComponent,
    },
}

impl WallGenerator {
    pub fn new_solid(thickness: u32, component: RenderingComponent) -> WallGenerator {
        WallGenerator::Solid {
            thickness,
            component,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NodeGenerator {
    size: u32,
    component: Component,
}

impl NodeGenerator {
    pub fn new(size: u32, component: Component) -> NodeGenerator {
        NodeGenerator { size, component }
    }
}
