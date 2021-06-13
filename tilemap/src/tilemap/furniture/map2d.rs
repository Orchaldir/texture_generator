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
            next_id: 0,
            furniture: HashMap::new(),
        }
    }

    pub fn get_all_furniture(&self) -> &HashMap<usize, Furniture> {
        &self.furniture
    }

    pub fn get_furniture(&self, id: usize) -> Option<&Furniture> {
        self.furniture.get(&id)
    }

    pub fn get_furniture_mut(&mut self, id: usize) -> Option<&mut Furniture> {
        self.furniture.get_mut(&id)
    }

    pub fn remove_furniture(&mut self, id: usize) -> bool {
        self.furniture.remove(&id).is_some()
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

    const FURNITURE0: Furniture = Furniture::new(0, 0, Size::square(1), Top);
    const FURNITURE1: Furniture = Furniture::new(1, 1, Size::square(2), Left);

    #[test]
    fn test_empty() {
        let size = Size::new(2, 3);
        let map = FurnitureMap2d::empty(size);

        assert_eq!(map.get_all_furniture(), &HashMap::new());
        assert_eq!(map.get_size(), &Size::new(4, 6));
    }

    #[test]
    fn test_add_and_get_furniture() {
        let size = Size::new(2, 3);
        let mut map = FurnitureMap2d::empty(size);
        map.add(FURNITURE0.clone());
        map.add(FURNITURE1.clone());

        assert_eq!(map.get_all_furniture().len(), 2);
        assert_eq!(map.get_furniture(0), Some(&FURNITURE0));
        assert_eq!(map.get_furniture(1), Some(&FURNITURE1));
    }

    #[test]
    fn test_remove_furniture() {
        let size = Size::new(2, 3);
        let mut map = FurnitureMap2d::empty(size);
        map.add(FURNITURE0.clone());
        map.add(FURNITURE1.clone());

        assert!(map.remove_furniture(0));

        assert_eq!(map.get_all_furniture().len(), 1);
        assert_eq!(map.get_furniture(0), None);
        assert_eq!(map.get_furniture(1), Some(&FURNITURE1));
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
