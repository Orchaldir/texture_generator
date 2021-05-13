use crate::definition::generation::component::ComponentDefinition;
use crate::definition::{convert, convert_size};
use crate::generation::component::layout::brick::BrickPattern;
use crate::generation::component::layout::herringbone::HerringbonePattern;
use crate::generation::component::layout::random_ashlar::RandomAshlarPattern;
use crate::generation::component::layout::repeat::RepeatLayout;
use crate::generation::component::layout::split::SplitLayout;
use crate::generation::component::layout::LayoutComponent;
use crate::generation::random::Random;
use crate::math::size::Size;
use anyhow::Result;
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
    RandomRepeatX {
        min_size: u32,
        max_size: u32,
        component: ComponentDefinition,
    },
    RandomRepeatY {
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
    Split {
        is_horizontal: bool,
        components: Vec<(u32, ComponentDefinition)>,
    },
}

impl LayoutDefinition {
    pub fn convert(&self, parent: &str, factor: f32) -> Result<LayoutComponent> {
        match self {
            LayoutDefinition::BrickWall {
                name,
                brick,
                offset,
                component,
            } => {
                let component =
                    component.convert(&format!("{}.BrickWall.component", parent), factor)?;
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
                let horizontal_component = horizontal_component.convert(
                    &format!("{}.Herringbone.horizontal_component", parent),
                    factor,
                )?;
                let vertical_component = vertical_component.convert(
                    &format!("{}.Herringbone.vertical_component", parent),
                    factor,
                )?;
                let pattern = HerringbonePattern::new(
                    convert(*side, factor),
                    *multiplier,
                    horizontal_component,
                    vertical_component,
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
                    component.convert(&format!("{}.RandomAshlar.component", parent), factor)?,
                );
                Ok(LayoutComponent::RandomAshlar(pattern))
            }
            LayoutDefinition::RandomRepeatX {
                min_size,
                max_size,
                component,
            } => {
                let component =
                    component.convert(&format!("{}.RandomRepeatX.component", parent), factor)?;
                let layout = RepeatLayout::new_random(
                    true,
                    convert(*min_size, factor),
                    convert(*max_size, factor),
                    component,
                    Random::Hash,
                );
                Ok(LayoutComponent::Repeat(layout))
            }
            LayoutDefinition::RandomRepeatY {
                min_size,
                max_size,
                component,
            } => {
                let component =
                    component.convert(&format!("{}.RandomRepeatY.component", parent), factor)?;
                let layout = RepeatLayout::new_random(
                    false,
                    convert(*min_size, factor),
                    convert(*max_size, factor),
                    component,
                    Random::Hash,
                );
                Ok(LayoutComponent::Repeat(layout))
            }
            LayoutDefinition::RepeatX { size, component } => {
                let component =
                    component.convert(&format!("{}.RepeatX.component", parent), factor)?;
                let layout = RepeatLayout::new(true, convert(*size, factor), component);
                Ok(LayoutComponent::Repeat(layout))
            }
            LayoutDefinition::RepeatY { size, component } => {
                let component =
                    component.convert(&format!("{}.RepeatY.component", parent), factor)?;
                let layout = RepeatLayout::new(false, convert(*size, factor), component);
                Ok(LayoutComponent::Repeat(layout))
            }
            LayoutDefinition::Square {
                name,
                side,
                component,
            } => {
                let component =
                    component.convert(&format!("{}.Square.component", parent), factor)?;
                let pattern = BrickPattern::new_square(name, convert(*side, factor), component)?;
                Ok(LayoutComponent::BrickWall(pattern))
            }
            LayoutDefinition::Split {
                is_horizontal,
                components,
            } => {
                let mut converted_components = Vec::with_capacity(components.len());

                for (i, (value, component)) in components.iter().enumerate() {
                    let component = component.convert(
                        &format!("{}.Split.component.{}|{}.", parent, i + 1, components.len()),
                        factor,
                    )?;
                    converted_components.push((*value, component));
                }

                let pattern = SplitLayout::new(*is_horizontal, converted_components);
                Ok(LayoutComponent::Split(pattern))
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
        let component = LayoutComponent::BrickWall(
            BrickPattern::new("test", Size::new(40, 20), 20, Component::Mock(66)).unwrap(),
        );

        assert_eq!(component, definition.convert("test", 2.0).unwrap())
    }

    #[test]
    fn test_convert_repeat_x() {
        let definition = LayoutDefinition::RepeatX {
            size: 20,
            component: ComponentDefinition::Mock(88),
        };
        let component = LayoutComponent::Repeat(RepeatLayout::new(true, 30, Component::Mock(88)));

        assert_eq!(component, definition.convert("test", 1.5).unwrap())
    }

    #[test]
    fn test_convert_repeat_y() {
        let definition = LayoutDefinition::RepeatY {
            size: 50,
            component: ComponentDefinition::Mock(11),
        };
        let component = LayoutComponent::Repeat(RepeatLayout::new(false, 75, Component::Mock(11)));

        assert_eq!(component, definition.convert("test", 1.5).unwrap())
    }

    #[test]
    fn test_convert_square() {
        let definition = LayoutDefinition::Square {
            name: "square".to_string(),
            side: 10,
            component: ComponentDefinition::Mock(66),
        };
        let component = LayoutComponent::BrickWall(
            BrickPattern::new("square", Size::square(25), 0, Component::Mock(66)).unwrap(),
        );

        assert_eq!(component, definition.convert("test", 2.5).unwrap())
    }

    #[test]
    fn test_convert_split() {
        let definition = LayoutDefinition::Split {
            is_horizontal: true,
            components: vec![
                (4, ComponentDefinition::Mock(11)),
                (6, ComponentDefinition::Mock(45)),
            ],
        };
        let component = LayoutComponent::Split(SplitLayout::new(
            true,
            vec![(4, Component::Mock(11)), (6, Component::Mock(45))],
        ));

        assert_eq!(component, definition.convert("test", 2.0).unwrap())
    }
}
