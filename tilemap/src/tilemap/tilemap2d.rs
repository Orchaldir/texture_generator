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

    /// Returns the [`Border`] on a specific side of a node.
    pub fn get_border_at_node(&self, node_index: usize, side: Side) -> Border {
        let node_size = get_node_size(self.size);
        let point = node_size.to_point(node_index);

        match side {
            Side::Top => {
                if point.y == 0 {
                    Border::Empty
                } else {
                    self.vertical_borders[node_index - node_size.width() as usize]
                }
            }
            Side::Left => {
                if point.x == 0 {
                    Border::Empty
                } else {
                    self.horizontal_borders[node_index - 1 - point.y as usize]
                }
            }
            Side::Bottom => {
                if point.y >= (node_size.height() - 1) as i32 {
                    Border::Empty
                } else {
                    self.vertical_borders[node_index]
                }
            }
            Side::Right => {
                if point.x == (node_size.width() - 1) as i32 {
                    Border::Empty
                } else {
                    self.horizontal_borders[node_index - point.y as usize]
                }
            }
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

fn get_node_size(size: Size) -> Size {
    Size::new(size.width() + 1, size.height() + 1)
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

    const WALL0: Border = Border::Wall(0);
    const WALL1: Border = Border::Wall(1);
    const WALL2: Border = Border::Wall(2);
    const WALL3: Border = Border::Wall(3);

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

        tilemap.set_border(4, Side::Top, WALL0);
        tilemap.set_border(4, Side::Left, WALL1);
        tilemap.set_border(4, Side::Bottom, WALL2);
        tilemap.set_border(4, Side::Right, WALL3);

        assert_eq!(tilemap.get_border(4, Side::Top), WALL0);
        assert_eq!(tilemap.get_border(1, Side::Bottom), WALL0);

        assert_eq!(tilemap.get_border(4, Side::Left), WALL1);
        assert_eq!(tilemap.get_border(3, Side::Right), WALL1);

        assert_eq!(tilemap.get_border(4, Side::Bottom), WALL2);
        assert_eq!(tilemap.get_border(7, Side::Top), WALL2);

        assert_eq!(tilemap.get_border(4, Side::Right), WALL3);
        assert_eq!(tilemap.get_border(5, Side::Left), WALL3);
    }

    #[test]
    fn test_get_node() {
        let size = Size::new(2, 2);

        let tilemap = Tilemap2d::with_borders(
            size,
            vec![Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            vec![
                Border::Empty,
                Border::Empty,
                WALL1,
                WALL3,
                Border::Empty,
                Border::Empty,
            ],
            vec![
                Border::Empty,
                WALL0,
                Border::Empty,
                Border::Empty,
                WALL2,
                Border::Empty,
            ],
        )
        .unwrap();

        assert_eq!(tilemap.get_border_at_node(4, Side::Top), WALL0);
        assert_eq!(tilemap.get_border_at_node(1, Side::Bottom), WALL0);

        assert_eq!(tilemap.get_border_at_node(4, Side::Left), WALL1);
        assert_eq!(tilemap.get_border_at_node(3, Side::Right), WALL1);

        assert_eq!(tilemap.get_border_at_node(4, Side::Bottom), WALL2);
        assert_eq!(tilemap.get_border_at_node(7, Side::Top), WALL2);

        assert_eq!(tilemap.get_border_at_node(4, Side::Right), WALL3);
        assert_eq!(tilemap.get_border_at_node(5, Side::Left), WALL3);
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
