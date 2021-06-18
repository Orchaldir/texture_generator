use crate::generation::component::rendering::color::ColorSelector;
use crate::generation::data::Data;
use crate::generation::random::{Random, COLOR_INDEX};
use crate::math::color::Color;
use anyhow::{bail, Result};

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

    pub fn new_probability(colors: Vec<(usize, Color)>) -> Result<ColorFactory> {
        Self::probability_width_random(Random::Hash, colors)
    }

    pub fn probability_width_random(
        random: Random,
        colors: Vec<(usize, Color)>,
    ) -> Result<ColorFactory> {
        if colors.len() < 2 {
            bail!("ColorFactory::Probability requires at least 2 colors");
        }

        let mut converted_colors = Vec::with_capacity(colors.len());
        let mut threshold = 0;

        for (i, (probability, color)) in colors.into_iter().enumerate() {
            if probability == 0 {
                bail!(format!("{}.probability of ColorFactory is 0", i + 1));
            }

            threshold += probability;
            converted_colors.push((threshold, color));
        }

        Ok(ColorFactory::Probability {
            random,
            colors: converted_colors,
            max_number: threshold,
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::color::{BLUE, GREEN, RED};

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
        ColorFactory::new_probability(vec![(100, RED)]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_new_probability_is_zero() {
        let colors = vec![(1, RED), (0, GREEN), (3, BLUE)];
        ColorFactory::new_probability(colors).unwrap();
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
