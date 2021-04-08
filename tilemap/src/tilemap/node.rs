use texture_generation::math::size::Size;

pub fn get_nodes_size(size: Size) -> Size {
    Size::new(size.width() + 1, size.height() + 1)
}
