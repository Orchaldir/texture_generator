use crate::definition::convert;
use crate::generation::component::rendering::color_factory::ColorFactory;
use crate::math::color::Color;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ColorFactoryDefinition {
    ConstantColor(String),
    Sequence(Vec<String>),
    Random(Vec<String>),
    Probability(Vec<(usize, String)>),
    Noise {
        color0: String,
        color1: String,
        scale: u32,
    },
}

impl ColorFactoryDefinition {
    pub fn convert(&self, factor: f32) -> Result<ColorFactory> {
        match self {
            ColorFactoryDefinition::ConstantColor(color) => {
                let color = Color::convert(&color)?;
                Ok(ColorFactory::ConstantColor(color))
            }
            ColorFactoryDefinition::Sequence(colors) => {
                ColorFactory::new_sequence(convert_colors(colors)?)
            }
            ColorFactoryDefinition::Random(colors) => {
                ColorFactory::new_random(convert_colors(colors)?)
            }
            ColorFactoryDefinition::Probability(colors) => {
                let mut converted_colors = Vec::with_capacity(colors.len());

                for (probability, color) in colors {
                    let color = Color::convert(&color)?;
                    converted_colors.push((*probability, color));
                }

                ColorFactory::new_probability(converted_colors)
            }
            ColorFactoryDefinition::Noise {
                color0,
                color1,
                scale,
            } => Ok(ColorFactory::Noise {
                color0: Color::convert(&color0)?,
                color1: Color::convert(&color1)?,
                scale: convert(*scale, factor) as f64,
            }),
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
        let definition = ColorFactoryDefinition::ConstantColor("#FFA500".to_string());
        let factory = ColorFactory::ConstantColor(ORANGE);

        assert_eq!(factory, definition.convert(1.0).unwrap())
    }

    #[test]
    fn test_convert_uniform() {
        let definition =
            ColorFactoryDefinition::Sequence(vec!["#FFA500".to_string(), "#FF0080".to_string()]);
        let factory = ColorFactory::new_sequence(vec![ORANGE, PINK]).unwrap();

        assert_eq!(factory, definition.convert(2.0).unwrap())
    }

    #[test]
    fn test_convert_random() {
        let definition =
            ColorFactoryDefinition::Random(vec!["#FFA500".to_string(), "#FF0080".to_string()]);
        let factory = ColorFactory::new_random(vec![ORANGE, PINK]).unwrap();

        assert_eq!(factory, definition.convert(3.0).unwrap())
    }

    #[test]
    fn test_convert_probability() {
        let definition = ColorFactoryDefinition::Probability(vec![
            (10, "#FFA500".to_string()),
            (5, "#FF0080".to_string()),
        ]);
        let factory = ColorFactory::Probability {
            random: Random::Hash,
            colors: vec![(10, ORANGE), (15, PINK)],
            max_number: 15,
        };

        assert_eq!(factory, definition.convert(4.0).unwrap())
    }

    #[test]
    fn test_convert_noise() {
        let definition = ColorFactoryDefinition::Noise {
            color0: "#FFA500".to_string(),
            color1: "#FF0080".to_string(),
            scale: 100,
        };
        let factory = ColorFactory::Noise {
            color0: ORANGE,
            color1: PINK,
            scale: 500.0,
        };

        assert_eq!(factory, definition.convert(5.0).unwrap())
    }
}
