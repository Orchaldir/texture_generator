use crate::definition::generation::component::ComponentDefinition;
use crate::definition::{convert, convert_size};
use crate::generation::component::layout::LayoutComponent;
use crate::math::size::Size;
use crate::utils::error::GenerationError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum LayoutDefinition {
    BrickWall {
        name: String,
        brick: Size,
        offset: u32,
        component: ComponentDefinition,
    },
    Square {
        name: String,
        side: u32,
        component: ComponentDefinition,
    },
}

impl LayoutDefinition {
    pub fn convert(&self, factor: f32) -> Result<LayoutComponent, GenerationError> {
        match self {
            LayoutDefinition::BrickWall {
                name,
                brick,
                offset,
                component,
            } => match component.convert(factor) {
                Ok(component) => LayoutComponent::new_brick_wall(
                    name,
                    convert_size(brick, factor),
                    convert(*offset, factor),
                    component,
                ),
                Err(error) => Err(error),
            },
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
    fn test_convert_brick_wall() {
        let definition = LayoutDefinition::BrickWall {
            name: "test".to_string(),
            brick: Size::new(20, 10),
            offset: 10,
            component: ComponentDefinition::Mock(66),
        };
        let component =
            LayoutComponent::new_brick_wall("test", Size::new(40, 20), 20, Component::Mock(66))
                .unwrap();

        assert_eq!(component, definition.convert(2.0).unwrap())
    }

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
