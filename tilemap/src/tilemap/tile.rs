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
