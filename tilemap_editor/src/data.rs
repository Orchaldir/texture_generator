use crate::resources::ResourceInfo;
use iced_native::image;
use texture_generation::math::color::convert_bgra;
use tilemap::rendering::Renderer;
use tilemap::tilemap::furniture::map2d::FurnitureMap2d;
use tilemap::tilemap::selector::Selector;
use tilemap::tilemap::tilemap2d::Tilemap2d;

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
        let (tilemap, furniture_map) = resource_info.load_tilemap();
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
}
