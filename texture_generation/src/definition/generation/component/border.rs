use crate::definition::convert;
use crate::definition::generation::component::ComponentDefinition;
use crate::generation::component::border::shrink::ShrinkAxis;
use crate::generation::component::border::BorderComponent;
use crate::generation::random::Random;
use anyhow::{Context, Result};
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
    pub fn convert(&self, parent: &str, factor: f32) -> Result<BorderComponent> {
        match self {
            BorderDefinition::UniformBorder { border, component } => {
                let component =
                    component.convert(&format!("{}.UniformBorder.component", parent), factor)?;
                let border = BorderComponent::new_uniform(convert(*border, factor), component);
                Ok(border)
            }
            BorderDefinition::ShrinkAxis {
                is_horizontal,
                min_border,
                max_border,
                component,
            } => {
                let component =
                    component.convert(&format!("{}.ShrinkAxis.component", parent), factor)?;
                let border = ShrinkAxis::new_random(
                    *is_horizontal,
                    convert(*min_border, factor),
                    convert(*max_border, factor),
                    component,
                    Random::Hash,
                )
                .context(format!("Failed to create '{}.ShrinkAxis'", parent))?;
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

        assert_eq!(component, definition.convert("test", 2.0).unwrap())
    }

    #[test]
    fn test_shrink_axis() {
        let definition = BorderDefinition::ShrinkAxis {
            is_horizontal: false,
            min_border: 5,
            max_border: 20,
            component: ComponentDefinition::Mock(42),
        };
        let shrink = ShrinkAxis::new_random(false, 10, 40, Component::Mock(42), Random::Hash);
        let component = BorderComponent::ShrinkAxis(shrink.unwrap());

        assert_eq!(component, definition.convert("test", 2.0).unwrap())
    }
}
