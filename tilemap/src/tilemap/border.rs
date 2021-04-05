#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Border {
    Empty,
    Wall(usize),
}
