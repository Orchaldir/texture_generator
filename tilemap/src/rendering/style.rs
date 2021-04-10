use texture_generation::generation::component::rendering::RenderingComponent;
use texture_generation::generation::data::Data;
use texture_generation::math::aabb::AABB;
use texture_generation::math::point::Point;
use texture_generation::math::size::Size;

#[derive(Clone, Debug, PartialEq)]
/// Determines how a wall is rendered.
pub struct WallStyle<T> {
    name: String,
    /// The style of a wall between or without nodes.
    edge_style: EdgeStyle,
    /// The optional style of a node between 2 wall segments in the same direction.
    node_style: Option<T>,
    /// The style of corners.
    corner_style: T,
}

impl<T> WallStyle<T> {
    pub fn new<S: Into<String>>(
        name: S,
        edge_style: EdgeStyle,
        node_style: Option<T>,
        corner_style: T,
    ) -> WallStyle<T> {
        WallStyle {
            name: name.into(),
            edge_style,
            node_style,
            corner_style,
        }
    }

    pub fn get_node_style(&self) -> &Option<T> {
        &self.node_style
    }

    pub fn get_corner_style(&self) -> &T {
        &self.corner_style
    }

    pub fn is_greater(&self, other: &WallStyle<T>) -> bool {
        self.edge_style.get_thickness() >= other.edge_style.get_thickness()
    }

    pub fn render_horizontal(
        &self,
        outer: &AABB,
        node: Point,
        tile_size: u32,
        start_node: Option<&NodeStyle>,
        end_node: Option<&NodeStyle>,
        data: &mut dyn Data,
    ) {
        match &self.edge_style {
            EdgeStyle::Mock(..) => {}
            EdgeStyle::Solid {
                thickness,
                half_thickness,
                component,
            } => {
                let start_half = start_node.map(|n| n.half).unwrap_or(0);
                let end_half = end_node.map(|n| n.half).unwrap_or(0);
                let start = Point::new(node.x + start_half, node.y - *half_thickness);
                let size = Size::new(tile_size - (start_half + end_half) as u32, *thickness);
                let aabb = AABB::new(start, size);
                component.render(data, outer, &aabb)
            }
        }
    }

    pub fn render_vertical(
        &self,
        outer: &AABB,
        node: Point,
        tile_size: u32,
        start_node: Option<&NodeStyle>,
        end_node: Option<&NodeStyle>,
        data: &mut dyn Data,
    ) {
        match &self.edge_style {
            EdgeStyle::Mock(..) => {}
            EdgeStyle::Solid {
                thickness,
                half_thickness,
                component,
            } => {
                let start_half = start_node.map(|n| n.half).unwrap_or(0);
                let end_half = end_node.map(|n| n.half).unwrap_or(0);
                let start = Point::new(node.x - *half_thickness, node.y + start_half);
                let size = Size::new(*thickness, tile_size - (start_half + end_half) as u32);
                let aabb = AABB::new(start, size);
                component.render(data, outer, &aabb)
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum EdgeStyle {
    Mock(u32),
    Solid {
        thickness: u32,
        half_thickness: i32,
        component: RenderingComponent,
    },
}

impl EdgeStyle {
    pub fn new_solid(thickness: u32, component: RenderingComponent) -> EdgeStyle {
        EdgeStyle::Solid {
            thickness,
            half_thickness: (thickness / 2) as i32,
            component,
        }
    }
}

impl EdgeStyle {
    pub fn get_thickness(&self) -> u32 {
        match self {
            EdgeStyle::Mock(thickness) => *thickness,
            EdgeStyle::Solid { thickness, .. } => *thickness,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NodeStyle {
    size: Size,
    half: i32,
    component: RenderingComponent,
}

impl NodeStyle {
    pub fn new(size: u32, component: RenderingComponent) -> NodeStyle {
        NodeStyle {
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

#[cfg(test)]
mod tests {
    use super::*;
    use texture_generation::generation::component::rendering::RenderingComponent;
    use texture_generation::generation::data::{Data, RuntimeData};
    use texture_generation::math::color::{BLACK, RED};

    #[test]
    fn test_render_node() {
        let component = RenderingComponent::new_fill_area("corner", RED, 9);
        let node_style = NodeStyle::new(2, component);
        let mut data = RuntimeData::new(Size::new(6, 5), BLACK);

        node_style.render(&data.get_aabb(), Point::new(3, 2), &mut data);

        #[rustfmt::skip]
        let result = vec![
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
            BLACK, BLACK,   RED,   RED, BLACK, BLACK,
            BLACK, BLACK,   RED,   RED, BLACK, BLACK,
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
        ];

        assert_eq!(data.get_color_data(), &result);

        #[rustfmt::skip]
        let depth = vec![
            0, 0, 0, 0, 0, 0,
            0, 0, 9, 9, 0, 0,
            0, 0, 9, 9, 0, 0,
            0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ];

        assert_eq!(data.get_depth_data(), &depth);
    }
}
