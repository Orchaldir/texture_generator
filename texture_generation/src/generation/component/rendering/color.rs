use crate::generation::data::Data;
use crate::generation::random::{Random, COLOR_INDEX};
use crate::math::color::Color;

#[derive(Clone, Debug, PartialEq)]
pub enum ColorSelector {
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

impl ColorSelector {
    pub fn new_sequence(colors: Vec<Color>) -> ColorSelector {
        ColorSelector::Sequence(colors)
    }

    pub fn new_random(colors: Vec<Color>) -> ColorSelector {
        ColorSelector::Random {
            random: Random::Hash,
            colors,
        }
    }

    pub fn mock_random(numbers: Vec<u64>, colors: Vec<Color>) -> ColorSelector {
        ColorSelector::Random {
            random: Random::Mock(numbers),
            colors,
        }
    }

    pub fn new_probability(colors: Vec<(usize, Color)>) -> ColorSelector {
        Self::probability_width_random(Random::Hash, colors)
    }

    pub fn probability_width_random(random: Random, colors: Vec<(usize, Color)>) -> ColorSelector {
        let mut converted_colors = Vec::with_capacity(colors.len());
        let mut threshold = 0;

        for (probability, color) in colors {
            threshold += probability;
            converted_colors.push((threshold, color));
        }

        ColorSelector::Probability {
            random,
            colors: converted_colors,
            max_number: threshold,
        }
    }

    pub fn select(&self, data: &Data) -> Color {
        match self {
            ColorSelector::ConstantColor(color) => *color,
            ColorSelector::Sequence(colors) => {
                let index = data.get_instance_id() % colors.len();
                colors[index]
            }
            ColorSelector::Random { random, colors } => {
                let index = random.get_random_instance_usize(data, colors.len(), COLOR_INDEX);
                colors[index]
            }
            ColorSelector::Probability {
                random,
                colors,
                max_number,
            } => {
                let index = random.get_random_instance_usize(data, *max_number, COLOR_INDEX);

                for (threshold, color) in colors {
                    if index < *threshold {
                        return *color;
                    }
                }

                colors[0].1
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::color::{BLUE, GREEN, RED};

    #[test]
    fn test_constant() {
        let selector = ColorSelector::ConstantColor(RED);

        assert_eq!(selector.select(&Data::only_instance_id(0)), RED);
        assert_eq!(selector.select(&Data::only_instance_id(1)), RED);
        assert_eq!(selector.select(&Data::only_instance_id(2)), RED);
    }

    #[test]
    fn test_uniform() {
        let selector = ColorSelector::new_sequence(vec![RED, GREEN, BLUE]);

        assert_eq!(selector.select(&Data::only_instance_id(0)), RED);
        assert_eq!(selector.select(&Data::only_instance_id(1)), GREEN);
        assert_eq!(selector.select(&Data::only_instance_id(2)), BLUE);
        assert_eq!(selector.select(&Data::only_instance_id(3)), RED);
        assert_eq!(selector.select(&Data::only_instance_id(4)), GREEN);
        assert_eq!(selector.select(&Data::only_instance_id(5)), BLUE);
    }

    #[test]
    fn test_random() {
        let numbers = vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let colors = vec![RED, GREEN, BLUE];
        let selector = ColorSelector::mock_random(numbers, colors);

        assert_eq!(selector.select(&Data::only_instance_id(0)), BLUE);
        assert_eq!(selector.select(&Data::only_instance_id(1)), RED);
        assert_eq!(selector.select(&Data::only_instance_id(2)), GREEN);
        assert_eq!(selector.select(&Data::only_instance_id(3)), BLUE);
        assert_eq!(selector.select(&Data::only_instance_id(4)), RED);
    }

    #[test]
    fn test_probability() {
        let random = Random::Mock(vec![3, 4, 5, 6, 7, 8, 9, 10, 11]);
        let colors = vec![(1, RED), (2, GREEN), (3, BLUE)];
        let selector = ColorSelector::probability_width_random(random, colors);

        assert_eq!(selector.select(&Data::only_instance_id(0)), BLUE);
        assert_eq!(selector.select(&Data::only_instance_id(1)), BLUE);
        assert_eq!(selector.select(&Data::only_instance_id(2)), BLUE);
        assert_eq!(selector.select(&Data::only_instance_id(3)), RED);
        assert_eq!(selector.select(&Data::only_instance_id(4)), GREEN);
        assert_eq!(selector.select(&Data::only_instance_id(5)), GREEN);
        assert_eq!(selector.select(&Data::only_instance_id(6)), BLUE);
        assert_eq!(selector.select(&Data::only_instance_id(7)), BLUE);
        assert_eq!(selector.select(&Data::only_instance_id(8)), BLUE);
    }
}
