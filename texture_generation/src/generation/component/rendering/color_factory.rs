use crate::generation::component::rendering::color::ColorSelector;
use crate::generation::data::Data;
use crate::generation::random::{Random, COLOR_INDEX};
use crate::math::color::Color;
use anyhow::{bail, Result};
use noise::{Perlin, Seedable};

#[derive(Clone, Debug, PartialEq)]
pub enum ColorFactory {
    /// Everything has the same color.
    ConstantColor(Color),
    /// A sequence of colors that repeats.
    Sequence(Vec<Color>),
    /// Randomly select a color from a list with equal probability.
    Random { random: Random, colors: Vec<Color> },
    /// Randomly select a color from a list based on probability.
    Probability {
        random: Random,
        colors: Vec<(usize, Color)>,
        max_number: usize,
    },
    /// Uses a noise function to interpolate between 2 colors.
    Noise {
        random: Random,
        colors: Vec<(usize, Color)>,
        max_number: usize,
        scale: f64,
    },
}

impl ColorFactory {
    pub fn new_sequence(colors: Vec<Color>) -> Result<ColorFactory> {
        if colors.len() < 2 {
            bail!("ColorFactory::Sequence requires at least 2 colors");
        }

        Ok(ColorFactory::Sequence(colors))
    }

    pub fn new_random(colors: Vec<Color>) -> Result<ColorFactory> {
        if colors.len() < 2 {
            bail!("ColorFactory::Random requires at least 2 colors");
        }

        Ok(ColorFactory::Random {
            random: Random::Hash,
            colors,
        })
    }

    pub fn mock_random(numbers: Vec<u64>, colors: Vec<Color>) -> Result<ColorFactory> {
        if colors.len() < 2 {
            bail!("ColorFactory::Random requires at least 2 colors");
        }

        Ok(ColorFactory::Random {
            random: Random::Mock(numbers),
            colors,
        })
    }

    pub fn new_probability(random: Random, colors: Vec<(usize, Color)>) -> Result<ColorFactory> {
        let (threshold, converted_colors) = convert_probability("Probability", colors)?;

        Ok(ColorFactory::Probability {
            random,
            colors: converted_colors,
            max_number: threshold,
        })
    }

    pub fn new_simple_noise(color0: Color, color1: Color, scale: u32) -> Result<ColorFactory> {
        Ok(ColorFactory::Noise {
            random: Random::Hash,
            colors: vec![(1, color0), (1, color1)],
            max_number: 2,
            scale: scale as f64,
        })
    }

    pub fn new_noise(
        random: Random,
        colors: Vec<(usize, Color)>,
        scale: u32,
    ) -> Result<ColorFactory> {
        let (threshold, converted_colors) = convert_probability("Noise", colors)?;

        Ok(ColorFactory::Noise {
            random,
            colors: converted_colors,
            max_number: threshold,
            scale: scale as f64,
        })
    }

    /// Creates a ['ColorSelector'].
    pub fn create(&self, data: &Data) -> ColorSelector {
        match self {
            ColorFactory::ConstantColor(color) => ColorSelector::ConstantColor(*color),
            ColorFactory::Sequence(colors) => {
                let index = data.get_instance_id() % colors.len();
                ColorSelector::ConstantColor(colors[index])
            }
            ColorFactory::Random { random, colors } => {
                let index = random.get_random_instance_usize(data, colors.len(), COLOR_INDEX);
                ColorSelector::ConstantColor(colors[index])
            }
            ColorFactory::Probability {
                random,
                colors,
                max_number,
            } => {
                let index = random.get_random_instance_usize(data, *max_number, COLOR_INDEX);

                for (threshold, color) in colors {
                    if index < *threshold {
                        return ColorSelector::ConstantColor(*color);
                    }
                }

                ColorSelector::ConstantColor(colors[0].1)
            }
            ColorFactory::Noise {
                random,
                colors,
                max_number,
                scale,
            } => ColorSelector::Noise {
                color0: colors[0].1.clone(),
                color1: colors[1].1.clone(),
                noise: Perlin::new().set_seed(data.get_instance_id() as u32),
                scale: *scale,
            },
        }
    }
}

