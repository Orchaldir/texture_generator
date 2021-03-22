use crate::generation::component::rendering::color::ColorSelector;
use crate::math::color::Color;
use crate::utils::error::GenerationError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ColorSelectorDefinition {
    ConstantColor(String),
    UniformDistribution(Vec<String>),
}

impl ColorSelectorDefinition {
    pub fn convert(&self, name: &str) -> Result<ColorSelector, GenerationError> {
        match self {
            ColorSelectorDefinition::ConstantColor(color) => {
                let color = Color::convert(&color)
                    .ok_or_else(|| GenerationError::invalid_color(name, &color))?;
                Ok(ColorSelector::ConstantColor(color))
            }
            ColorSelectorDefinition::UniformDistribution(colors) => {
                let mut converted_colors = Vec::with_capacity(colors.len());

                for color in colors {
                    let color = Color::convert(&color)
                        .ok_or_else(|| GenerationError::invalid_color(name, &color))?;
                    converted_colors.push(color);
                }

                Ok(ColorSelector::UniformDistribution(converted_colors))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::color::{ORANGE, PINK};

    #[test]
    fn test_convert_const() {
        let definition = ColorSelectorDefinition::ConstantColor("#FFA500".to_string());
        let selector = ColorSelector::ConstantColor(ORANGE);

        assert_eq!(selector, definition.convert("test").unwrap())
    }

    #[test]
    fn test_convert_uniform() {
        let definition = ColorSelectorDefinition::UniformDistribution(vec![
            "#FFA500".to_string(),
            "#FF0080".to_string(),
        ]);
        let selector = ColorSelector::UniformDistribution(vec![ORANGE, PINK]);

        assert_eq!(selector, definition.convert("test").unwrap())
    }
}
