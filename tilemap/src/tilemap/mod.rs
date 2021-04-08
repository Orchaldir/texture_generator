use self::Side::*;
use std::slice::Iter;

pub mod border;
pub mod node;
pub mod tile;
pub mod tilemap2d;

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
