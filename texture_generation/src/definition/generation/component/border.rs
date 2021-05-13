use crate::definition::convert;
use crate::definition::generation::component::ComponentDefinition;
use crate::generation::component::border::shrink::ShrinkAxis;
use crate::generation::component::border::BorderComponent;
use crate::generation::random::Random;
use crate::utils::error::DefinitionError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum BorderDefinition {
    UniformBorder {
        border: u32,
        component: ComponentDefinition,
    },
    ShrinkAxis {
        is_horizontal: bool,
        min_border: u32,
        max_border: u32,
        component: ComponentDefinition,
    },
}

impl BorderDefinition {
    pub fn convert(&self, factor: f32) -> Result<BorderComponent, DefinitionError> {
        match self {
            BorderDefinition::UniformBorder { border, component } => {
                let component = component.convert(factor)?;
                let border = BorderComponent::new_uniform(convert(*border, factor), component);
                Ok(border)
            }
            BorderDefinition::ShrinkAxis {
                is_horizontal,
                min_border,
                max_border,
                component,
            } => {
                let component = component.convert(factor)?;
                let border = ShrinkAxis::new_random(
                    *is_horizontal,
                    convert(*min_border, factor),
                    convert(*max_border, factor),
                    component,
                    Random::Hash,
                );
                Ok(BorderComponent::ShrinkAxis(border))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generation::component::Component;

    #[test]
    fn test_convert_uniform() {
        let definition = BorderDefinition::UniformBorder {
            border: 10,
            component: ComponentDefinition::Mock(66),
        };
        let component = BorderComponent::new_uniform(20, Component::Mock(66));

        assert_eq!(component, definition.convert(2.0).unwrap())
    }
}
