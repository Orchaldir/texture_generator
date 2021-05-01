use crate::math::occupancy::tile::OccupancyTile;
use crate::math::size::Size;
use std::ops::SubAssign;

pub mod tile;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OccupancyMap {
    tiles: Size,
    cells_per_side: usize,
    tilemap: Vec<OccupancyTile>,
}

impl OccupancyMap {
    pub fn new(tiles: Size, cells_per_side: usize) -> Self {
        Self {
            tiles,
            cells_per_side,
            tilemap: vec![OccupancyTile::Disabled; tiles.len()],
        }
    }

    /// Returns the [`OccupancyTile`]`of a specific tile.
    pub fn get_tile(&self, tile: usize) -> &OccupancyTile {
        &self.tilemap[tile]
    }

    /// Returns the mutable [`OccupancyTile`]`of a specific tile.
    pub fn get_tile_mute(&mut self, tile: usize) -> &mut OccupancyTile {
        &mut self.tilemap[tile]
    }

    /// Enables a specific tile. This should be done for all relevant tiles, before the 1.tile is generated.
    ///
    /// Panics if the tile is already enabled.
    ///
    /// ```
    ///# use texture_generation::math::occupancy::OccupancyMap;
    ///# use texture_generation::math::occupancy::tile::OccupancyTile;
    ///# use texture_generation::math::size::Size;
    /// let mut map = OccupancyMap::new(Size::square(2), 3);
    /// map.enable(1);
    ///
    /// assert_eq!(map.get_tile(0), &OccupancyTile::Disabled);
    /// assert_eq!(map.get_tile(1), &OccupancyTile::Empty);
    /// assert_eq!(map.get_tile(2), &OccupancyTile::Disabled);
    /// assert_eq!(map.get_tile(3), &OccupancyTile::Disabled);
    /// ```
    pub fn enable(&mut self, tile: usize) {
        if self.tilemap[tile] != OccupancyTile::Disabled {
            panic!(
                "Unable to activate tile {}, because it is not disabled!",
                tile
            );
        }

        self.tilemap[tile] = OccupancyTile::Empty;
    }

    /// Activates a specific tile. This alows tile an occupancy between empty & full,
    /// but consumes more memory.
    ///
    /// Panics if the tile is not empty.
    ///
    /// ```
    ///# use texture_generation::math::occupancy::OccupancyMap;
    ///# use texture_generation::math::occupancy::tile::OccupancyTile;
    ///# use texture_generation::math::size::Size;
    /// let mut map = OccupancyMap::new(Size::square(2), 3);
    /// map.enable(1);
    /// map.activate(1);
    ///
    /// assert_eq!(map.get_tile(0), &OccupancyTile::Disabled);
    /// assert_eq!(map.get_tile(1), &OccupancyTile::new_active(3));
    /// assert_eq!(map.get_tile(2), &OccupancyTile::Disabled);
    /// assert_eq!(map.get_tile(3), &OccupancyTile::Disabled);
    /// ```
    pub fn activate(&mut self, tile: usize) {
        if self.tilemap[tile] != OccupancyTile::Empty {
            panic!("Unable to activate tile {}, because it is not empty!", tile);
        }

        self.tilemap[tile] = OccupancyTile::new_active(self.cells_per_side);
    }
}