fn convert_probability(
    parent: &str,
    colors: Vec<(usize, Color)>,
) -> Result<(usize, Vec<(usize, Color)>)> {
    if colors.len() < 2 {
        bail!("ColorFactory::{} requires at least 2 colors", parent);
    }

    let mut converted_colors = Vec::with_capacity(colors.len());
    let mut threshold = 0;

    for (i, (probability, color)) in colors.into_iter().enumerate() {
        if probability == 0 {
            bail!("{}.probability of ColorFactory::{} is 0", i + 1, parent);
        }

        threshold += probability;
        converted_colors.push((threshold, color));
    }

    Ok((threshold, converted_colors))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::color::{BLUE, GREEN, RED};
    use Random::Hash;

    #[test]
    #[should_panic]
    fn test_new_sequence_too_few_colors() {
        ColorFactory::new_sequence(vec![RED]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_new_random_too_few_colors() {
        ColorFactory::new_random(vec![RED]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_new_probability_too_colors() {
        ColorFactory::new_probability(Hash, vec![(100, RED)]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_new_probability_is_zero() {
        let colors = vec![(1, RED), (0, GREEN), (3, BLUE)];
        ColorFactory::new_probability(Hash, colors).unwrap();
    }

    #[test]
    fn test_constant() {
        let factory = ColorFactory::ConstantColor(RED);

        assert_cost(factory.create(&Data::only_instance_id(0)), RED);
        assert_cost(factory.create(&Data::only_instance_id(1)), RED);
        assert_cost(factory.create(&Data::only_instance_id(2)), RED);
    }

    #[test]
    fn test_sequence() {
        let factory = ColorFactory::new_sequence(vec![RED, GREEN, BLUE]).unwrap();

        assert_cost(factory.create(&Data::only_instance_id(0)), RED);
        assert_cost(factory.create(&Data::only_instance_id(1)), GREEN);
        assert_cost(factory.create(&Data::only_instance_id(2)), BLUE);
        assert_cost(factory.create(&Data::only_instance_id(3)), RED);
        assert_cost(factory.create(&Data::only_instance_id(4)), GREEN);
        assert_cost(factory.create(&Data::only_instance_id(5)), BLUE);
    }

    #[test]
    fn test_random() {
        let numbers = vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let colors = vec![RED, GREEN, BLUE];
        let factory = ColorFactory::mock_random(numbers, colors).unwrap();

        assert_cost(factory.create(&Data::only_instance_id(0)), BLUE);
        assert_cost(factory.create(&Data::only_instance_id(1)), RED);
        assert_cost(factory.create(&Data::only_instance_id(2)), GREEN);
        assert_cost(factory.create(&Data::only_instance_id(3)), BLUE);
        assert_cost(factory.create(&Data::only_instance_id(4)), RED);
    }

    #[test]
    fn test_probability() {
        let random = Random::Mock(vec![3, 4, 5, 6, 7, 8, 9, 10, 11]);
        let colors = vec![(1, RED), (2, GREEN), (3, BLUE)];
        let factory = ColorFactory::probability_width_random(random, colors).unwrap();

        assert_cost(factory.create(&Data::only_instance_id(0)), BLUE);
        assert_cost(factory.create(&Data::only_instance_id(1)), BLUE);
        assert_cost(factory.create(&Data::only_instance_id(2)), BLUE);
        assert_cost(factory.create(&Data::only_instance_id(3)), RED);
        assert_cost(factory.create(&Data::only_instance_id(4)), GREEN);
        assert_cost(factory.create(&Data::only_instance_id(5)), GREEN);
        assert_cost(factory.create(&Data::only_instance_id(6)), BLUE);
        assert_cost(factory.create(&Data::only_instance_id(7)), BLUE);
        assert_cost(factory.create(&Data::only_instance_id(8)), BLUE);
    }

    fn assert_cost(factory: ColorSelector, color: Color) {
        assert_eq!(factory, ColorSelector::ConstantColor(color));
    }
}
