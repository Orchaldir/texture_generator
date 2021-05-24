use crate::rendering::style::node::NodeStyle;
use anyhow::{bail, Result};
use texture_generation::generation::component::layout::LayoutComponent;
use texture_generation::generation::component::rendering::RenderingComponent;
use texture_generation::generation::data::texture::Texture;
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
    pub fn default(thickness: u32) -> Result<EdgeStyle> {
        Self::new_solid(thickness, RenderingComponent::default())
    }

    pub fn new_layout(thickness: u32, component: LayoutComponent) -> Result<EdgeStyle> {
        if thickness == 0 {
            bail!("Argument 'thickness' needs to be greater than 0");
        }

        let vertical = component.flip();
        Ok(EdgeStyle::Layout {
            thickness,
            half_thickness: (thickness / 2) as i32,
            horizontal: component,
            vertical,
        })
    }

    pub fn new_solid(thickness: u32, component: RenderingComponent) -> Result<EdgeStyle> {
        if thickness == 0 {
            bail!("Argument 'thickness' needs to be greater than 0");
        }

        Ok(EdgeStyle::Solid {
            thickness,
            half_thickness: (thickness / 2) as i32,
            component,
        })
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
        data: &Data,
        node: Point,
        edge: (i32, u32),
        offset: i32,
        texture: &mut Texture,
    ) {
        match self {
            EdgeStyle::Layout {
                thickness,
                half_thickness,
                horizontal,
                ..
            } => {
                let aabb =
                    calculate_horizontal_aabb2(node, edge, offset, *thickness, *half_thickness);
                horizontal.generate(texture, &data.transform(aabb))
            }
            EdgeStyle::Mock(..) => {}
            EdgeStyle::Solid {
                thickness,
                half_thickness,
                component,
            } => {
                let aabb =
                    calculate_horizontal_aabb2(node, edge, offset, *thickness, *half_thickness);
                component.render(texture, &data.transform(aabb))
            }
        }
    }

    pub fn render_vertical(
        &self,
        data: &Data,
        node: Point,
        tile_size: u32,
        offset: i32,
        start_node: Option<&NodeStyle>,
        end_node: Option<&NodeStyle>,
        texture: &mut Texture,
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
                vertical.generate(texture, &data.transform(aabb))
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
                component.render(texture, &data.transform(aabb))
            }
        }
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
        EdgeStyle::default(1).unwrap()
    }
}

fn calculate_horizontal_aabb2(
    node: Point,
    edge: (i32, u32),
    offset: i32,
    thickness: u32,
    half_thickness: i32,
) -> AABB {
    let (start, length) = edge;
    let start = Point::new(node.x + start, node.y - half_thickness + offset);
    let size = Size::new(length, thickness);
    AABB::new(start, size)
}

#[cfg(test)]
mod tests {
    use super::*;
    use texture_generation::generation::component::rendering::RenderingComponent;
    use texture_generation::generation::data::texture::Texture;
    use texture_generation::math::color::{BLACK, GREEN};

    #[test]
    #[should_panic]
    fn test_new_layout_with_thickness_too_small() {
        EdgeStyle::new_layout(0, LayoutComponent::Mock(8)).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_new_solid_with_thickness_too_small() {
        EdgeStyle::new_solid(0, RenderingComponent::default()).unwrap();
    }

    #[test]
    fn test_render_horizontal() {
        let edge_component = RenderingComponent::new_fill_area(GREEN, 4);
        let edge_style = EdgeStyle::new_solid(2, edge_component).unwrap();
        let mut texture = Texture::new(Size::new(11, 6), BLACK);

        edge_style.render_horizontal(
            &Data::for_texture(texture.get_aabb()),
            Point::new(3, 3),
            (2, 4),
            0,
            &mut texture,
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

        assert_eq!(texture.get_color_data(), &result);

        #[rustfmt::skip]
        let depth = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 4, 4, 4, 4, 0, 0,
            0, 0, 0, 0, 0, 4, 4, 4, 4, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        assert_eq!(texture.get_depth_data(), &depth);
    }
}
