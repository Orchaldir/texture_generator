use crate::rendering::resource::Resources;
use crate::tilemap::furniture::map2d::{FurnitureMap2d, RESOLUTION};
use crate::tilemap::furniture::Furniture;
use crate::tilemap::selector::Selector;
use crate::tilemap::tilemap2d::Tilemap2d;
use crate::tilemap::Side;
use texture_generation::generation::component::rendering::color::ColorSelector;
use texture_generation::generation::component::rendering::depth_factory::DepthFactory;
use texture_generation::generation::component::rendering::RenderingComponent;
use texture_generation::generation::data::texture::Texture;
use texture_generation::generation::data::{AabbData, Data};
use texture_generation::math::aabb::AABB;
use texture_generation::math::color::{BLUE, GREEN, RED};
use texture_generation::math::shape_factory::ShapeFactory;
use texture_generation::math::size::Size;

/// Renders a [`FurnitureMap2d`] in a specific style.
pub struct FurnitureRenderer {
    cell_size: Size,
    selector: Selector,
    wall_height: u8,
}

impl FurnitureRenderer {
    pub fn new(tile_size: u32, wall_height: u8) -> Self {
        FurnitureRenderer {
            cell_size: Size::square(tile_size).divide(RESOLUTION),
            selector: Selector::new(tile_size),
            wall_height,
        }
    }

    /// Renders a [`FurnitureMap2d`].
    pub fn render(
        &self,
        texture: &mut Texture,
        furniture_map: &FurnitureMap2d,
        tilemap: &Tilemap2d,
        resources: &Resources,
    ) {
        let color_selector = ColorSelector::Sequence(vec![RED, GREEN, BLUE]);
        let depth_factory = DepthFactory::new_dome(50, 1);
        let component = RenderingComponent::new_shape_with_depth(
            ShapeFactory::RoundedRectangle(0.3),
            color_selector,
            depth_factory,
        );
        let map_size = furniture_map.get_size();

        texture.set_base_depth(100);

        for (id, furniture) in furniture_map.get_furniture() {
            let aabb = self.calculate_aabb(&map_size, *id, furniture, tilemap, resources);
            let aabb_data = AabbData::TwoAabbs {
                outer: texture.get_aabb(),
                inner: aabb,
            };
            let data = Data::new(0, *id, aabb_data);

            component.render(texture, &data);
        }
    }

    fn calculate_aabb(
        &self,
        map_size: &Size,
        id: usize,
        furniture: &Furniture,
        tilemap: &Tilemap2d,
        resources: &Resources,
    ) -> AABB {
        let start_cell_xy = map_size.to_point(furniture.position);
        let start = start_cell_xy * self.cell_size;
        let size = furniture.size * self.cell_size;

        let start_tile_xy = start_cell_xy / RESOLUTION;
        let start_tile = tilemap.get_size().to_index(&start_tile_xy).expect(&format!(
            "Start point of furniture {} is outside tilemap!",
            id
        ));

        info!(
            "id={} pos={} start: cell={:?} tile={}",
            id, furniture.position, start_cell_xy, start_tile
        );

        let end_cell_xy = start_cell_xy + furniture.size - Size::square(1);
        let end_tile_xy = end_cell_xy / RESOLUTION;
        let end_tile = tilemap.get_size().to_index(&end_tile_xy).expect(&format!(
            "End point of furniture {} is outside tilemap!",
            id
        ));

        info!("end: cell={:?} tile={}", end_cell_xy, end_tile);

        let top_border = if start_cell_xy.y % RESOLUTION as i32 == 0 {
            get_border(resources, tilemap, start_tile, Side::Top)
        } else {
            0
        };
        let left_border = if start_cell_xy.x % RESOLUTION as i32 == 0 {
            get_border(resources, tilemap, start_tile, Side::Left)
        } else {
            0
        };
        let bottom_border = if end_cell_xy.y % RESOLUTION as i32 != 0 {
            get_border(resources, tilemap, end_tile, Side::Bottom)
        } else {
            0
        };
        let right_border = if end_cell_xy.x % RESOLUTION as i32 != 0 {
            get_border(resources, tilemap, end_tile, Side::Right)
        } else {
            0
        };

        let start = start + Size::new(left_border, top_border);
        let size = Size::new(
            size.width() - left_border - right_border,
            size.height() - top_border - bottom_border,
        );

        AABB::new(start, size)
    }
}

fn get_border(resources: &Resources, tilemap: &Tilemap2d, tile: usize, side: Side) -> u32 {
    let border = tilemap.get_border(tile, side);

    let thickness = border.get_wall_style().map_or(0, |id| {
        resources
            .wall_styles
            .get(id)
            .get_edge_style()
            .get_thickness()
            / 2
    });

    info!(
        "tile={} side={:?} border={:?} thickness={}",
        tile, side, border, thickness
    );
    thickness
}
