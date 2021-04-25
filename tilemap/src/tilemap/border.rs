use texture_generation::math::size::Size;

/// Defines a border between 2 [`Tile`]s.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Border {
    /// No border between tiles.
    Empty,
    /// A wall blocks the border between the 2 tiles.
    Wall(usize),
    /// A wall with a door.
    Door {
        wall_id: usize,
        door_id: usize,
        is_front: bool,
    },
    /// A wall with a window.
    Window { wall_id: usize, window_id: usize },
}

impl Border {
    pub const fn new_door(wall_id: usize, door_id: usize, is_front: bool) -> Border {
        Border::Door {
            wall_id,
            door_id,
            is_front,
        }
    }

    pub const fn new_window(wall_id: usize, window_id: usize) -> Border {
        Border::Window { wall_id, window_id }
    }

    pub fn get_wall_style(&self) -> Option<usize> {
        match self {
            Border::Empty => None,
            Border::Wall(id) => Some(*id),
            Border::Door { wall_id, .. } => Some(*wall_id),
            Border::Window { wall_id, .. } => Some(*wall_id),
        }
    }

    pub fn switch_is_front(&self) -> Border {
        match self {
            Border::Door {
                wall_id,
                door_id,
                is_front,
            } => Border::new_door(*wall_id, *door_id, !*is_front),
            _ => *self,
        }
    }

    pub fn reduce(&self) -> Border {
        match self {
            Border::Empty => Border::Empty,
            Border::Wall(..) => Border::Empty,
            Border::Door { wall_id, .. } => Border::Wall(*wall_id),
            Border::Window { wall_id, .. } => Border::Wall(*wall_id),
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
    const DOOR: Border = Border::new_door(42, 2, true);
    const DOOR2: Border = Border::new_door(42, 2, false);
    const WINDOW: Border = Border::new_window(42, 5);

    #[test]
    fn test_get_wall_style() {
        assert_eq!(Empty.get_wall_style(), None);
        assert_eq!(WALL.get_wall_style(), Some(42));
        assert_eq!(DOOR.get_wall_style(), Some(42));
        assert_eq!(WINDOW.get_wall_style(), Some(42));
    }

    #[test]
    fn test_switch_is_front() {
        assert_eq!(Empty.switch_is_front(), Empty);
        assert_eq!(WALL.switch_is_front(), WALL);
        assert_eq!(DOOR.switch_is_front(), DOOR2);
        assert_eq!(DOOR2.switch_is_front(), DOOR);
        assert_eq!(WINDOW.switch_is_front(), WINDOW);
    }

    #[test]
    fn test_reduce() {
        assert_eq!(Empty.reduce(), Empty);
        assert_eq!(WALL.reduce(), Empty);
        assert_eq!(DOOR.reduce(), WALL);
        assert_eq!(WINDOW.reduce(), WALL);
    }
}
