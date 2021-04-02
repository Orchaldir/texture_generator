pub mod surface;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Tile {
    Empty,
    Floor(usize),
    Full(usize),
}
