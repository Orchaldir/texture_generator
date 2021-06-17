use crate::resources::ResourceInfo;
use iced_native::image;
use texture_generation::math::color::convert_bgra;
use tilemap::rendering::Renderer;
use tilemap::tilemap::furniture::map2d::FurnitureMap2d;
use tilemap::tilemap::selector::Selector;
use tilemap::tilemap::tilemap2d::Tilemap2d;
use tilemap_io::tilemap::furniture::map2d::{save_furniture_map, FURNITURE_MAP_FILE_ENDING};
use tilemap_io::tilemap::{save_tilemap, TILEMAP_FILE_ENDING};

pub struct EditorData {
    pub resource_info: ResourceInfo,
    pub renderer: Renderer,
    pub preview_renderer: Renderer,
    pub tilemap: Tilemap2d,
    pub furniture_map: FurnitureMap2d,
    pub selector: Selector,
}

impl EditorData {
    pub fn new(resource_info: ResourceInfo) -> Self {
        let (renderer, preview_renderer) = resource_info.create_renderers();
        let (tilemap, furniture_map) = resource_info.load_maps().unwrap();
        let selector = Selector::new(preview_renderer.get_tile_size());

        EditorData {
            resource_info,
            renderer,
            preview_renderer,
            tilemap,
            furniture_map,
            selector,
        }
    }

    pub fn render_preview(&self) -> image::Handle {
        let texture = self
            .preview_renderer
            .render(&self.tilemap, Some(&self.furniture_map));
        let rbg = convert_bgra(texture.get_color_data());
        let size = texture.get_size();
        image::Handle::from_pixels(size.width(), size.height(), rbg)
    }

    pub fn reload_resources(&mut self) {
        let (renderer, preview_renderer) = self.resource_info.create_renderers();
        self.renderer = renderer;
        self.preview_renderer = preview_renderer;
    }

    pub fn load_maps(&mut self) -> bool {
        info!("Load the tilemap & furniture map");

        match self.resource_info.load_maps() {
            Ok((tilemap, furniture_map)) => {
                self.tilemap = tilemap;
                self.furniture_map = furniture_map;
                info!("Finished loading");
                true
            }
            Err(error) => {
                eprintln!("Error: {:?}", error);
                false
            }
        }
    }

    pub fn save_maps(&self) {
        info!("Save the tilemap & furniture map");

        let map_path = self.resource_info.get_map_path();
        let tilemap_path = map_path.with_extension(TILEMAP_FILE_ENDING);
        let furniture_map_path = map_path.with_extension(FURNITURE_MAP_FILE_ENDING);

        save_tilemap(&self.tilemap, &tilemap_path).unwrap();
        save_furniture_map(&self.furniture_map, &furniture_map_path);

        info!("Finished saving");
    }
}
