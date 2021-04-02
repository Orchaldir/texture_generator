use crate::rendering::texture::TextureManager;
use crate::tilemap::tile::Tile;
use crate::tilemap::tilemap2d::Tilemap2d;
use texture_generation::generation::data::{Data, RuntimeData};
use texture_generation::generation::process::PostProcess;
use texture_generation::math::aabb::AABB;
use texture_generation::math::color::PINK;
use texture_generation::math::point::Point;
use texture_generation::math::size::Size;

pub mod texture;

pub struct Renderer {
    tile_size: u32,
    textures: TextureManager,
    post_processes: Vec<PostProcess>,
}

impl Renderer {
    pub fn new(tile_size: u32, textures: TextureManager, post_processes: Vec<PostProcess>) -> Self {
        Renderer {
            tile_size,
            textures,
            post_processes,
        }
    }

    pub fn render(&self, tilemap: &Tilemap2d) -> RuntimeData {
        let tiles = tilemap.get_size();
        let tile_size = Size::square(self.tile_size);
        let size = tile_size * tiles;
        let mut start = Point::default();
        let mut data = RuntimeData::new(size, PINK);

        for y in 0..tiles.height() {
            start.x = 0;

            for x in 0..tiles.height() {
                let index = tiles.convert_x_y(x, y);
                let tile = tilemap.get_tile(index);
                let aabb = AABB::new(start, tile_size);

                match tile {
                    Tile::Empty => {}
                    Tile::Floor(id) => self.render_texture(index, id, &mut data, &aabb),
                    Tile::Full(id) => self.render_texture(index, id, &mut data, &aabb),
                }

                start.x = tile_size.width() as i32;
            }

            start.y = tile_size.height() as i32;
        }

        for post_process in self.post_processes.iter() {
            post_process.process(&mut data);
        }

        data
    }

    fn render_texture(&self, index: usize, texture_id: usize, data: &mut dyn Data, aabb: &AABB) {
        if let Some(texture) = self.textures.get(texture_id) {
            texture.render(data, aabb);
        } else {
            warn!(
                "Cannot render unknown texture '{}' for tile '{}'!",
                texture_id, index
            );
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::rendering::texture::Texture;
    use texture_generation::generation::component::rendering::RenderingComponent;
    use texture_generation::generation::component::Component;
    use texture_generation::generation::data::Data;
    use texture_generation::math::color::{Color, BLUE, RED};

    #[test]
    fn test_generate() {
        let texture0 = create_texture("texture0", RED, 99);
        let texture1 = create_texture("texture0", BLUE, 42);
        let textures = TextureManager::new(vec![texture0, texture1]);
        let renderer = Renderer::new(2, textures, Vec::default());
        let tiles = vec![Tile::Empty, Tile::Floor(0), Tile::Full(1), Tile::Empty];
        let tilemap = Tilemap2d::new(Size::square(2), tiles).unwrap();

        let data = renderer.render(&tilemap);

        #[rustfmt::skip]
        let result = vec![
            PINK, PINK,  RED,  RED,
            PINK, PINK,  RED,  RED,
            BLUE, BLUE, PINK, PINK,
            BLUE, BLUE, PINK, PINK,
        ];

        assert_eq!(data.get_color_data(), &result);

        #[rustfmt::skip]
        let depth = vec![
             0,  0, 99, 99,
             0,  0, 99, 99,
            42, 42,  0,  0,
            42, 42,  0,  0,
        ];

        assert_eq!(data.get_depth_data(), &depth);
    }

    fn create_texture(name: &str, color: Color, depth: u8) -> Texture {
        let rendering = RenderingComponent::new_fill_area(name, color, depth);
        let component = Component::Rendering(Box::new(rendering));
        Texture::new(name, component)
    }
}
