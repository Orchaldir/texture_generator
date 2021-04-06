use texture_generation::generation::component::rendering::RenderingComponent;

#[derive(Clone, Debug, PartialEq)]
/// Determines how a wall is rendered.
pub struct WallStyle {
    name: String,
    /// The style of a wall between or without nodes.
    wall_generator: WallGenerator,
    /// The optional style of a node between 2 wall segments in the same direction.
    node_generator: NodeGenerator,
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
    component: RenderingComponent,
}

impl NodeGenerator {
    pub fn new(size: u32, component: RenderingComponent) -> NodeGenerator {
        NodeGenerator { size, component }
    }
}
