use crate::rendering::resource::Resources;
use crate::tilemap::furniture::map2d::FurnitureMap2d;
use crate::tilemap::furniture::Furniture;
use crate::tilemap::tilemap2d::Tilemap2d;
use crate::tilemap::Side;
use crate::tilemap::Side::*;
use texture_generation::generation::data::texture::Texture;
use texture_generation::generation::data::{AabbData, Data};
use texture_generation::math::aabb::AABB;
use texture_generation::math::point::Point;
use texture_generation::math::size::Size;

/// Renders a [`FurnitureMap2d`] in a specific style.
pub struct FurnitureRenderer<'a> {
    resources: &'a Resources,
    furniture_map: &'a FurnitureMap2d,
    tilemap: &'a Tilemap2d,
    cell_size: Size,
}

impl<'a> FurnitureRenderer<'a> {
    pub fn new(
        resources: &'a Resources,
        furniture_map: &'a FurnitureMap2d,
        tilemap: &'a Tilemap2d,
        tile_size: u32,
    ) -> Self {
        FurnitureRenderer {
            resources,
            furniture_map,
            tilemap,
            cell_size: Size::square(furniture_map.convert_from_tile_size(tile_size)),
        }
    }

    /// Renders a [`FurnitureMap2d`].
    pub fn render(&self, texture: &mut Texture) {
        for (id, furniture) in self.furniture_map.get_furniture() {
            let aabb = self.calculate_aabb(*id, furniture);
            let aabb_data = AabbData::TwoAabbs {
                outer: texture.get_aabb(),
                inner: aabb,
            };
            let data = Data::new(0, *id, aabb_data);

            self.resources
                .furniture_styles
                .get(furniture.style_id)
                .render(texture, &data);
        }
    }

    fn calculate_aabb(&self, id: usize, furniture: &Furniture) -> AABB {
        let start_cell_xy = self.furniture_map.get_size().to_point(furniture.position);
        let start = start_cell_xy * self.cell_size;
        let size = furniture.size * self.cell_size;

        let start_tile_xy = self.furniture_map.convert_to_tile(start_cell_xy);
        let start_tile = self
            .tilemap
            .get_size()
            .to_index(&start_tile_xy)
            .unwrap_or_else(|| panic!("Start point of furniture {} is outside tilemap!", id));

        let end_cell_xy = start_cell_xy + furniture.size - Size::square(1);
        let end_tile_xy = self.furniture_map.convert_to_tile(end_cell_xy);
        let end_tile = self
            .tilemap
            .get_size()
            .to_index(&end_tile_xy)
            .unwrap_or_else(|| panic!("End point of furniture {} is outside tilemap!", id));

        let top_border = self.get_border(start_cell_xy, start_tile, Top);
        let left_border = self.get_border(start_cell_xy, start_tile, Left);
        let bottom_border = self.get_border(end_cell_xy, end_tile, Bottom);
        let right_border = self.get_border(end_cell_xy, end_tile, Right);

        let start = start + Size::new(left_border, top_border);
        let size = Size::new(
            size.width() - left_border - right_border,
            size.height() - top_border - bottom_border,
        );

        AABB::new(start, size)
    }

    fn get_border(&self, cell: Point, tile: usize, side: Side) -> u32 {
        if self.furniture_map.is_border(cell, side) {
            let border = self.tilemap.get_border(tile, side);

            border.get_wall_style().map_or(0, |id| {
                self.resources
                    .wall_styles
                    .get(id)
                    .get_edge_style()
                    .get_thickness()
                    / 2
            })
        } else {
            0
        }
    }
}
