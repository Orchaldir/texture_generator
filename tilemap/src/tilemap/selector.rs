use crate::tilemap::tilemap2d::Tilemap2d;
use crate::tilemap::Side;
use crate::tilemap::Side::*;

/// Renders a [`Tilemap2d`] in a specific style.
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

    pub fn get_tile_index(&self, tilemap: &Tilemap2d, x: u32, y: u32) -> usize {
        let tile_x = x / self.tile_size;
        let tile_y = y / self.tile_size;
        tilemap.get_size().convert_x_y(tile_x, tile_y)
    }

    pub fn get_side(&self, tilemap: &Tilemap2d, x: u32, y: u32, tile_index: usize) -> Option<Side> {
        let tile_size = self.tile_size;
        let start = tilemap.get_size().to_point(tile_index);
        let x = (x - start.x as u32 * tile_size) as f32 / tile_size as f32;
        let y = (y - start.y as u32 * tile_size) as f32 / tile_size as f32;
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
        let renderer = Selector::new(100);
        let tilemap = Tilemap2d::default(Size::new(2, 3), Tile::Empty);

        assert_eq!(renderer.get_tile_index(&tilemap, 50, 50), 0);
        assert_eq!(renderer.get_tile_index(&tilemap, 150, 50), 1);
        assert_eq!(renderer.get_tile_index(&tilemap, 50, 150), 2);
        assert_eq!(renderer.get_tile_index(&tilemap, 150, 150), 3);
        assert_eq!(renderer.get_tile_index(&tilemap, 50, 250), 4);
        assert_eq!(renderer.get_tile_index(&tilemap, 150, 250), 5);
    }

    #[test]
    fn test_get_side() {
        let renderer = Selector::new(100);
        let tilemap = Tilemap2d::default(Size::new(2, 3), Tile::Empty);

        assert_eq!(renderer.get_side(&tilemap, 50, 150, 2), None);
        assert_eq!(renderer.get_side(&tilemap, 50, 105, 2), Some(Top));
        assert_eq!(renderer.get_side(&tilemap, 5, 150, 2), Some(Left));
        assert_eq!(renderer.get_side(&tilemap, 50, 195, 2), Some(Bottom));
        assert_eq!(renderer.get_side(&tilemap, 95, 150, 2), Some(Right));
    }
}
