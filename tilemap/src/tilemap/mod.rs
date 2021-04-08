use self::Side::*;
use std::slice::Iter;

pub mod border;
pub mod node;
pub mod tile;
pub mod tilemap2d;

/// The 4 sides of a [`Tile`].
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Side {
    Top,
    Left,
    Bottom,
    Right,
}

impl Side {
    /// Iterates of all sides counter-clockwise.
    pub fn iterator() -> Iter<'static, Side> {
        static SIDES: [Side; 4] = [Top, Left, Bottom, Right];
        SIDES.iter()
    }

    pub fn is_straight(&self, other: Side) -> bool {
        match self {
            Top => other == Bottom,
            Left => other == Right,
            Bottom => other == Top,
            Right => other == Left,
        }
    }
}
