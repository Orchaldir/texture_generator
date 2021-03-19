use crate::definition::generation::component::rendering::RenderingDefinition;
use crate::generation::component::Component;
use crate::utils::error::GenerationError;
use layout::LayoutDefinition;
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};

pub mod layout;
pub mod rendering;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ComponentDefinition {
    Layout(Box<LayoutDefinition>),
    Rendering(Box<RenderingDefinition>),
}

impl TryFrom<ComponentDefinition> for Component {
    type Error = GenerationError;

    fn try_from(definition: ComponentDefinition) -> Result<Self, Self::Error> {
        match definition {
            ComponentDefinition::Layout(definition) => {
                Ok(Component::Layout(Box::new((*definition).try_into()?)))
            }
            ComponentDefinition::Rendering(definition) => {
                Ok(Component::Rendering(Box::new((*definition).try_into()?)))
            }
        }
    }
}

impl From<&Component> for ComponentDefinition {
    fn from(component: &Component) -> Self {
        match component {
            Component::Layout(component) => {
                let definition: LayoutDefinition = (&(**component)).into();
                ComponentDefinition::Layout(Box::new(definition))
            }
            Component::Rendering(component) => {
                let definition: RenderingDefinition = (&(**component)).into();
                ComponentDefinition::Rendering(Box::new(definition))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definition::generation::component::rendering::depth::DepthDefinition;
    use crate::definition::math::shape::ShapeDefinition;
    use crate::math::color::RED;
    use std::convert::TryInto;

    const SHAPE: ShapeDefinition = ShapeDefinition::Circle(42);

    #[test]
    fn test_convert_layout() {
        let depth = DepthDefinition::Uniform(42);
        let rendering = RenderingDefinition::Shape {
            name: "brick".to_string(),
            shape: SHAPE,
            color: RED,
            depth,
        };
        let layout = LayoutDefinition::Square {
            name: "test".to_string(),
            size: 10,
            component: ComponentDefinition::Rendering(Box::new(rendering)),
        };

        assert_convert(ComponentDefinition::Layout(Box::new(layout)));
    }

    #[test]
    fn test_convert_rendering() {
        let depth = DepthDefinition::Uniform(77);
        let rendering = RenderingDefinition::Shape {
            name: "brick".to_string(),
            shape: SHAPE,
            color: RED,
            depth,
        };
        assert_convert(ComponentDefinition::Rendering(Box::new(rendering)));
    }

    fn assert_convert(definition: ComponentDefinition) {
        let shape: Component = definition.clone().try_into().unwrap();
        let result: ComponentDefinition = (&shape).into();

        assert_eq!(result, definition)
    }
}
