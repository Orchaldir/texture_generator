use crate::tilemap::furniture::Furniture;
use std::collections::HashMap;
use texture_generation::math::size::Size;

const EMPTY: usize = 0;
const RESOLUTION: usize = 2;

pub struct FurnitureMap2d {
    size: Size,
    /// The id of the next [`Furniture`]. It starts at 1.
    next_id: usize,
    /// Is the cell free? Is 0 if free and otherwise is the id of the [`Furniture`] occupying the cell.
    cells: Vec<usize>,
    furniture: HashMap<usize, Furniture>,
}

impl FurnitureMap2d {
    pub fn empty(tilemap_size: Size) -> FurnitureMap2d {
        let size = tilemap_size * RESOLUTION as f32;
        let cells = vec![EMPTY; size.len()];

        FurnitureMap2d {
            size,
            next_id: 1,
            cells,
            furniture: HashMap::new(),
        }
    }

    pub fn get_furniture(&self) -> &HashMap<usize, Furniture> {
        &self.furniture
    }

    pub fn get_size(&self) -> &Size {
        &self.size
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

    #[test]
    fn test_empty() {
        let size = Size::new(2, 3);
        let map = FurnitureMap2d::empty(size);

        assert_eq!(map.get_furniture(), &HashMap::new());
        assert_eq!(map.get_size(), &Size::new(4, 6));
    }
}
