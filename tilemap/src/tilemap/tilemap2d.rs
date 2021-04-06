use crate::tilemap::border::Border;
use crate::tilemap::tile::{Side, Tile};
use texture_generation::math::size::Size;

/// The tilemap contains the information of what is where,
/// but it doesn't contain how it is rendered.
pub struct Tilemap2d {
    /// The size of a rectangle of [`Tile`].
    size: Size,
    /// A rectangle of [`Tile`]s.
    tiles: Vec<Tile>,
    /// The [`Border`]s at the top & bottom of each [`Tile`].
    horizontal_borders: Vec<Border>,
    /// The [`Border`]s to the left & right of each [`Tile`].
    vertical_borders: Vec<Border>,
}

impl Tilemap2d {
    /// Returns a tilemap of the desired size with the default [`Tile`], but no [`Border`]s.
    pub fn default(size: Size, tile: Tile) -> Tilemap2d {
        let tiles = vec![tile; size.len()];

        Self::new(size, tiles).unwrap()
    }

    /// Returns a tilemap of the desired [`Tile`]s, but no [`Border`]s.
    pub fn new(size: Size, tiles: Vec<Tile>) -> Option<Tilemap2d> {
        let horizontal_borders = vec![Border::Empty; get_horizontal_borders_size(size).len()];
        let vertical_borders = vec![Border::Empty; get_vertical_borders_size(size).len()];

        Self::with_borders(size, tiles, horizontal_borders, vertical_borders)
    }

    /// Returns a tilemap of the desired [`Tile`]s & [`Border`]s.
    pub fn with_borders(
        size: Size,
        tiles: Vec<Tile>,
        horizontal_borders: Vec<Border>,
        vertical_borders: Vec<Border>,
    ) -> Option<Tilemap2d> {
        if size.len() != tiles.len()
            || get_horizontal_borders_size(size).len() != horizontal_borders.len()
            || get_vertical_borders_size(size).len() != vertical_borders.len()
        {
            return None;
        }

        Some(Tilemap2d {
            size,
            tiles,
            horizontal_borders,
            vertical_borders,
        })
    }

    pub fn get_size(&self) -> Size {
        self.size
    }

    // Tiles

    pub fn get_tiles(&self) -> &Vec<Tile> {
        &self.tiles
    }

    pub fn get_tile(&self, index: usize) -> Tile {
        self.tiles[index]
    }

    pub fn set_tile(&mut self, index: usize, tile: Tile) {
        info!("Set index {} to {:?}", index, tile);
        self.tiles[index] = tile;
    }

    /// Borders

    pub fn get_horizontal_borders(&self) -> &Vec<Border> {
        &self.horizontal_borders
    }

    pub fn get_vertical_borders(&self) -> &Vec<Border> {
        &self.vertical_borders
    }

    pub fn get_border(&self, tile_index: usize, side: Side) -> Border {
        if tile_index >= self.size.len() {
            panic!("get_border(): Tile {} is outside the tilemap!", tile_index);
        }

        match side {
            Side::Top => self.horizontal_borders[tile_index],
            Side::Left => self.vertical_borders[tile_index],
            Side::Bottom => self.horizontal_borders[bottom(self.size, tile_index)],
            Side::Right => self.vertical_borders[right(tile_index)],
        }
    }

    pub fn set_border(&mut self, tile_index: usize, side: Side, border: Border) {
        if tile_index >= self.size.len() {
            panic!("set_border(): Index {} is outside the tilemap!", tile_index);
        }

        match side {
            Side::Top => self.horizontal_borders[tile_index] = border,
            Side::Left => self.vertical_borders[tile_index] = border,
            Side::Bottom => self.horizontal_borders[bottom(self.size, tile_index)] = border,
            Side::Right => self.vertical_borders[right(tile_index)] = border,
        };
    }
}

pub fn get_horizontal_borders_size(size: Size) -> Size {
    Size::new(size.width(), size.height() + 1)
}

pub fn get_vertical_borders_size(size: Size) -> Size {
    Size::new(size.width() + 1, size.height())
}

fn bottom(size: Size, tile_index: usize) -> usize {
    tile_index + size.width() as usize
}

fn right(tile_index: usize) -> usize {
    tile_index + 1
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

    #[test]
    fn test_borders() {
        let size = Size::new(3, 3);
        let mut tilemap = Tilemap2d::default(size, Tile::Empty);

        let wall0 = Border::Wall(0);
        let wall1 = Border::Wall(1);
        let wall2 = Border::Wall(2);
        let wall3 = Border::Wall(3);

        tilemap.set_border(4, Side::Top, wall0);
        tilemap.set_border(4, Side::Left, wall1);
        tilemap.set_border(4, Side::Bottom, wall2);
        tilemap.set_border(4, Side::Right, wall3);

        assert_eq!(tilemap.get_border(4, Side::Top), wall0);
        assert_eq!(tilemap.get_border(7, Side::Bottom), wall0);

        assert_eq!(tilemap.get_border(4, Side::Left), wall1);
        assert_eq!(tilemap.get_border(3, Side::Right), wall1);

        assert_eq!(tilemap.get_border(4, Side::Bottom), wall2);
        assert_eq!(tilemap.get_border(1, Side::Top), wall2);

        assert_eq!(tilemap.get_border(4, Side::Right), wall3);
        assert_eq!(tilemap.get_border(5, Side::Left), wall3);
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
