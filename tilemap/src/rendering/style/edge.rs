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
        component: LayoutComponent,
    },
    Mock(u32),
    Solid {
        thickness: u32,
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

        Ok(EdgeStyle::Layout {
            thickness,
            component,
        })
    }

    pub fn new_solid(thickness: u32, component: RenderingComponent) -> Result<EdgeStyle> {
        if thickness == 0 {
            bail!("Argument 'thickness' needs to be greater than 0");
        }

        Ok(EdgeStyle::Solid {
            thickness,
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

    pub fn render(&self, data: &Data, node: Point, edge: (i32, u32), texture: &mut Texture) {
        match self {
            EdgeStyle::Layout {
                thickness,
                component,
            } => {
                let aabb = calculate_aabb(node, edge, *thickness);
                component.generate(texture, &data.transform(aabb))
            }
            EdgeStyle::Mock(..) => {}
            EdgeStyle::Solid {
                thickness,
                component,
            } => {
                let aabb = calculate_aabb(node, edge, *thickness);
                component.render(texture, &data.transform(aabb))
            }
        }
    }
}

impl Default for EdgeStyle {
    fn default() -> Self {
        EdgeStyle::default(1).unwrap()
    }
}

fn calculate_aabb(node: Point, edge: (i32, u32), thickness: u32) -> AABB {
    let (start, length) = edge;
    let half_thickness = (thickness / 2) as i32;
    let start = Point::new(node.x + start, node.y - half_thickness);
    let size = Size::new(length, thickness);
    AABB::new(start, size)
}

#[cfg(test)]
mod tests {
    use super::*;
    use texture_generation::generation::component::rendering::RenderingComponent;
    use texture_generation::generation::data::aabb::AabbData;
    use texture_generation::generation::data::texture::Texture;
    use texture_generation::math::color::{BLACK, GREEN};
    use texture_generation::math::side::Side;

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

        edge_style.render(
            &Data::for_texture(texture.get_aabb()),
            Point::new(3, 3),
            (2, 4),
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

    #[test]
    fn test_render_vertical() {
        let edge_component = RenderingComponent::new_fill_area(GREEN, 4);
        let edge_style = EdgeStyle::new_solid(2, edge_component).unwrap();
        let mut texture = Texture::new(Size::new(6, 11), BLACK);
        let aabb_data = AabbData::from_one_aabb(texture.get_aabb());
        let data = Data::with_orientation(0, 0, aabb_data, Side::Bottom);

        edge_style.render(&data, Point::new(3, 3), (2, 4), &mut texture);

        #[rustfmt::skip]
        let result = vec![
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
            BLACK, BLACK, GREEN, GREEN, BLACK, BLACK,
            BLACK, BLACK, GREEN, GREEN, BLACK, BLACK,
            BLACK, BLACK, GREEN, GREEN, BLACK, BLACK,
            BLACK, BLACK, GREEN, GREEN, BLACK, BLACK,
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
        ];

        assert_eq!(texture.get_color_data(), &result);

        #[rustfmt::skip]
        let depth = vec![
            0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
            0, 0, 4, 4, 0, 0,
            0, 0, 4, 4, 0, 0,
            0, 0, 4, 4, 0, 0,
            0, 0, 4, 4, 0, 0,
            0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ];

        assert_eq!(texture.get_depth_data(), &depth);
    }
}
