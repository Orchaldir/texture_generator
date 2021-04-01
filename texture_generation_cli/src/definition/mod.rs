use crate::generation::TextureGenerator;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use texture_generation::definition::generation::component::ComponentDefinition;
use texture_generation::definition::generation::process::PostProcessDefinition;
use texture_generation::math::color::Color;
use texture_generation::math::size::Size;
use texture_generation::utils::error::GenerationError;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextureDefinition {
    name: String,
    size: Size,
    background: String,
    component: ComponentDefinition,
    #[serde(default)]
    post_processes: Vec<PostProcessDefinition>,
}

impl TextureDefinition {
    pub fn new<S: Into<String>>(
        name: S,
        size: Size,
        background: String,
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
        let color = Color::convert(&self.background)
            .ok_or_else(|| GenerationError::invalid_color("background", &self.background))?;

        Ok(TextureGenerator::new(
            self.name.clone(),
            self.size * factor,
            color,
            component,
            post_processes,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use texture_generation::generation::component::Component;
    use texture_generation::generation::process::PostProcess;
    use texture_generation::math::color::BLUE;

    #[test]
    fn test_convert_layout() {
        let definition = TextureDefinition::new(
            "test",
            Size::new(100, 50),
            "#0000FF".to_string(),
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
