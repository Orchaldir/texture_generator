use crate::definition::generation::component::ComponentDefinition;
use crate::generation::layout::{LayoutComponent, LayoutError};
use std::convert::{TryFrom, TryInto};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LayoutDefinition {
    Square {
        size: u32,
        component: Box<ComponentDefinition>,
    },
}

impl TryFrom<LayoutDefinition> for LayoutComponent {
    type Error = LayoutError;

    fn try_from(definition: LayoutDefinition) -> Result<Self, Self::Error> {
        match definition {
            LayoutDefinition::Square { size, component } => {
                LayoutComponent::new_square(size, (*component).try_into()?)
            }
        }
    }
}

impl From<&LayoutComponent> for LayoutDefinition {
    fn from(layout: &LayoutComponent) -> Self {
        match layout {
            LayoutComponent::Square { size, component } => {
                let definition: ComponentDefinition = (&(**component)).into();
                LayoutDefinition::Square {
                    size: *size,
                    component: Box::new(definition),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definition::generation::rendering::RenderDefinition;
    use crate::definition::math::shape::ShapeDefinition;
    use crate::math::color::RED;
    use std::convert::TryInto;

    #[test]
    fn test_convert_square() {
        let shape = ShapeDefinition::Circle(42);
        let rendering = RenderDefinition::Shape { shape, color: RED };
        let component = Box::new(ComponentDefinition::Rendering(rendering));
        assert_convert(LayoutDefinition::Square {
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
