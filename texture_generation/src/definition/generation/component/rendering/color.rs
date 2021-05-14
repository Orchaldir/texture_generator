use crate::generation::component::rendering::color::ColorSelector;
use crate::math::color::Color;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ColorSelectorDefinition {
    ConstantColor(String),
    Sequence(Vec<String>),
    Random(Vec<String>),
    Probability(Vec<(usize, String)>),
}

impl ColorSelectorDefinition {
    pub fn convert(&self) -> Result<ColorSelector> {
        match self {
            ColorSelectorDefinition::ConstantColor(color) => {
                let color = Color::convert(&color)?;
                Ok(ColorSelector::ConstantColor(color))
            }
            ColorSelectorDefinition::Sequence(colors) => {
                ColorSelector::new_sequence(convert_colors(colors)?)
            }
            ColorSelectorDefinition::Random(colors) => {
                ColorSelector::new_random(convert_colors(colors)?)
            }
            ColorSelectorDefinition::Probability(colors) => {
                let mut converted_colors = Vec::with_capacity(colors.len());

                for (probability, color) in colors {
                    let color = Color::convert(&color)?;
                    converted_colors.push((*probability, color));
                }

                ColorSelector::new_probability(converted_colors)
            }
        }
    }
}

fn convert_colors(colors: &[String]) -> Result<Vec<Color>> {
    let mut converted_colors = Vec::with_capacity(colors.len());

    for color in colors {
        let color = Color::convert(&color)?;
        converted_colors.push(color);
    }

    Ok(converted_colors)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generation::random::Random;
    use crate::math::color::{ORANGE, PINK};

    #[test]
    fn test_convert_const() {
        let definition = ColorSelectorDefinition::ConstantColor("#FFA500".to_string());
        let selector = ColorSelector::ConstantColor(ORANGE);

        assert_eq!(selector, definition.convert().unwrap())
    }

    #[test]
    fn test_convert_uniform() {
        let definition =
            ColorSelectorDefinition::Sequence(vec!["#FFA500".to_string(), "#FF0080".to_string()]);
        let selector = ColorSelector::new_sequence(vec![ORANGE, PINK]).unwrap();

        assert_eq!(selector, definition.convert().unwrap())
    }

    #[test]
    fn test_convert_random() {
        let definition =
            ColorSelectorDefinition::Random(vec!["#FFA500".to_string(), "#FF0080".to_string()]);
        let selector = ColorSelector::new_random(vec![ORANGE, PINK]).unwrap();

        assert_eq!(selector, definition.convert().unwrap())
    }

    #[test]
    fn test_convert_probability() {
        let definition = ColorSelectorDefinition::Probability(vec![
            (10, "#FFA500".to_string()),
            (5, "#FF0080".to_string()),
        ]);
        let selector = ColorSelector::Probability {
            random: Random::Hash,
            colors: vec![(10, ORANGE), (15, PINK)],
            max_number: 15,
        };

        assert_eq!(selector, definition.convert().unwrap())
    }
}
