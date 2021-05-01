use crate::math::size::Size;
use std::ops::SubAssign;

pub const FREE: usize = 0;
pub const BLOCKED: usize = 1;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OccupancyTile {
    Disabled,
    Empty,
    Active {
        cells: Vec<usize>,
        free_cells: usize,
    },
    Full,
}

impl OccupancyTile {
    pub fn new_active(cells_per_side: usize) -> Self {
        let number = cells_per_side * cells_per_side;
        Self::Active {
            cells: vec![FREE; number],
            free_cells: number,
        }
    }

    pub fn from_cells(cells: Vec<usize>) -> Self {
        let number = cells.iter().filter(|&cell| *cell == FREE).count();
        Self::Active {
            cells,
            free_cells: number,
        }
    }

    /// Does the tile have free cells?
    ///
    /// ```
    ///# use texture_generation::math::occupancy::tile::{OccupancyTile, FREE};
    /// assert_eq!(OccupancyTile::Disabled.has_free_cells(), false);
    /// assert_eq!(OccupancyTile::Empty.has_free_cells(), true);
    /// assert_eq!(OccupancyTile::from_cells(vec![FREE, 2]).has_free_cells(), true);
    /// assert_eq!(OccupancyTile::from_cells(vec![2, 3]).has_free_cells(), false);
    /// assert_eq!(OccupancyTile::Full.has_free_cells(), false);
    /// ```
    pub fn has_free_cells(&self) -> bool {
        match self {
            OccupancyTile::Disabled => false,
            OccupancyTile::Empty => true,
            OccupancyTile::Active { free_cells, .. } => *free_cells > 0,
            OccupancyTile::Full => false,
        }
    }

    /// Is the cell free?
    ///
    /// ```
    ///# use texture_generation::math::occupancy::tile::{OccupancyTile, FREE};
    /// let active_tile = OccupancyTile::from_cells(vec![FREE, 2]);
    ///
    /// assert_eq!(OccupancyTile::Disabled.is_free(0), false);
    /// assert_eq!(OccupancyTile::Empty.is_free(0), true);
    /// assert_eq!(active_tile.is_free(0), true);
    /// assert_eq!(active_tile.is_free(1), false);
    /// assert_eq!(OccupancyTile::Full.is_free(0), false);
    /// ```
    pub fn is_free(&self, cell_index: usize) -> bool {
        match self {
            OccupancyTile::Disabled => false,
            OccupancyTile::Empty => true,
            OccupancyTile::Active { cells, .. } => cells[cell_index] == FREE,
            OccupancyTile::Full => false,
        }
    }

    /// Fills a free cell with a spcific value. Returns true if it was possible.
    ///
    /// ```
    ///# use texture_generation::math::occupancy::tile::{OccupancyTile, FREE};
    /// let mut active_tile = OccupancyTile::from_cells(vec![FREE, FREE, 2]);
    ///
    /// assert_eq!(active_tile.fill(0, 10), true);
    /// assert_eq!(active_tile, OccupancyTile::from_cells(vec![10, FREE, 2]));
    ///
    /// assert_eq!(active_tile.fill(0, 10), false);
    /// ```
    /// Its not possible for tile types other than Active:
    ///
    /// ```
    ///# use texture_generation::math::occupancy::tile::{OccupancyTile, FREE};
    /// assert_eq!(OccupancyTile::Disabled.fill(0,10), false);
    /// assert_eq!(OccupancyTile::Empty.fill(0,10), false);
    /// assert_eq!(OccupancyTile::Full.fill(0,10), false);
    /// ```
    pub fn fill(&mut self, cell_index: usize, value: usize) -> bool {
        match self {
            OccupancyTile::Active { cells, free_cells } => {
                if cells[cell_index] == FREE {
                    cells[cell_index] = value;
                    free_cells.sub_assign(1);
                    return true;
                }
                false
            }
            _ => false,
        }
    }
}

/// Checks if several cells in a column are free.
///
/// ```
///# use texture_generation::math::occupancy::tile::{OccupancyTile, FREE, check_column};
///# use texture_generation::math::size::Size;
/// let size = Size::square(3);
/// let tile = OccupancyTile::from_cells(vec![
///   2, FREE, FREE,
///   FREE, 2, FREE,
///   FREE, FREE, 2]);
///
/// assert_eq!(check_column(&tile, size, 0, 0, 2), false);
/// assert_eq!(check_column(&tile, size, 1, 0, 2), false);
/// assert_eq!(check_column(&tile, size, 2, 0, 2), true);
/// assert_eq!(check_column(&tile, size, 0, 1, 2), true);
/// assert_eq!(check_column(&tile, size, 1, 1, 2), false);
/// assert_eq!(check_column(&tile, size, 2, 1, 2), false);
/// assert_eq!(check_column(&tile, size, 0, 2, 1), true);
/// assert_eq!(check_column(&tile, size, 1, 2, 1), true);
/// assert_eq!(check_column(&tile, size, 2, 2, 1), false);
/// ```
pub fn check_column(
    occupancy_tile: &OccupancyTile,
    tile_size: Size,
    x: u32,
    y: u32,
    height: u32,
) -> bool {
    let mut start_index = tile_size.convert_x_y(x, y);

    for _i in 0..height {
        if !occupancy_tile.is_free(start_index) {
            return false;
        }

        start_index += tile_size.width() as usize;
    }

    true
}

/// Checks if several cells in a row are free.
///
/// ```
///# use texture_generation::math::occupancy::tile::{OccupancyTile, FREE, check_row};
///# use texture_generation::math::size::Size;
/// let size = Size::square(3);
/// let tile = OccupancyTile::from_cells(vec![
///   2, FREE, FREE,
///   FREE, 2, FREE,
///   FREE, FREE, 2]);
///
/// assert_eq!(check_row(&tile, size, 0, 0, 2), false);
/// assert_eq!(check_row(&tile, size, 1, 0, 2), true);
/// assert_eq!(check_row(&tile, size, 2, 0, 1), true);
/// assert_eq!(check_row(&tile, size, 0, 1, 2), false);
/// assert_eq!(check_row(&tile, size, 1, 1, 2), false);
/// assert_eq!(check_row(&tile, size, 2, 1, 1), true);
/// assert_eq!(check_row(&tile, size, 0, 2, 2), true);
/// assert_eq!(check_row(&tile, size, 1, 2, 2), false);
/// assert_eq!(check_row(&tile, size, 2, 2, 1), false);
/// ```
pub fn check_row(
    occupancy_tile: &OccupancyTile,
    tile_size: Size,
    x: u32,
    y: u32,
    width: u32,
) -> bool {
    let mut start_index = tile_size.convert_x_y(x, y);

    for _i in 0..width {
        if !occupancy_tile.is_free(start_index) {
            return false;
        }

        start_index += 1;
    }

    true
}
