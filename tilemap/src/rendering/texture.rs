use texture_generation::generation::component::Component;

pub struct Texture {
    name: String,
    height: u32,
    component: Component,
}

impl Texture {
    pub fn new<S: Into<String>>(name: S, height: u32, component: Component) -> Texture {
        Texture {
            name: name.into(),
            height,
            component,
        }
    }
}

#[derive(Default)]
pub struct TextureManager {
    surfaces: Vec<Texture>,
}

impl TextureManager {
    pub fn new(surfaces: Vec<Texture>) -> TextureManager {
        TextureManager { surfaces }
    }

    pub fn get(&self, id: usize) -> Option<&Texture> {
        self.surfaces.get(id)
    }
}
