use texture_generation::generation::component::Component;

pub struct Surface {
    name: String,
    height: u32,
    component: Component,
}

impl Surface {
    pub fn new<S: Into<String>>(name: S, height: u32, component: Component) -> Surface {
        Surface {
            name: name.into(),
            height,
            component,
        }
    }
}

#[derive(Default)]
pub struct SurfaceManager {
    surfaces: Vec<Surface>,
}

impl SurfaceManager {
    pub fn new(surfaces: Vec<Surface>) -> SurfaceManager {
        SurfaceManager { surfaces }
    }

    pub fn get(&self, id: usize) -> Option<&Surface> {
        self.surfaces.get(id)
    }
}
