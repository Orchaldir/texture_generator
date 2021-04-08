use texture_generation::math::size::Size;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Border {
    Empty,
    Wall(usize),
}

pub fn get_horizontal_borders_size(size: Size) -> Size {
    Size::new(size.width(), size.height() + 1)
}

pub fn get_vertical_borders_size(size: Size) -> Size {
    Size::new(size.width() + 1, size.height())
}

pub fn below_tile(size: Size, tile_index: usize) -> usize {
    tile_index + size.width() as usize
}

pub fn right_of_tile(tile_index: usize) -> usize {
    tile_index + 1
}
