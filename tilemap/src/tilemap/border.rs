use texture_generation::math::size::Size;

/// Defines a border between 2 [`Tile`]s.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Border {
    /// No border between tiles.
    Empty,
    /// A wall blocks the border between the 2 tiles.
    Wall(usize),
    Door {
        wall_id: usize,
        door_id: usize,
    },
}

impl Border {
    pub fn get_wall_style(&self) -> Option<usize> {
        match self {
            Border::Empty => None,
            Border::Wall(id) => Some(*id),
            Border::Door { wall_id, .. } => Some(*wall_id),
        }
    }

    pub fn reduce(&self) -> Border {
        match self {
            Border::Empty => Border::Empty,
            Border::Wall(..) => Border::Empty,
            Border::Door { wall_id, .. } => Border::Wall(*wall_id),
        }
    }
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
pub fn left_of_tile(size: Size, tile_index: usize) -> usize {
    tile_index + size.to_y(tile_index) as usize
}

/// Returns the index of the horizontal [`Border`] below the [`Tile`].
pub fn below_tile(size: Size, tile_index: usize) -> usize {
    tile_index + size.width() as usize
}

/// Returns the index of the vertical [`Border`] to the right of the [`Tile`].
pub fn right_of_tile(size: Size, tile_index: usize) -> usize {
    left_of_tile(size, tile_index) + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use Border::*;

    const WALL: Border = Wall(42);
    const DOOR: Border = Door {
        wall_id: 42,
        door_id: 2,
    };

    #[test]
    fn test_get_wall_style() {
        assert_eq!(Empty.get_wall_style(), None);
        assert_eq!(WALL.get_wall_style(), Some(42));
        assert_eq!(DOOR.get_wall_style(), Some(42));
    }

    #[test]
    fn test_reduce() {
        assert_eq!(Empty.reduce(), Empty);
        assert_eq!(WALL.reduce(), Empty);
        assert_eq!(DOOR.reduce(), WALL);
    }
}
