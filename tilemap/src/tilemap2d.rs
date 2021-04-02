use crate::tile::Tile;
use texture_generation::math::size::Size;

pub struct Tilemap2d {
    size: Size,
    tiles: Vec<Tile>,
}

impl Tilemap2d {
    pub fn default(size: Size, tile: Tile) -> Tilemap2d {
        let tiles = vec![tile; size.get_number_of_cells()];
        Tilemap2d { size, tiles }
    }

    pub fn new(size: Size, tiles: Vec<Tile>) -> Option<Tilemap2d> {
        if size.get_number_of_cells() != tiles.len() {
            return None;
        }

        Some(Tilemap2d { size, tiles })
    }

    pub fn get_size(&self) -> Size {
        self.size
    }

    pub fn get_tile(&self, index: usize) -> Tile {
        self.tiles[index]
    }

    pub fn get_tiles(&self) -> &Vec<Tile> {
        &self.tiles
    }

    pub fn set_tile(&mut self, index: usize, tile: Tile) {
        info!("Set index {} to {:?}", index, tile);
        self.tiles[index] = tile;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use texture_generation::math::size::Size;

    #[test]
    fn test_default() {
        let size = Size::new(2, 3);
        let tilemap = Tilemap2d::default(size, Tile::Empty);

        assert_eq!(tilemap.get_size(), size);

        for i in 0..6 {
            assert_eq!(tilemap.get_tile(i), Tile::Empty);
        }
    }

    #[test]
    fn test_new() {
        let size = Size::new(2, 3);
        let tiles = create_tiles();
        let tilemap = Tilemap2d::new(size, tiles.clone()).unwrap();

        assert_eq!(tilemap.get_size(), size);
        assert_eq!(tilemap.get_tiles(), &tiles);
    }

    #[test]
    fn test_set_tile() {
        let size = Size::new(2, 3);
        let mut tilemap = Tilemap2d::default(size, Tile::Empty);

        tilemap.set_tile(0, Tile::Floor(1));
        tilemap.set_tile(2, Tile::Full(3));
        tilemap.set_tile(4, Tile::Floor(4));

        assert_eq!(tilemap.get_size(), size);
        assert_eq!(tilemap.get_tiles(), &create_tiles());
    }

    fn create_tiles() -> Vec<Tile> {
        vec![
            Tile::Floor(1),
            Tile::Empty,
            Tile::Full(3),
            Tile::Empty,
            Tile::Floor(4),
            Tile::Empty,
        ]
    }
}
