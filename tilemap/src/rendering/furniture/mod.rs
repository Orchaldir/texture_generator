use crate::tilemap::furniture::map2d::{FurnitureMap2d, RESOLUTION};
use crate::tilemap::tilemap2d::Tilemap2d;
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
    tile_size: u32,
    wall_height: u8,
}

impl FurnitureRenderer {
    pub fn new(tile_size: u32, wall_height: u8) -> Self {
        FurnitureRenderer {
            tile_size,
            wall_height,
        }
    }

    /// Renders a [`FurnitureMap2d`].
    pub fn render(&self, texture: &mut Texture, furniture_map: &FurnitureMap2d) {
        let color_selector = ColorSelector::Sequence(vec![RED, GREEN, BLUE]);
        let depth_factory = DepthFactory::new_dome(50, 1);
        let component = RenderingComponent::new_shape_with_depth(
            ShapeFactory::RoundedRectangle(0.3),
            color_selector,
            depth_factory,
        );
        let cell_size = Size::square(self.tile_size).divide(RESOLUTION);
        let map_size = furniture_map.get_size();

        texture.set_base_depth(100);

        for (id, furniture) in furniture_map.get_furniture() {
            let cell_xy = map_size.to_point(furniture.position);
            let start = cell_xy * cell_size;
            info!(
                "id={} pos={} cell={:?} start={:?}",
                *id, furniture.position, cell_xy, start
            );
            let size = furniture.size * cell_size;
            let aabb_data = AabbData::TwoAabbs {
                outer: texture.get_aabb(),
                inner: AABB::new(start, size),
            };
            let data = Data::new(0, *id, aabb_data);

            component.render(texture, &data);
        }
    }
}
