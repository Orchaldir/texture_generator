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
}

impl WallStyle {
    pub fn new<S: Into<String>>(name: S, wall_generator: WallGenerator) -> WallStyle {
        WallStyle {
            name: name.into(),
            wall_generator,
        }
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
                let start = Point::new(node.x, node.y - *half_thickness);
                let size = Size::new(tile_size, *thickness);
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
    size: u32,
    component: RenderingComponent,
}

impl NodeGenerator {
    pub fn new(size: u32, component: RenderingComponent) -> NodeGenerator {
        NodeGenerator { size, component }
    }
}
