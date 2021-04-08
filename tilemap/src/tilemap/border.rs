use texture_generation::math::size::Size;

/// Defines a border between 2 [`Tile`]s.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Border {
    /// No border between tiles.
    Empty,
    /// A wall blocks the border between the 2 tiles.
    Wall(usize),
}

/// Returns the [`Size`] of the horizontal [`Border`]s based on the size of the [`Tilemap2d`].
pub fn get_horizontal_borders_size(size: Size) -> Size {
    Size::new(size.width(), size.height() + 1)
}

/// Returns the [`Size`] of the vertical [`Border`]s based on the size of the [`Tilemap2d`].
pub fn get_vertical_borders_size(size: Size) -> Size {
    Size::new(size.width() + 1, size.height())
}

/// Returns the index of the horizontal [`Border`] below the [`Tile`].
pub fn below_tile(size: Size, tile_index: usize) -> usize {
    tile_index + size.width() as usize
}

/// Returns the index of the vertical [`Border`] to the right of the [`Tile`].
pub fn right_of_tile(tile_index: usize) -> usize {
    tile_index + 1
}
