use crate::definition::convert;
use crate::definition::generation::component::ComponentDefinition;
use crate::generation::component::layout::LayoutComponent;
use crate::utils::error::GenerationError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum LayoutDefinition {
    Square {
        name: String,
        side: u32,
        component: ComponentDefinition,
    },
}

impl LayoutDefinition {
    pub fn convert(&self, factor: f32) -> Result<LayoutComponent, GenerationError> {
        match self {
            LayoutDefinition::Square {
                name,
                side,
                component,
            } => match component.convert(factor) {
                Ok(component) => {
                    LayoutComponent::new_square(name, convert(*side, factor), component)
                }
                Err(error) => Err(error),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generation::component::Component;

    #[test]
    fn test_convert_square() {
        let definition = LayoutDefinition::Square {
            name: "test".to_string(),
            side: 10,
            component: ComponentDefinition::Mock(66),
        };
        let component = LayoutComponent::new_square("test", 25, Component::Mock(66)).unwrap();

        assert_eq!(component, definition.convert(2.5).unwrap())
    }
}
