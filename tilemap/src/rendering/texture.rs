use texture_generation::generation::TextureGenerator;

pub struct TextureManager {
    surfaces: Vec<TextureGenerator>,
}

impl TextureManager {
    pub fn new(surfaces: Vec<TextureGenerator>) -> TextureManager {
        TextureManager { surfaces }
    }

    pub fn get(&self, id: usize) -> Option<&TextureGenerator> {
        self.surfaces.get(id)
    }
}
