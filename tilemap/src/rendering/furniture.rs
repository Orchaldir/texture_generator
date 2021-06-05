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
                .render(self.resources, texture, &data);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rendering::style::edge::EdgeStyle;
    use crate::rendering::style::furniture::FurnitureStyle;
    use crate::rendering::style::wall::WallStyle;
    use crate::tilemap::border::Border;
    use crate::tilemap::tile::Tile;
    use texture_generation::generation::component::rendering::RenderingComponent;
    use texture_generation::generation::component::Component;
    use texture_generation::generation::data::texture::Texture;
    use texture_generation::math::color::{Color, BLACK, GREEN, RED};
    use texture_generation::utils::resource::ResourceManager;

    #[test]
    fn test_render_furniture_covering_whole_tiles() {
        let resources = create_resources();

        let size = Size::new(1, 2);
        let mut tilemap = Tilemap2d::default(size, Tile::Empty);
        tilemap.set_border(0, Top, Border::Wall(0));
        tilemap.set_border(0, Left, Border::Wall(1));
        tilemap.set_border(1, Right, Border::Wall(2));
        tilemap.set_border(1, Bottom, Border::Wall(3));

        let mut furniture_map = FurnitureMap2d::empty(size);
        furniture_map.add(Furniture::new(0, 0, Size::new(2, 4)));

        let mut texture = Texture::new(Size::new(8, 16), BLACK);

        let renderer = FurnitureRenderer::new(&resources, &furniture_map, &tilemap, 8);

        renderer.render(&mut texture);

        #[rustfmt::skip]
        let result = vec![
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
            BLACK, BLACK, GREEN, GREEN, GREEN, BLACK, BLACK, BLACK,
            BLACK, BLACK, GREEN, GREEN, GREEN, BLACK, BLACK, BLACK,
            BLACK, BLACK, GREEN, GREEN, GREEN, BLACK, BLACK, BLACK,
            BLACK, BLACK, GREEN, GREEN, GREEN, BLACK, BLACK, BLACK,
            BLACK, BLACK, GREEN, GREEN, GREEN, BLACK, BLACK, BLACK,
            BLACK, BLACK, GREEN, GREEN, GREEN, BLACK, BLACK, BLACK,
            BLACK, BLACK, GREEN, GREEN, GREEN, BLACK, BLACK, BLACK,

            BLACK, BLACK, GREEN, GREEN, GREEN, BLACK, BLACK, BLACK,
            BLACK, BLACK, GREEN, GREEN, GREEN, BLACK, BLACK, BLACK,
            BLACK, BLACK, GREEN, GREEN, GREEN, BLACK, BLACK, BLACK,
            BLACK, BLACK, GREEN, GREEN, GREEN, BLACK, BLACK, BLACK,
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
        ];

        assert_eq!(texture.get_color_data(), &result);
    }

    #[test]
    fn test_render_furniture_no_touching_a_border() {
        let resources = create_resources();

        let size = Size::new(1, 1);
        let mut tilemap = Tilemap2d::default(size, Tile::Empty);
        tilemap.set_border(0, Left, Border::Wall(0));
        tilemap.set_border(0, Bottom, Border::Wall(0));

        let mut furniture_map = FurnitureMap2d::empty(size);
        furniture_map.add(Furniture::new(0, 1, Size::new(1, 1)));

        let mut texture = Texture::new(Size::new(8, 8), BLACK);

        let renderer = FurnitureRenderer::new(&resources, &furniture_map, &tilemap, 8);

        renderer.render(&mut texture);

        #[rustfmt::skip]
        let result = vec![
            BLACK, BLACK, BLACK, BLACK, GREEN, GREEN, GREEN, GREEN,
            BLACK, BLACK, BLACK, BLACK, GREEN, GREEN, GREEN, GREEN,
            BLACK, BLACK, BLACK, BLACK, GREEN, GREEN, GREEN, GREEN,
            BLACK, BLACK, BLACK, BLACK, GREEN, GREEN, GREEN, GREEN,
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
        ];

        assert_eq!(texture.get_color_data(), &result);
    }

    fn create_resources() -> Resources {
        let mut resources = Resources::empty();
        resources.furniture_styles = ResourceManager::new(
            vec![create_furniture("f0", GREEN)],
            FurnitureStyle::default(),
        );
        resources.wall_styles = ResourceManager::new(
            vec![
                create_wall("wall0", RED, 2),
                create_wall("wall1", RED, 4),
                create_wall("wall2", RED, 6),
                create_wall("wall3", RED, 8),
            ],
            WallStyle::default(1),
        );
        resources
    }

    fn create_wall(name: &str, color: Color, thickness: u32) -> WallStyle {
        let rendering = RenderingComponent::new_fill_area(color, 1);
        let edge = EdgeStyle::new_solid(thickness, rendering).unwrap();
        WallStyle::new(name, edge, None, None)
    }

    fn create_furniture(name: &str, color: Color) -> FurnitureStyle {
        let rendering = RenderingComponent::new_fill_area(color, 1);
        let component = Component::Rendering(Box::new(rendering));
        FurnitureStyle::new(name, component)
    }
}
