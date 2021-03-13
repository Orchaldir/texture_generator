use crate::definition::generation::component::ComponentDefinition;
use crate::generation::TextureGenerator;
use crate::utils::error::GenerationError;
use std::convert::{TryFrom, TryInto};

pub mod component;
pub mod layout;
pub mod rendering;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextureDefinition {
    name: String,
    component: ComponentDefinition,
}

impl TextureDefinition {
    pub fn new<S: Into<String>>(name: S, component: ComponentDefinition) -> TextureDefinition {
        TextureDefinition {
            name: name.into(),
            component,
        }
    }
}

impl TryFrom<TextureDefinition> for TextureGenerator {
    type Error = GenerationError;

    fn try_from(definition: TextureDefinition) -> Result<Self, Self::Error> {
        Ok(TextureGenerator::new(
            definition.name,
            definition.component.try_into()?,
        ))
    }
}

impl From<&TextureGenerator> for TextureDefinition {
    fn from(generator: &TextureGenerator) -> Self {
        TextureDefinition::new(generator.name.clone(), (&generator.component).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definition::generation::component::ComponentDefinition;
    use crate::definition::generation::rendering::RenderingDefinition;
    use crate::definition::math::shape::ShapeDefinition;
    use crate::math::color::RED;
    use std::convert::TryInto;

    const SHAPE: ShapeDefinition = ShapeDefinition::Circle(42);

    #[test]
    fn test_convert_layout() {
        let rendering = RenderingDefinition::Shape {
            name: "brick".to_string(),
            shape: SHAPE,
            color: RED,
        };
        let component = ComponentDefinition::Rendering(Box::new(rendering));

        assert_convert(TextureDefinition::new("test", component));
    }

    fn assert_convert(definition: TextureDefinition) {
        let generator: TextureGenerator = definition.clone().try_into().unwrap();
        let result: TextureDefinition = (&generator).into();

        assert_eq!(result, definition)
    }
}
