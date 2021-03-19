use crate::definition::generation::component::ComponentDefinition;
use crate::definition::generation::process::PostProcessDefinition;
use crate::generation::TextureGenerator;
use crate::math::color::Color;
use crate::math::size::Size;
use crate::utils::error::GenerationError;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub mod component;
pub mod process;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextureDefinition {
    name: String,
    size: Size,
    background: Color,
    component: ComponentDefinition,
    post_processes: Vec<PostProcessDefinition>,
}

impl TextureDefinition {
    pub fn new<S: Into<String>>(
        name: S,
        size: Size,
        background: Color,
        component: ComponentDefinition,
        post_processes: Vec<PostProcessDefinition>,
    ) -> TextureDefinition {
        TextureDefinition {
            name: name.into(),
            size,
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

    pub fn convert(&self, size: u32) -> Result<TextureGenerator, GenerationError> {
        let max = self.size.width().max(self.size.height());
        let factor = size as f32 / max as f32;
        let component = self.component.convert(factor)?;
        let post_processes = self
            .post_processes
            .clone()
            .into_iter()
            .map(|process| process.into())
            .collect();

        Ok(TextureGenerator::new(
            self.name.clone(),
            self.size * factor,
            self.background,
            component,
            post_processes,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generation::component::Component;
    use crate::generation::process::PostProcess;
    use crate::math::color::BLUE;

    #[test]
    fn test_convert_layout() {
        let definition = TextureDefinition::new(
            "test",
            Size::new(100, 50),
            BLUE,
            ComponentDefinition::Mock(42),
            vec![PostProcessDefinition::Mock(13)],
        );
        let generator = TextureGenerator::new(
            "test",
            Size::new(200, 100),
            BLUE,
            Component::Mock(42),
            vec![PostProcess::Mock(13)],
        );

        assert_eq!(generator, definition.convert(200).unwrap());
    }
}
