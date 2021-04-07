use texture_generation::generation::component::rendering::RenderingComponent;
use texture_generation::generation::data::Data;
use texture_generation::math::aabb::AABB;
use texture_generation::math::point::Point;
use texture_generation::math::size::Size;

#[derive(Clone, Debug, PartialEq)]
/// Determines how a wall is rendered.
pub struct WallStyle {
    name: String,
    /// The style of a wall between or without nodes.
    wall_generator: WallGenerator,
    /// The optional style of a node between 2 wall segments in the same direction.
    node_generator: NodeGenerator,
}

impl WallStyle {
    pub fn new<S: Into<String>>(
        name: S,
        wall_generator: WallGenerator,
        node_generator: NodeGenerator,
    ) -> WallStyle {
        WallStyle {
            name: name.into(),
            wall_generator,
            node_generator,
        }
    }

    pub fn get_node_generator(&self) -> &NodeGenerator {
        &self.node_generator
    }

    pub fn render_horizontal(
        &self,
        outer: &AABB,
        node: Point,
        tile_size: u32,
        data: &mut dyn Data,
    ) {
        match &self.wall_generator {
            WallGenerator::Solid {
                thickness,
                half_thickness,
                component,
            } => {
                let node_half = self.node_generator.half;
                let start = Point::new(node.x + node_half, node.y - *half_thickness);
                let size = Size::new(tile_size - (node_half * 2) as u32, *thickness);
                let aabb = AABB::new(start, size);
                component.render(data, outer, &aabb)
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum WallGenerator {
    Solid {
        thickness: u32,
        half_thickness: i32,
        component: RenderingComponent,
    },
}

impl WallGenerator {
    pub fn new_solid(thickness: u32, component: RenderingComponent) -> WallGenerator {
        WallGenerator::Solid {
            thickness,
            half_thickness: (thickness / 2) as i32,
            component,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NodeGenerator {
    size: Size,
    half: i32,
    component: RenderingComponent,
}

impl NodeGenerator {
    pub fn new(size: u32, component: RenderingComponent) -> NodeGenerator {
        NodeGenerator {
            size: Size::square(size),
            half: (size / 2) as i32,
            component,
        }
    }

    pub fn render(&self, outer: &AABB, node: Point, data: &mut dyn Data) {
        let start = node - self.half;
        let aabb = AABB::new(start, self.size);
        self.component.render(data, outer, &aabb)
    }
}
