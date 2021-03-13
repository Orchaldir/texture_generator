use crate::generation::component::Component;

pub mod component;
pub mod data;
pub mod layout;
pub mod rendering;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextureGenerator {
    pub name: String,
    pub component: Component,
}

impl TextureGenerator {
    pub fn new<S: Into<String>>(name: S, component: Component) -> TextureGenerator {
        TextureGenerator {
            name: name.into(),
            component,
        }
    }
}
