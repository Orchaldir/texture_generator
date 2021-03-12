use crate::definition::generation::rendering::RenderDefinition;
use crate::generation::component::GenerationComponent;
use crate::generation::rendering::RenderError;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Eq, PartialEq)]
pub enum ComponentError {
    RenderError(RenderError),
}

impl From<RenderError> for ComponentError {
    fn from(error: RenderError) -> Self {
        ComponentError::RenderError(error)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ComponentDefinition {
    Rendering(RenderDefinition),
}

impl TryFrom<ComponentDefinition> for GenerationComponent {
    type Error = ComponentError;

    fn try_from(definition: ComponentDefinition) -> Result<Self, Self::Error> {
        match definition {
            ComponentDefinition::Rendering(definition) => {
                Ok(GenerationComponent::Rendering(definition.try_into()?))
            }
        }
    }
}

impl From<&GenerationComponent> for ComponentDefinition {
    fn from(component: &GenerationComponent) -> Self {
        match component {
            GenerationComponent::Layout(_) => unimplemented!(),
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

    #[test]
    fn test_convert_rendering() {
        let shape = ShapeDefinition::Circle(42);
        let rendering = RenderDefinition::Shape { shape, color: RED };
        assert_convert(ComponentDefinition::Rendering(rendering));
    }

    fn assert_convert(definition: ComponentDefinition) {
        let shape: GenerationComponent = definition.clone().try_into().unwrap();
        let result: ComponentDefinition = (&shape).into();

        assert_eq!(result, definition)
    }
}
