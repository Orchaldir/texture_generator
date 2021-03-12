use crate::definition::generation::layout::LayoutDefinition;
use crate::definition::generation::rendering::RenderingDefinition;
use crate::generation::component::Component;
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

impl TryFrom<ComponentDefinition> for Component {
    type Error = ComponentError;

    fn try_from(definition: ComponentDefinition) -> Result<Self, Self::Error> {
        match definition {
            ComponentDefinition::Layout(definition) => {
                Ok(Component::Layout(definition.try_into()?))
            }
            ComponentDefinition::Rendering(definition) => {
                Ok(Component::Rendering(definition.try_into()?))
            }
        }
    }
}

impl From<&Component> for ComponentDefinition {
    fn from(component: &Component) -> Self {
        match component {
            Component::Layout(layout) => ComponentDefinition::Layout(layout.into()),
            Component::Rendering(render) => ComponentDefinition::Rendering(render.into()),
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
        let shape: Component = definition.clone().try_into().unwrap();
        let result: ComponentDefinition = (&shape).into();

        assert_eq!(result, definition)
    }
}
