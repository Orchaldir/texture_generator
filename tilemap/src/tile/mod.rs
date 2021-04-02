pub mod surface;

pub enum Tile {
    Empty,
    Floor(usize),
    Full(usize),
}
