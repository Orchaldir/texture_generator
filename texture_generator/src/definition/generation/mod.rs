use crate::definition::generation::component::ComponentDefinition;
use crate::definition::process::PostProcessDefinition;
use crate::generation::TextureGenerator;
use crate::math::color::Color;
use crate::utils::error::GenerationError;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub mod component;
pub mod layout;
pub mod rendering;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextureDefinition {
    name: String,
    background: Color,
    component: ComponentDefinition,
    post_processes: Vec<PostProcessDefinition>,
}

impl TextureDefinition {
    pub fn new<S: Into<String>>(
        name: S,
        background: Color,
        component: ComponentDefinition,
        post_processes: Vec<PostProcessDefinition>,
    ) -> TextureDefinition {
        TextureDefinition {
            name: name.into(),
            background,
            component,
            post_processes,
        }
    }

    pub fn read(path: &PathBuf) -> Result<TextureDefinition> {
        let string = fs::read_to_string(path).context(format!("Unable to read {:?}", path))?;
        let data: TextureDefinition =
            serde_yaml::from_str(&string).context(format!("Unable to parse {:?}", path))?;
        Ok(data)
    }

    pub fn write(&self, path: &str) -> Result<(), GenerationError> {
        let mut file = File::create(path)?;

        let s = serde_yaml::to_string(self)?;

        file.write_all(s.as_bytes())?;

        Ok(())
    }
}

impl TryFrom<TextureDefinition> for TextureGenerator {
    type Error = GenerationError;

    fn try_from(definition: TextureDefinition) -> Result<Self, Self::Error> {
        Ok(TextureGenerator::new(
            definition.name,
            definition.background,
            definition.component.try_into()?,
            definition
                .post_processes
                .into_iter()
                .map(|process| process.into())
                .collect(),
        ))
    }
}

impl From<&TextureGenerator> for TextureDefinition {
    fn from(generator: &TextureGenerator) -> Self {
        TextureDefinition::new(
            generator.name.clone(),
            generator.background,
            (&generator.component).into(),
            generator
                .post_processes
                .iter()
                .map(|process| process.into())
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definition::generation::component::ComponentDefinition;
    use crate::definition::generation::rendering::depth::DepthDefinition;
    use crate::definition::generation::rendering::RenderingDefinition;
    use crate::definition::math::shape::ShapeDefinition;
    use crate::math::color::{BLUE, RED};
    use std::convert::TryInto;

    const SHAPE: ShapeDefinition = ShapeDefinition::Circle(42);

    #[test]
    fn test_convert_layout() {
        let depth = DepthDefinition::Uniform(111);
        let rendering = RenderingDefinition::Shape {
            name: "brick".to_string(),
            shape: SHAPE,
            color: RED,
            depth,
        };
        let component = ComponentDefinition::Rendering(Box::new(rendering));

        assert_convert(TextureDefinition::new("test", BLUE, component, Vec::new()));
    }

    fn assert_convert(definition: TextureDefinition) {
        let generator: TextureGenerator = definition.clone().try_into().unwrap();
        let result: TextureDefinition = (&generator).into();

        assert_eq!(result, definition)
    }
}
