use crate::tilemap::furniture::Furniture;
use anyhow::{bail, Result};
use std::collections::HashMap;
use texture_generation::math::point::Point;
use texture_generation::math::side::Side;
use texture_generation::math::size::Size;

const RESOLUTION: u32 = 2;

#[derive(Clone, Debug, PartialEq)]
pub struct FurnitureMap2d {
    size: Size,
    /// The id of the next [`Furniture`].
    next_id: usize,
    furniture: HashMap<usize, Furniture>,
}

impl FurnitureMap2d {
    pub fn new(size: Size, furniture: HashMap<usize, Furniture>) -> Result<Self> {
        if size.width() == 0 {
            bail!("Argument 'size.width' needs to be greater than 0");
        } else if size.height() == 0 {
            bail!("Argument 'size.height' needs to be greater than 0");
        }

        Ok(FurnitureMap2d {
            size,
            next_id: furniture.iter().map(|e| *e.0 + 1).max().unwrap_or_default(),
            furniture,
        })
    }

    pub fn empty(tilemap_size: Size) -> Self {
        let size = tilemap_size * RESOLUTION as f32;

        FurnitureMap2d {
            size,
            next_id: 0,
            furniture: HashMap::new(),
        }
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

    pub fn add(&mut self, furniture: Furniture) -> Option<usize> {
        if !self.size.is_aabb_inside(&furniture.aabb) {
            return None;
        }

        let id = self.next_id;
        self.furniture.insert(id, furniture);
        self.next_id += 1;
        Some(id)
    }

    pub fn get_all_furniture(&self) -> &HashMap<usize, Furniture> {
        &self.furniture
    }

    pub fn get_furniture(&self, id: usize) -> Option<&Furniture> {
        self.furniture.get(&id)
    }

    pub fn get_id_at(&self, position: usize) -> Option<usize> {
        let point = self.size.to_point(position);
        for (id, furniture) in &self.furniture {
            if furniture.aabb.is_inside(&point) {
                return Some(*id);
            }
        }

        None
    }

    pub fn remove_furniture(&mut self, id: usize) -> bool {
        self.furniture.remove(&id).is_some()
    }

    /// Updates the existing ['Furniture'] with a specific id, if the new one fits into the map.
    /// Also fails if the furniture didn't change.
    pub fn update_furniture(&mut self, id: usize, furniture: Furniture) -> bool {
        if !self.size.is_aabb_inside(&furniture.aabb) || !self.can_update(id, &furniture) {
            return false;
        }

        self.furniture.insert(id, furniture);
        true
    }

    fn can_update(&self, id: usize, furniture: &Furniture) -> bool {
        if let Some(old_furniture) = self.furniture.get(&id) {
            !furniture.eq(old_furniture)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use texture_generation::math::side::Side::*;

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

        assert_eq!(map.add(furniture0()), Some(0));
        assert_eq!(map.add(furniture1()), Some(1));

        assert_eq!(map.get_all_furniture().len(), 2);
        assert_eq!(map.get_furniture(0), Some(&furniture0()));
        assert_eq!(map.get_furniture(1), Some(&furniture1()));
    }

    #[test]
    fn test_add_furniture_with_start_outside() {
        let size = Size::new(2, 3);
        let mut map = FurnitureMap2d::empty(size);

        assert_eq!(
            map.add(Furniture::new(0, Point::new(-1, 0), Size::square(1), Top).unwrap()),
            None
        );

        assert!(map.get_all_furniture().is_empty());
    }

    #[test]
    fn test_add_furniture_with_end_outside() {
        let size = Size::new(2, 3);
        let mut map = FurnitureMap2d::empty(size);

        assert_eq!(
            map.add(Furniture::new(0, Point::new(4, 2), Size::square(4), Top).unwrap()),
            None
        );

        assert!(map.get_all_furniture().is_empty());
    }

    #[test]
    fn test_remove_furniture() {
        let size = Size::new(2, 3);
        let mut map = FurnitureMap2d::empty(size);

        assert_eq!(map.add(furniture0()), Some(0));
        assert_eq!(map.add(furniture1()), Some(1));

        assert!(map.remove_furniture(0));

        assert_eq!(map.get_all_furniture().len(), 1);
        assert_eq!(map.get_furniture(0), None);
        assert_eq!(map.get_furniture(1), Some(&furniture1()));
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

    fn furniture0() -> Furniture {
        Furniture::new(0, Point::new(0, 0), Size::square(1), Top).unwrap()
    }

    fn furniture1() -> Furniture {
        Furniture::new(1, Point::new(1, 0), Size::square(2), Left).unwrap()
    }
}
