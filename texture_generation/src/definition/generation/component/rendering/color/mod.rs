use crate::definition::convert;
use crate::definition::generation::component::rendering::color::wood::WoodRingDefinition;
use crate::generation::component::rendering::color::factory::ColorFactory;
use crate::generation::component::rendering::color::wood::WoodFactory;
use crate::generation::random::Random;
use crate::math::color::Color;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

pub mod wood;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ColorFactoryDefinition {
    ConstantColor(String),
    Sequence(Vec<String>),
    Random(Vec<String>),
    Probability(Vec<(usize, String)>),
    Noise {
        color0: String,
        color1: String,
        base_factor: Option<f32>,
        scale_x: u32,
        scale_y: u32,
    },
    NoiseWithRandomColors {
        colors: Vec<(usize, String)>,
        base_factor: Option<f32>,
        scale_x: u32,
        scale_y: u32,
    },
    WoodRings {
        early_wood: WoodRingDefinition,
        late_wood: WoodRingDefinition,
        noise_amplitude: f32,
        noise_scale: u32,
    },
    Wood {
        is_horizontal: bool,
        early_wood: WoodRingDefinition,
        late_wood: WoodRingDefinition,
        noise_amplitude: f32,
        noise_scale: u32,
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
                ColorFactory::new_sequence(convert_colors(colors, "Sequence")?)
            }
            ColorFactoryDefinition::Random(colors) => {
                ColorFactory::new_random(convert_colors(colors, "Random")?)
            }
            ColorFactoryDefinition::Probability(colors) => {
                let converted_colors = convert_probability(colors, "Probability")?;

                ColorFactory::new_probability(Random::Hash, converted_colors)
            }
            ColorFactoryDefinition::Noise {
                color0,
                color1,
                base_factor,
                scale_x,
                scale_y,
            } => {
                let color0 = Color::convert(&color0)
                    .context("Failed to convert 'color0' of 'ColorFactory.Noise'")?;
                let color1 = Color::convert(&color1)
                    .context("Failed to convert 'color1' of 'ColorFactory.Noise'")?;

                Ok(ColorFactory::Noise {
                    color0,
                    color1,
                    base_factor: base_factor.unwrap_or_default(),
                    scale_x: convert(*scale_x, factor) as f64,
                    scale_y: convert(*scale_y, factor) as f64,
                })
            }
            ColorFactoryDefinition::NoiseWithRandomColors {
                colors,
                base_factor,
                scale_x,
                scale_y,
            } => {
                let converted_colors = convert_probability(colors, "NoiseWithRandomColors")?;
                ColorFactory::new_noise(
                    Random::Hash,
                    converted_colors,
                    base_factor.unwrap_or_default(),
                    convert(*scale_x, factor),
                    convert(*scale_y, factor),
                )
            }
            ColorFactoryDefinition::WoodRings {
                early_wood,
                late_wood,
                noise_amplitude,
                noise_scale,
            } => {
                let early_wood = early_wood
                    .convert(factor)
                    .context("Failed to convert 'early_wood' of 'ColorFactory.WoodRings'")?;
                let late_wood = late_wood
                    .convert(factor)
                    .context("Failed to convert 'late_wood' of 'ColorFactory.WoodRings'")?;

                Ok(ColorFactory::WoodRings(WoodFactory::new(
                    early_wood,
                    late_wood,
                    *noise_amplitude,
                    convert(*noise_scale, factor),
                )))
            }
            ColorFactoryDefinition::Wood {
                is_horizontal,
                early_wood,
                late_wood,
                noise_amplitude,
                noise_scale,
            } => {
                let early_wood = early_wood
                    .convert(factor)
                    .context("Failed to convert 'early_wood' of 'ColorFactory.Wood'")?;
                let late_wood = late_wood
                    .convert(factor)
                    .context("Failed to convert 'late_wood' of 'ColorFactory.Wood'")?;

                if *is_horizontal {
                    Ok(ColorFactory::WoodX(WoodFactory::new(
                        early_wood,
                        late_wood,
                        *noise_amplitude,
                        convert(*noise_scale, factor),
                    )))
                } else {
                    Ok(ColorFactory::WoodY(WoodFactory::new(
                        early_wood,
                        late_wood,
                        *noise_amplitude,
                        convert(*noise_scale, factor),
                    )))
                }
            }
        }
    }
}

fn convert_colors(colors: &[String], parent: &str) -> Result<Vec<Color>> {
    let mut converted_colors = Vec::with_capacity(colors.len());

    for (i, color) in colors.iter().enumerate() {
        let color = Color::convert(&color).context(format!(
            "Failed to convert the {}.color of 'ColorFactory.{}'",
            i + 1,
            parent
        ))?;
        converted_colors.push(color);
    }

    Ok(converted_colors)
}

fn convert_probability(colors: &[(usize, String)], parent: &str) -> Result<Vec<(usize, Color)>> {
    let mut converted_colors = Vec::with_capacity(colors.len());

    for (i, (probability, color)) in colors.iter().enumerate() {
        let color = Color::convert(&color).context(format!(
            "Failed to convert the {}.color of 'ColorFactory.{}'",
            i + 1,
            parent
        ))?;
        converted_colors.push((*probability, color));
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
            base_factor: Some(0.4),
            scale_x: 100,
            scale_y: 120,
        };
        let factory = ColorFactory::Noise {
            color0: ORANGE,
            color1: PINK,
            base_factor: 0.4,
            scale_x: 500.0,
            scale_y: 600.0,
        };

        assert_eq!(factory, definition.convert(5.0).unwrap())
    }

    #[test]
    fn test_convert_noise_with_random_colors() {
        let definition = ColorFactoryDefinition::NoiseWithRandomColors {
            colors: vec![(10, "#FFA500".to_string()), (5, "#FF0080".to_string())],
            base_factor: None,
            scale_x: 100,
            scale_y: 120,
        };
        let factory = ColorFactory::NoiseWithRandomColors {
            random: Random::Hash,
            colors: vec![(10, ORANGE), (15, PINK)],
            base_factor: 0.0,
            max_number: 15,
            scale_x: 600.0,
            scale_y: 720.0,
        };

        assert_eq!(factory, definition.convert(6.0).unwrap())
    }
}
