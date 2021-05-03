use crate::definition::generation::component::ComponentDefinition;
use crate::definition::{convert, convert_size};
use crate::generation::component::layout::brick::BrickPattern;
use crate::generation::component::layout::herringbone::HerringbonePattern;
use crate::generation::component::layout::random_ashlar::RandomAshlarPattern;
use crate::generation::component::layout::LayoutComponent;
use crate::math::size::Size;
use crate::utils::error::DefinitionError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LayoutDefinition {
    BrickWall {
        name: String,
        brick: Size,
        offset: u32,
        component: ComponentDefinition,
    },
    Herringbone {
        side: u32,
        multiplier: u32,
        horizontal_component: ComponentDefinition,
        vertical_component: ComponentDefinition,
    },
    Mock(u32),
    RandomAshlar {
        cells_per_side: u32,
        min_size: u32,
        max_size: u32,
        component: ComponentDefinition,
    },
    RepeatX {
        size: u32,
        component: ComponentDefinition,
    },
    RepeatY {
        size: u32,
        component: ComponentDefinition,
    },
    Square {
        name: String,
        side: u32,
        component: ComponentDefinition,
    },
}

impl LayoutDefinition {
    pub fn convert(&self, factor: f32) -> Result<LayoutComponent, DefinitionError> {
        match self {
            LayoutDefinition::BrickWall {
                name,
                brick,
                offset,
                component,
            } => {
                let component = component.convert(factor)?;
                let pattern = BrickPattern::new(
                    name,
                    convert_size(brick, factor),
                    convert(*offset, factor),
                    component,
                )?;
                Ok(LayoutComponent::BrickWall(pattern))
            }
            LayoutDefinition::Herringbone {
                side,
                multiplier,
                horizontal_component,
                vertical_component,
            } => {
                let pattern = HerringbonePattern::new(
                    convert(*side, factor),
                    *multiplier,
                    horizontal_component.convert(factor)?,
                    vertical_component.convert(factor)?,
                );
                Ok(LayoutComponent::Herringbone(pattern))
            }
            LayoutDefinition::Mock(id) => Ok(LayoutComponent::Mock(*id)),
            LayoutDefinition::RandomAshlar {
                cells_per_side,
                min_size,
                max_size,
                component,
            } => {
                let pattern = RandomAshlarPattern::new(
                    *cells_per_side,
                    *min_size,
                    *max_size,
                    component.convert(factor)?,
                );
                Ok(LayoutComponent::RandomAshlar(pattern))
            }
            LayoutDefinition::RepeatX { size, component } => {
                let component = component.convert(factor)?;
                let layout = LayoutComponent::new_repeat_x(convert(*size, factor), component)?;
                Ok(layout)
            }
            LayoutDefinition::RepeatY { size, component } => {
                let component = component.convert(factor)?;
                let layout = LayoutComponent::new_repeat_y(convert(*size, factor), component)?;
                Ok(layout)
            }
            LayoutDefinition::Square {
                name,
                side,
                component,
            } => {
                let component = component.convert(factor)?;
                let layout = LayoutComponent::new_square(name, convert(*side, factor), component)?;
                Ok(layout)
            }
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
    fn test_convert_repeat_x() {
        let definition = LayoutDefinition::RepeatX {
            size: 20,
            component: ComponentDefinition::Mock(88),
        };
        let component = LayoutComponent::new_repeat_x(30, Component::Mock(88)).unwrap();

        assert_eq!(component, definition.convert(1.5).unwrap())
    }

    #[test]
    fn test_convert_repeat_y() {
        let definition = LayoutDefinition::RepeatY {
            size: 50,
            component: ComponentDefinition::Mock(11),
        };
        let component = LayoutComponent::new_repeat_y(75, Component::Mock(11)).unwrap();

        assert_eq!(component, definition.convert(1.5).unwrap())
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
