use crate::tilemap::furniture::map2d::FurnitureMap2d;
use crate::tilemap::tilemap2d::Tilemap2d;
use crate::tilemap::Side;
use crate::tilemap::Side::*;
use texture_generation::math::point::Point;

/// Transform coordinates into indices of a [`Tilemap2d`].
pub struct Selector {
    tile_size: u32,
}

impl Selector {
    pub fn new(tile_size: u32) -> Self {
        Selector { tile_size }
    }

    pub fn get_tile_size(&self) -> u32 {
        self.tile_size
    }

    /// Returns the tile of the coordinates for the [`Tilemap2d`].
    pub fn get_tile_index(&self, tilemap: &Tilemap2d, point: Point) -> Option<usize> {
        if point.x < 0 || point.y < 0 {
            return None;
        }

        let tile = point / self.tile_size;
        tilemap.get_size().to_index(&tile)
    }

    /// Returns the tile of the coordinates for the [`FurnitureMap2d`].
    pub fn get_furniture_tile_index(&self, map: &FurnitureMap2d, point: Point) -> Option<usize> {
        if point.x < 0 || point.y < 0 {
            return None;
        }

        let furniture_tile_size = map.convert_from_tile_size(self.tile_size);
        let tile = point / furniture_tile_size;
        map.get_size().to_index(&tile)
    }

    /// Returns which [`Side`] of a tile the coordinates are inside or None for its center or corners.
    pub fn get_side(&self, tilemap: &Tilemap2d, point: Point, tile_index: usize) -> Option<Side> {
        let tile_size = self.tile_size;
        let start = tilemap.get_size().to_point(tile_index) * tile_size;
        let local = point - start;
        let x = local.x as f32 / tile_size as f32;
        let y = local.y as f32 / tile_size as f32;
        let border = 0.1;
        let is_top = y < border;
        let is_left = x < border;
        let is_bottom = y > (1.0 - border);
        let is_right = x > (1.0 - border);

        Some(if is_top && !is_left && !is_right {
            Top
        } else if is_left && !is_top && !is_bottom {
            Left
        } else if is_bottom && !is_left && !is_right {
            Bottom
        } else if is_right && !is_top && !is_bottom {
            Right
        } else {
            return None;
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tilemap::tile::Tile;
    use texture_generation::math::size::Size;

    #[test]
    fn test_get_tile_index() {
        let selector = Selector::new(100);
        let tilemap = Tilemap2d::default(Size::new(2, 3), Tile::Empty);

        assert_index(&selector, &tilemap, 50, 50, 0);
        assert_index(&selector, &tilemap, 150, 50, 1);
        assert_index(&selector, &tilemap, 50, 150, 2);
        assert_index(&selector, &tilemap, 150, 150, 3);
        assert_index(&selector, &tilemap, 50, 250, 4);
        assert_index(&selector, &tilemap, 150, 250, 5);
    }

    #[test]
    fn test_get_tile_index_outside() {
        let selector = Selector::new(100);
        let tilemap = Tilemap2d::default(Size::new(2, 3), Tile::Empty);

        assert_outside(&selector, &tilemap, -50, 50);
        assert_outside(&selector, &tilemap, 50, -50);
        assert_outside(&selector, &tilemap, 250, 50);
        assert_outside(&selector, &tilemap, 50, 350);
    }

    #[test]
    fn test_get_furniture_tile_index() {
        let selector = Selector::new(100);
        let map = FurnitureMap2d::empty(Size::new(2, 3));
        let mut index = 0;

        for y in 0..4 {
            for x in 0..4 {
                assert_furniture(&selector, &map, 25 + 50 * x, 25 + 50 * y, index);
                index += 1;
            }
        }
    }

    #[test]
    fn test_get_furniture_tile_index_outside() {
        let selector = Selector::new(100);
        let map = FurnitureMap2d::empty(Size::new(2, 3));

        assert_furniture_outside(&selector, &map, -50, 50);
        assert_furniture_outside(&selector, &map, 50, -50);
        assert_furniture_outside(&selector, &map, 250, 50);
        assert_furniture_outside(&selector, &map, 50, 350);
    }

    #[test]
    fn test_get_side() {
        let selector = Selector::new(100);
        let tilemap = Tilemap2d::default(Size::new(2, 3), Tile::Empty);

        assert_side(&selector, &tilemap, 50, 150, 2, None);
        assert_side(&selector, &tilemap, 50, 105, 2, Some(Top));
        assert_side(&selector, &tilemap, 5, 150, 2, Some(Left));
        assert_side(&selector, &tilemap, 50, 195, 2, Some(Bottom));
        assert_side(&selector, &tilemap, 95, 150, 2, Some(Right));
    }

    fn assert_index(selector: &Selector, tilemap: &Tilemap2d, x: i32, y: i32, index: usize) {
        assert_eq!(
            selector.get_tile_index(tilemap, Point::new(x, y)),
            Some(index)
        );
    }

    fn assert_furniture(selector: &Selector, map: &FurnitureMap2d, x: i32, y: i32, index: usize) {
        assert_eq!(
            selector.get_furniture_tile_index(map, Point::new(x, y)),
            Some(index)
        );
    }

    fn assert_outside(selector: &Selector, tilemap: &Tilemap2d, x: i32, y: i32) {
        assert_eq!(selector.get_tile_index(tilemap, Point::new(x, y)), None);
    }

    fn assert_furniture_outside(selector: &Selector, map: &FurnitureMap2d, x: i32, y: i32) {
        assert_eq!(
            selector.get_furniture_tile_index(map, Point::new(x, y)),
            None
        );
    }

    fn assert_side(
        selector: &Selector,
        tilemap: &Tilemap2d,
        x: i32,
        y: i32,
        index: usize,
        result: Option<Side>,
    ) {
        assert_eq!(selector.get_side(tilemap, Point::new(x, y), index), result);
    }
}
