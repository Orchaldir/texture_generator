use crate::rendering::style::node::NodeStyle;
use texture_generation::generation::component::layout::LayoutComponent;
use texture_generation::generation::component::rendering::RenderingComponent;
use texture_generation::generation::data::Data;
use texture_generation::math::aabb::AABB;
use texture_generation::math::point::Point;
use texture_generation::math::size::Size;

#[derive(Clone, Debug, PartialEq)]
pub enum EdgeStyle {
    Layout {
        thickness: u32,
        half_thickness: i32,
        horizontal: LayoutComponent,
        vertical: LayoutComponent,
    },
    Mock(u32),
    Solid {
        thickness: u32,
        half_thickness: i32,
        component: RenderingComponent,
    },
}

impl EdgeStyle {
    pub fn default(thickness: u32) -> EdgeStyle {
        Self::new_solid(thickness, RenderingComponent::default())
    }

    pub fn new_layout(thickness: u32, component: LayoutComponent) -> EdgeStyle {
        let vertical = component.flip();
        EdgeStyle::Layout {
            thickness,
            half_thickness: (thickness / 2) as i32,
            horizontal: component,
            vertical,
        }
    }

    pub fn new_solid(thickness: u32, component: RenderingComponent) -> EdgeStyle {
        EdgeStyle::Solid {
            thickness,
            half_thickness: (thickness / 2) as i32,
            component,
        }
    }

    pub fn get_thickness(&self) -> u32 {
        match self {
            EdgeStyle::Layout { thickness, .. } => *thickness,
            EdgeStyle::Mock(thickness) => *thickness,
            EdgeStyle::Solid { thickness, .. } => *thickness,
        }
    }

    pub fn render_horizontal(
        &self,
        outer: &AABB,
        node: Point,
        tile_size: u32,
        offset: i32,
        start_node: Option<&NodeStyle>,
        end_node: Option<&NodeStyle>,
        data: &mut dyn Data,
    ) {
        match self {
            EdgeStyle::Layout {
                thickness,
                half_thickness,
                horizontal,
                ..
            } => {
                let aabb = EdgeStyle::calculate_horizontal_aabb(
                    node,
                    tile_size,
                    offset,
                    start_node,
                    end_node,
                    *thickness,
                    *half_thickness,
                );
                horizontal.generate(data, outer, &aabb)
            }
            EdgeStyle::Mock(..) => {}
            EdgeStyle::Solid {
                thickness,
                half_thickness,
                component,
            } => {
                let aabb = EdgeStyle::calculate_horizontal_aabb(
                    node,
                    tile_size,
                    offset,
                    start_node,
                    end_node,
                    *thickness,
                    *half_thickness,
                );
                component.render(data, outer, &aabb)
            }
        }
    }

    pub fn render_vertical(
        &self,
        outer: &AABB,
        node: Point,
        tile_size: u32,
        offset: i32,
        start_node: Option<&NodeStyle>,
        end_node: Option<&NodeStyle>,
        data: &mut dyn Data,
    ) {
        match self {
            EdgeStyle::Layout {
                thickness,
                half_thickness,
                vertical,
                ..
            } => {
                let aabb = EdgeStyle::calculate_vertical_aabb(
                    node,
                    tile_size,
                    offset,
                    start_node,
                    end_node,
                    *thickness,
                    *half_thickness,
                );
                vertical.generate(data, outer, &aabb)
            }
            EdgeStyle::Mock(..) => {}
            EdgeStyle::Solid {
                thickness,
                half_thickness,
                component,
            } => {
                let aabb = EdgeStyle::calculate_vertical_aabb(
                    node,
                    tile_size,
                    offset,
                    start_node,
                    end_node,
                    *thickness,
                    *half_thickness,
                );
                component.render(data, outer, &aabb)
            }
        }
    }

    fn calculate_horizontal_aabb(
        node: Point,
        tile_size: u32,
        offset: i32,
        start_node: Option<&NodeStyle>,
        end_node: Option<&NodeStyle>,
        thickness: u32,
        half_thickness: i32,
    ) -> AABB {
        let start_half = start_node.map(|n| n.get_half()).unwrap_or(0);
        let end_half = end_node.map(|n| n.get_half()).unwrap_or(0);
        let start = Point::new(node.x + start_half, node.y - half_thickness + offset);
        let size = Size::new(tile_size - (start_half + end_half) as u32, thickness);
        AABB::new(start, size)
    }

    fn calculate_vertical_aabb(
        node: Point,
        tile_size: u32,
        offset: i32,
        start_node: Option<&NodeStyle>,
        end_node: Option<&NodeStyle>,
        thickness: u32,
        half_thickness: i32,
    ) -> AABB {
        let start_half = start_node.map(|n| n.get_half()).unwrap_or(0);
        let end_half = end_node.map(|n| n.get_half()).unwrap_or(0);
        let start = Point::new(node.x - half_thickness + offset, node.y + start_half);
        let size = Size::new(thickness, tile_size - (start_half + end_half) as u32);
        AABB::new(start, size)
    }
}

impl Default for EdgeStyle {
    fn default() -> Self {
        EdgeStyle::default(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use texture_generation::generation::component::rendering::RenderingComponent;
    use texture_generation::generation::data::{Data, RuntimeData};
    use texture_generation::math::color::{BLACK, GREEN, RED};

    #[test]
    fn test_render_horizontal() {
        let component = RenderingComponent::new_fill_area("corner", RED, 9);
        let edge_component = RenderingComponent::new_fill_area("edge", GREEN, 4);
        let node_style0 = NodeStyle::new(4, component.clone());
        let node_style1 = NodeStyle::new(2, component);
        let edge_style = EdgeStyle::new_solid(2, edge_component);
        let mut data = RuntimeData::new(Size::new(11, 6), BLACK);

        edge_style.render_horizontal(
            &data.get_aabb(),
            Point::new(3, 3),
            7,
            0,
            Some(&node_style0),
            Some(&node_style1),
            &mut data,
        );

        #[rustfmt::skip]
        let result = vec![
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
            BLACK, BLACK, BLACK, BLACK, BLACK, GREEN, GREEN, GREEN, GREEN, BLACK, BLACK,
            BLACK, BLACK, BLACK, BLACK, BLACK, GREEN, GREEN, GREEN, GREEN, BLACK, BLACK,
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
        ];

        assert_eq!(data.get_color_data(), &result);

        #[rustfmt::skip]
        let depth = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 4, 4, 4, 4, 0, 0,
            0, 0, 0, 0, 0, 4, 4, 4, 4, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        assert_eq!(data.get_depth_data(), &depth);
    }
}
