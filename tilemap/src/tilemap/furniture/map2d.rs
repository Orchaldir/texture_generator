use crate::tilemap::furniture::Furniture;
use crate::tilemap::Side;
use std::collections::HashMap;
use texture_generation::math::point::Point;
use texture_generation::math::size::Size;

const RESOLUTION: u32 = 2;

pub struct FurnitureMap2d {
    size: Size,
    /// The id of the next [`Furniture`]. It starts at 1.
    next_id: usize,
    furniture: HashMap<usize, Furniture>,
}

impl FurnitureMap2d {
    pub fn empty(tilemap_size: Size) -> FurnitureMap2d {
        let size = tilemap_size * RESOLUTION as f32;

        FurnitureMap2d {
            size,
            next_id: 1,
            furniture: HashMap::new(),
        }
    }

    pub fn get_furniture(&self) -> &HashMap<usize, Furniture> {
        &self.furniture
    }

    pub fn get_size(&self) -> &Size {
        &self.size
    }

    /// Calculates the cell size from the size of a [`Tile`].
    pub fn convert_from_tile_size(&self, tile_size: u32) -> u32 {
        tile_size / RESOLUTION
    }

    /// Calculates the cell size from the size of a [`Tile`].
    pub fn convert_to_tile(&self, cell: Point) -> Point {
        cell / RESOLUTION
    }

    /// Is the side of the cell a [`Border`] of the [`Tilemap2d`] or in the middle of a [`Tile`]?
    pub fn is_border(&self, cell: Point, side: Side) -> bool {
        match side {
            Side::Top => cell.y % RESOLUTION as i32 == 0,
            Side::Left => cell.x % RESOLUTION as i32 == 0,
            Side::Bottom => cell.y % RESOLUTION as i32 != 0,
            Side::Right => cell.x % RESOLUTION as i32 != 0,
        }
    }

    pub fn add(&mut self, furniture: Furniture) -> usize {
        let id = self.next_id;
        self.furniture.insert(id, furniture);
        self.next_id += 1;
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tilemap::Side::*;

    #[test]
    fn test_empty() {
        let size = Size::new(2, 3);
        let map = FurnitureMap2d::empty(size);

        assert_eq!(map.get_furniture(), &HashMap::new());
        assert_eq!(map.get_size(), &Size::new(4, 6));
    }

    #[test]
    fn test_is_border() {
        let size = Size::new(2, 3);
        let map = FurnitureMap2d::empty(size);
        let cell = Point::new(2, 4);

        assert!(map.is_border(cell, Top));
        assert!(map.is_border(cell, Left));
        assert!(!map.is_border(cell, Bottom));
        assert!(!map.is_border(cell, Right));

        let cell = Point::new(2, 5);

        assert!(!map.is_border(cell, Top));
        assert!(map.is_border(cell, Left));
        assert!(map.is_border(cell, Bottom));
        assert!(!map.is_border(cell, Right));

        let cell = Point::new(3, 5);

        assert!(!map.is_border(cell, Top));
        assert!(!map.is_border(cell, Left));
        assert!(map.is_border(cell, Bottom));
        assert!(map.is_border(cell, Right));

        let cell = Point::new(3, 4);

        assert!(map.is_border(cell, Top));
        assert!(!map.is_border(cell, Left));
        assert!(!map.is_border(cell, Bottom));
        assert!(map.is_border(cell, Right));
    }
}
