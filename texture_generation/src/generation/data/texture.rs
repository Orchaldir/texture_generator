use crate::generation::process::PostProcess;
use crate::math::aabb::AABB;
use crate::math::color::{convert, Color};
use crate::math::occupancy::OccupancyMap;
use crate::math::point::Point;
use crate::math::size::Size;
use std::collections::HashMap;

/// A trait used to store the data during the generation of the texture.
pub trait Data {
    /// Gets the [`Size`] of the textures.
    fn get_size(&self) -> &Size;

    /// Gets the [`Size`] of the tilemap.
    fn get_tiles(&self) -> &Size;

    /// Gets the [`Size`] of each tile.
    fn get_tile_size(&self) -> &Size;

    /// Sets the [`Color`] & depth at the [`Point`].
    fn set(&mut self, point: &Point, color: &Color, depth: u8);

    /// Gets all the colors.
    fn get_color_data(&self) -> &[Color];

    /// Gets all the colors mutable.
    fn get_color_data_mut(&mut self) -> &mut [Color];

    /// Gets all the depth values.
    fn get_depth_data(&self) -> &[u8];

    /// Gets the base depth for the current tile of the tilemap.
    fn get_base_depth(&self) -> u8;

    /// Gets the mutable [`OccupancyMap`] with a specific resolution. Creates it, if it doesn't exist yet.
    fn get_occupancy_map_mut(&mut self, cells_per_side: usize) -> &mut OccupancyMap;
}

/// An implementation of [`Data`] for the actual usage.
pub struct RuntimeData {
    size: Size,
    tiles: Size,
    tile_size: Size,
    colors: Vec<Color>,
    depth: Vec<u8>,
    base_depth: u8,
    occupancy_maps: HashMap<usize, OccupancyMap>,
}

impl RuntimeData {
    pub fn new(size: Size, default: Color) -> RuntimeData {
        Self::with_depth(size, default, 0)
    }

    pub fn with_depth(size: Size, default: Color, base_depth: u8) -> RuntimeData {
        Self::tilemap_with_depth(Size::square(1), size, default, base_depth)
    }

    pub fn for_tilemap(tiles: Size, tile_size: Size, default: Color) -> RuntimeData {
        Self::tilemap_with_depth(tiles, tile_size, default, 0)
    }

    pub fn tilemap_with_depth(
        tiles: Size,
        tile_size: Size,
        default: Color,
        base_depth: u8,
    ) -> RuntimeData {
        let size = tile_size * tiles;
        let n = size.len();
        let colors = vec![default; n];
        let depth = vec![0; n];

        RuntimeData {
            size,
            tiles,
            tile_size,
            colors,
            depth,
            base_depth,
            occupancy_maps: HashMap::new(),
        }
    }

    pub fn get_aabb(&self) -> AABB {
        AABB::with_size(self.size)
    }

    pub fn set_base_depth(&mut self, depth: u8) {
        self.base_depth = depth;
    }

    /// Applies the post processes.
    pub fn apply(&mut self, post_processes: &[PostProcess]) {
        for post_process in post_processes.iter() {
            post_process.process(self);
        }
    }

    /// Save the color image.
    pub fn save_color_image(&self, path: &str) {
        info!("Save color to {:?}", path);

        let color_data = convert(&self.colors);

        image::save_buffer(
            path,
            &color_data,
            self.size.width(),
            self.size.height(),
            image::ColorType::Rgb8,
        )
        .unwrap();
    }

    /// Save the depth image.
    pub fn save_depth_image(&self, path: &str) {
        info!("Save depth to {:?}", path);

        image::save_buffer(
            path,
            &self.depth,
            self.size.width(),
            self.size.height(),
            image::ColorType::L8,
        )
        .unwrap();
    }
}

impl Data for RuntimeData {
    fn get_size(&self) -> &Size {
        &self.size
    }

    fn get_tiles(&self) -> &Size {
        &self.tiles
    }

    fn get_tile_size(&self) -> &Size {
        &self.tile_size
    }

    fn set(&mut self, point: &Point, color: &Color, depth: u8) {
        let index = self.size.to_index_risky(point);

        self.depth[index] = depth;
        self.colors[index] = *color;
    }

    fn get_color_data(&self) -> &[Color] {
        &self.colors
    }

    fn get_color_data_mut(&mut self) -> &mut [Color] {
        &mut self.colors
    }

    fn get_depth_data(&self) -> &[u8] {
        &self.depth
    }

    fn get_base_depth(&self) -> u8 {
        self.base_depth
    }

    fn get_occupancy_map_mut(&mut self, cells_per_side: usize) -> &mut OccupancyMap {
        let tiles = self.tiles;
        self.occupancy_maps
            .entry(cells_per_side)
            .or_insert_with(|| OccupancyMap::new(tiles, cells_per_side))
    }
}
