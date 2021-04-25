#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Tile {
    /// Empty like a hole to a lower level or around a flying island.
    Empty,
    /// The ground outside or the floor of a building.
    Floor(usize),
    /// Full of a solid material like earth or stone. E.g. underground
    Solid(usize),
}
