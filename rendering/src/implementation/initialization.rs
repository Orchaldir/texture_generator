use crate::implementation::renderer::GliumRenderer;
use crate::implementation::texture::load_texture;
use crate::interface::rendering::Initialization;
use crate::interface::{Size2d, TextureId};

pub struct GliumInitialization {
    display: glium::Display,
    textures: Vec<glium::texture::Texture2d>,
}

impl GliumInitialization {
    pub fn new(display: glium::Display) -> GliumInitialization {
        GliumInitialization {
            display,
            textures: Vec::new(),
        }
    }

    pub fn finish(self, size: Size2d) -> GliumRenderer {
        GliumRenderer::new(self.display, self.textures, size)
    }
}

impl Initialization for GliumInitialization {
    fn load_texture(&mut self, filename: &str) -> TextureId {
        let texture = load_texture(&self.display, filename).unwrap();
        let id = self.textures.len();

        self.textures.push(texture);

        id
    }
}
