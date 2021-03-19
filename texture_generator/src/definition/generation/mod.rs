use crate::definition::generation::component::ComponentDefinition;
use crate::definition::generation::process::PostProcessDefinition;
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
pub mod process;

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
    use crate::generation::component::rendering::depth::DepthCalculator;
    use crate::generation::component::rendering::RenderingComponent;
    use crate::generation::component::Component;
    use crate::generation::process::lighting::Lighting;
    use crate::generation::process::PostProcess;
    use crate::math::color::{BLUE, RED};
    use crate::math::shape::Shape;
    use crate::math::vector3::Vector3;
    use std::convert::TryInto;

    const SHAPE: Shape = Shape::Circle(42);

    #[test]
    fn test_convert_layout() {
        let depth = DepthCalculator::Uniform(111);
        let rendering = RenderingComponent::new_shape_with_depth("brick", SHAPE, RED, depth);
        let component = Component::Rendering(Box::new(rendering));
        let lighting = Lighting::new(Vector3::new(1.0, 0.0, 0.0), 20, 32);
        let processes = vec![PostProcess::Lighting(lighting)];

        assert_convert(TextureGenerator::new("test", BLUE, component, processes));
    }

    fn assert_convert(generator: TextureGenerator) {
        let definition: TextureDefinition = (&generator).into();
        let result: TextureGenerator = definition.clone().try_into().unwrap();

        assert_eq!(result, generator)
    }
}
