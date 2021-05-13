use crate::definition::generation::component::ComponentDefinition;
use crate::generation::TextureGenerator;
use crate::math::color::Color;
use crate::math::size::Size;
use crate::utils::error::ResourceError;
use crate::utils::resource::ResourceDefinition;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

pub mod component;
pub mod process;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextureDefinition {
    name: String,
    size: u32,
    background: String,
    component: ComponentDefinition,
}

impl TextureDefinition {
    pub fn new<S: Into<String>>(
        name: S,
        size: u32,
        background: String,
        component: ComponentDefinition,
    ) -> TextureDefinition {
        TextureDefinition {
            name: name.into(),
            size,
            background,
            component,
        }
    }

    pub fn write(&self, path: &str) -> Result<(), ResourceError> {
        let mut file = File::create(path)?;

        let s = serde_yaml::to_string(self)?;

        file.write_all(s.as_bytes())?;

        Ok(())
    }
}

impl ResourceDefinition for TextureDefinition {
    type R = TextureGenerator;

    fn convert(&self, size: u32) -> Result<TextureGenerator> {
        let factor = size as f32 / self.size as f32;
        let component = self
            .component
            .convert(&"component", factor)
            .context(format!(
                "Failed to convert 'component' of the texture '{}'",
                self.name
            ))?;
        let color = Color::convert(&self.background).context(format!(
            "Failed to convert 'background' of the texture '{}'",
            self.name
        ))?;

        Ok(TextureGenerator::new(
            self.name.clone(),
            Size::square(size),
            color,
            component,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generation::component::Component;
    use crate::math::color::BLUE;

    #[test]
    fn test_convert_layout() {
        let definition = TextureDefinition::new(
            "test",
            100,
            "#0000FF".to_string(),
            ComponentDefinition::Mock(42),
        );
        let generator = TextureGenerator::new("test", Size::square(200), BLUE, Component::Mock(42));

        assert_eq!(generator, definition.convert(200).unwrap());
    }
}
