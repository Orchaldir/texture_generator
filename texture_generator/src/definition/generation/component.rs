use crate::definition::generation::layout::LayoutDefinition;
use crate::definition::generation::rendering::RenderingDefinition;
use crate::generation::component::GenerationComponent;
use crate::generation::layout::LayoutError;
use crate::generation::rendering::RenderingError;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Eq, PartialEq)]
pub enum ComponentError {
    LayoutError(LayoutError),
    RenderingError(RenderingError),
}

impl From<LayoutError> for ComponentError {
    fn from(error: LayoutError) -> Self {
        ComponentError::LayoutError(error)
    }
}

impl From<RenderingError> for ComponentError {
    fn from(error: RenderingError) -> Self {
        ComponentError::RenderingError(error)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ComponentDefinition {
    Layout(LayoutDefinition),
    Rendering(RenderingDefinition),
}

impl TryFrom<ComponentDefinition> for GenerationComponent {
    type Error = ComponentError;

    fn try_from(definition: ComponentDefinition) -> Result<Self, Self::Error> {
        match definition {
            ComponentDefinition::Layout(definition) => {
                Ok(GenerationComponent::Layout(definition.try_into()?))
            }
            ComponentDefinition::Rendering(definition) => {
                Ok(GenerationComponent::Rendering(definition.try_into()?))
            }
        }
    }
}

impl From<&GenerationComponent> for ComponentDefinition {
    fn from(component: &GenerationComponent) -> Self {
        match component {
            GenerationComponent::Layout(layout) => ComponentDefinition::Layout(layout.into()),
            GenerationComponent::Rendering(render) => ComponentDefinition::Rendering(render.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definition::math::shape::ShapeDefinition;
    use crate::math::color::RED;
    use std::convert::TryInto;

    const SHAPE: ShapeDefinition = ShapeDefinition::Circle(42);
    const RENDERING: RenderingDefinition = RenderingDefinition::Shape {
        shape: SHAPE,
        color: RED,
    };

    #[test]
    fn test_convert_layout() {
        let layout = LayoutDefinition::Square {
            size: 10,
            component: Box::new(ComponentDefinition::Rendering(RENDERING)),
        };

        assert_convert(ComponentDefinition::Layout(layout));
    }

    #[test]
    fn test_convert_rendering() {
        assert_convert(ComponentDefinition::Rendering(RENDERING));
    }

    fn assert_convert(definition: ComponentDefinition) {
        let shape: GenerationComponent = definition.clone().try_into().unwrap();
        let result: ComponentDefinition = (&shape).into();

        assert_eq!(result, definition)
    }
}
