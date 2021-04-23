use crate::definition::generation::component::ComponentDefinition;
use crate::generation::TextureGenerator;
use crate::math::color::Color;
use crate::math::size::Size;
use crate::utils::error::DefinitionError;
use crate::utils::resource::ResourceDefinition;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

pub mod component;
pub mod process;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextureDefinition {
    name: String,
    size: Size,
    background: String,
    component: ComponentDefinition,
}

impl TextureDefinition {
    pub fn new<S: Into<String>>(
        name: S,
        size: Size,
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

    pub fn write(&self, path: &str) -> Result<(), DefinitionError> {
        let mut file = File::create(path)?;

        let s = serde_yaml::to_string(self)?;

        file.write_all(s.as_bytes())?;

        Ok(())
    }
}

impl ResourceDefinition for TextureDefinition {
    type R = TextureGenerator;

    fn convert(&self, size: u32) -> Result<TextureGenerator, DefinitionError> {
        let max = self.size.width().max(self.size.height());
        let factor = size as f32 / max as f32;
        let component = self.component.convert(factor)?;
        let color = Color::convert(&self.background)
            .ok_or_else(|| DefinitionError::invalid_color("background", &self.background))?;

        Ok(TextureGenerator::new(
            self.name.clone(),
            self.size * factor,
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
            Size::new(100, 50),
            "#0000FF".to_string(),
            ComponentDefinition::Mock(42),
        );
        let generator =
            TextureGenerator::new("test", Size::new(200, 100), BLUE, Component::Mock(42));

        assert_eq!(generator, definition.convert(200).unwrap());
    }
}
