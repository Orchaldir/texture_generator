use crate::generation::process::PostProcess;
use crate::math::aabb::AABB;
use crate::math::color::Color;
use crate::math::occupancy::OccupancyMap;
use crate::math::point::Point;
use crate::math::size::Size;
use std::collections::HashMap;

/// Stores all the data of the texture.
pub struct Texture {
    size: Size,
    tiles: Size,
    tile_size: Size,
    colors: Vec<Color>,
    depth: Vec<u8>,
    base_depth: u8,
    occupancy_maps: HashMap<usize, OccupancyMap>,
}

impl Texture {
    pub fn new(size: Size, default: Color) -> Texture {
        Self::with_depth(size, default, 0)
    }

    pub fn with_depth(size: Size, default: Color, base_depth: u8) -> Texture {
        Self::tilemap_with_depth(Size::square(1), size, default, base_depth)
    }

    pub fn for_tilemap(tiles: Size, tile_size: Size, default: Color) -> Texture {
        Self::tilemap_with_depth(tiles, tile_size, default, 0)
    }

    pub fn tilemap_with_depth(
        tiles: Size,
        tile_size: Size,
        default: Color,
        base_depth: u8,
    ) -> Texture {
        let size = tile_size * tiles;
        let n = size.len();
        let colors = vec![default; n];
        let depth = vec![0; n];

        Texture {
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

    /// Gets the [`Size`] of the textures.
    pub fn get_size(&self) -> &Size {
        &self.size
    }

    /// Gets the [`Size`] of the tilemap.
    pub fn get_tiles(&self) -> &Size {
        &self.tiles
    }

    /// Gets the [`Size`] of each tile.
    pub fn get_tile_size(&self) -> &Size {
        &self.tile_size
    }

    /// Sets the [`Color`] & depth at the [`Point`].
    pub fn set(&mut self, point: &Point, color: &Color, depth: u8) {
        let index = self.size.to_index_risky(point);

        self.depth[index] = depth;
        self.colors[index] = *color;
    }

    /// Gets all the colors.
    pub fn get_color_data(&self) -> &[Color] {
        &self.colors
    }

    /// Gets all the colors mutable.
    pub fn get_color_data_mut(&mut self) -> &mut [Color] {
        &mut self.colors
    }

    /// Gets all the depth values.
    pub fn get_depth_data(&self) -> &[u8] {
        &self.depth
    }

    /// Gets the base depth for the current tile of the tilemap.
    pub fn get_base_depth(&self) -> u8 {
        self.base_depth
    }

    /// Gets the mutable [`OccupancyMap`] with a specific resolution. Creates it, if it doesn't exist yet.
    pub fn get_occupancy_map_mut(&mut self, cells_per_side: usize) -> &mut OccupancyMap {
        let tiles = self.tiles;
        self.occupancy_maps
            .entry(cells_per_side)
            .or_insert_with(|| OccupancyMap::new(tiles, cells_per_side))
    }

    /// Applies the post processes.
    pub fn apply(&mut self, post_processes: &[PostProcess]) {
        for post_process in post_processes.iter() {
            post_process.process(self);
        }
    }
}
