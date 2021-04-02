use texture_generation::generation::component::Component;
use texture_generation::generation::data::Data;
use texture_generation::math::aabb::AABB;

pub struct Texture {
    name: String,
    component: Component,
}

impl Texture {
    pub fn new<S: Into<String>>(name: S, component: Component) -> Texture {
        Texture {
            name: name.into(),
            component,
        }
    }

    pub fn render(&self, data: &mut dyn Data, aabb: &AABB) {
        self.component.generate(data, aabb);
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
