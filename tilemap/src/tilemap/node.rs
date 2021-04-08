use texture_generation::math::size::Size;

/// Returns the [`Size`] of the nodes based on the size of the [`Tilemap2d`].
pub fn get_nodes_size(size: Size) -> Size {
    Size::new(size.width() + 1, size.height() + 1)
}

/// Returns the index of the node at the start of the horizontal [`Border`].
pub fn get_start_of_horizontal_border(border_index: usize, y: u32) -> usize {
    border_index + y as usize
}

/// Returns the index of the node at the end of the horizontal [`Border`].
pub fn get_end_of_horizontal_border(border_index: usize, y: u32) -> usize {
    border_index + y as usize + 1
}
