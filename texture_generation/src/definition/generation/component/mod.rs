use crate::definition::generation::component::border::BorderDefinition;
use crate::definition::generation::component::rendering::RenderingDefinition;
use crate::generation::component::Component;
use anyhow::Result;
use layout::LayoutDefinition;
use serde::{Deserialize, Serialize};

pub mod border;
pub mod layout;
pub mod rendering;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ComponentDefinition {
    Border(Box<BorderDefinition>),
    Empty,
    Layers(Vec<ComponentDefinition>),
    Layout(Box<LayoutDefinition>),
    Mock(u8),
    Rendering(Box<RenderingDefinition>),
}

impl ComponentDefinition {
    pub fn convert(&self, parent: &str, factor: f32) -> Result<Component> {
        match self {
            ComponentDefinition::Border(definition) => Ok(Component::Border(Box::new(
                definition.convert(parent, factor)?,
            ))),
            ComponentDefinition::Empty => Ok(Component::Empty),
            ComponentDefinition::Layers(layers) => {
                let mut converted_layers = Vec::with_capacity(layers.len());

                for (i, definition) in layers.iter().enumerate() {
                    let component = definition.convert(
                        &format!("{}.Layers.{}|{}.", parent, i + 1, layers.len()),
                        factor,
                    )?;
                    converted_layers.push(component);
                }
                Ok(Component::Layers(converted_layers))
            }
            ComponentDefinition::Layout(definition) => Ok(Component::Layout(Box::new(
                definition.convert(parent, factor)?,
            ))),
            ComponentDefinition::Mock(id) => Ok(Component::Mock(*id)),
            ComponentDefinition::Rendering(definition) => Ok(Component::Rendering(Box::new(
                definition.convert(parent, factor)?,
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_layers() {
        let definition = ComponentDefinition::Layers(vec![
            ComponentDefinition::Mock(1),
            ComponentDefinition::Mock(2),
        ]);
        let component = Component::Layers(vec![Component::Mock(1), Component::Mock(2)]);

        assert_eq!(component, definition.convert("test", 2.0).unwrap())
    }
}
