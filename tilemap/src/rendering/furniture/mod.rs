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
        let cell_xy = map_size.to_point(furniture.position);
        let start = cell_xy * self.cell_size;
        let size = furniture.size * self.cell_size;
        let end = start + size;

        let start_tile = self
            .selector
            .get_tile_index(tilemap, start)
            .expect(&format!(
                "Start point of furniture {} is outside tilemap!",
                id
            ));
        let end_tile = self.selector.get_tile_index(tilemap, end).expect(&format!(
            "End point of furniture {} is outside tilemap!",
            id
        ));

        let top_border = get_border(resources, tilemap, start_tile, Side::Top);
        let left_border = get_border(resources, tilemap, start_tile, Side::Left);
        let bottom_border = get_border(resources, tilemap, end_tile, Side::Bottom);
        let right_border = get_border(resources, tilemap, end_tile, Side::Right);

        let start = start + Size::new(left_border, top_border);
        let size = Size::new(
            size.width() - left_border - right_border,
            size.height() - top_border - bottom_border,
        );

        AABB::new(start, size)
    }
}

fn get_border(resources: &Resources, tilemap: &Tilemap2d, tile: usize, side: Side) -> u32 {
    tilemap
        .get_border(tile, side)
        .get_wall_style()
        .map_or(0, |id| {
            resources
                .wall_styles
                .get(id)
                .get_edge_style()
                .get_thickness()
                / 2
        })
}
