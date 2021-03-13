use crate::definition::generation::component::ComponentDefinition;
use crate::generation::layout::LayoutComponent;
use crate::utils::error::GenerationError;
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum LayoutDefinition {
    Square {
        name: String,
        size: u32,
        component: ComponentDefinition,
    },
}

impl TryFrom<LayoutDefinition> for LayoutComponent {
    type Error = GenerationError;

    fn try_from(definition: LayoutDefinition) -> Result<Self, Self::Error> {
        match definition {
            LayoutDefinition::Square {
                name,
                size,
                component,
            } => match component.try_into() {
                Ok(component) => LayoutComponent::new_square(name, size, component),
                Err(error) => Err(error),
            },
        }
    }
}

impl From<&LayoutComponent> for LayoutDefinition {
    fn from(layout: &LayoutComponent) -> Self {
        match layout {
            LayoutComponent::Square {
                name,
                size,
                component,
            } => LayoutDefinition::Square {
                name: name.clone(),
                size: *size,
                component: component.into(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definition::generation::rendering::RenderingDefinition;
    use crate::definition::math::shape::ShapeDefinition;
    use crate::math::color::RED;
    use std::convert::TryInto;

    #[test]
    fn test_convert_square() {
        let shape = ShapeDefinition::Circle(42);
        let rendering = Box::new(RenderingDefinition::Shape {
            name: "brick".to_string(),
            shape,
            color: RED,
        });
        let component = ComponentDefinition::Rendering(rendering);

        assert_convert(LayoutDefinition::Square {
            name: "test".to_string(),
            size: 10,
            component,
        });
    }

    fn assert_convert(definition: LayoutDefinition) {
        let shape: LayoutComponent = definition.clone().try_into().unwrap();
        let result: LayoutDefinition = (&shape).into();

        assert_eq!(result, definition)
    }
}
