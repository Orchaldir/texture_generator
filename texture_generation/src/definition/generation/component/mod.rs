use crate::definition::generation::component::border::BorderDefinition;
use crate::definition::generation::component::rendering::RenderingDefinition;
use crate::generation::component::Component;
use crate::utils::error::DefinitionError;
use layout::LayoutDefinition;
use serde::{Deserialize, Serialize};

pub mod border;
pub mod layout;
pub mod rendering;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ComponentDefinition {
    Border(Box<BorderDefinition>),
    Layout(Box<LayoutDefinition>),
    Mock(u8),
    Rendering(Box<RenderingDefinition>),
}

impl ComponentDefinition {
    pub fn convert(&self, factor: f32) -> Result<Component, DefinitionError> {
        match self {
            ComponentDefinition::Border(definition) => {
                Ok(Component::Border(Box::new(definition.convert(factor)?)))
            }
            ComponentDefinition::Layout(definition) => {
                Ok(Component::Layout(Box::new(definition.convert(factor)?)))
            }
            ComponentDefinition::Mock(id) => Ok(Component::Mock(*id)),
            ComponentDefinition::Rendering(definition) => {
                Ok(Component::Rendering(Box::new(definition.convert(factor)?)))
            }
        }
    }
}
