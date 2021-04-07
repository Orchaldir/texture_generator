use self::Side::*;
use std::slice::Iter;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Tile {
    Empty,
    Floor(usize),
    Full(usize),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Side {
    Top,
    Left,
    Bottom,
    Right,
}

impl Side {
    pub fn iterator() -> Iter<'static, Side> {
        static SIDES: [Side; 4] = [Top, Left, Bottom, Right];
        SIDES.iter()
    }
}
