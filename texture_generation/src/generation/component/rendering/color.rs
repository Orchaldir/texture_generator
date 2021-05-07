use crate::generation::data::Data;
use crate::generation::random::get_random_instance_usize;
use crate::math::color::Color;

#[derive(Clone, Debug, PartialEq)]
pub enum ColorSelector {
    /// Everything has the same color.
    ConstantColor(Color),
    /// A sequence of colors that repeats.
    Sequence(Vec<Color>),
    /// Randomly select a color from a list with equal probability.
    Random(Vec<Color>),
}

impl ColorSelector {
    pub fn new_sequence(colors: Vec<Color>) -> ColorSelector {
        ColorSelector::Sequence(colors)
    }

    pub fn select(&self, data: &Data) -> Color {
        match self {
            ColorSelector::ConstantColor(color) => *color,
            ColorSelector::Sequence(colors) => {
                let index = data.get_instance_id() % colors.len();
                colors[index]
            }
            ColorSelector::Random(colors) => {
                let index = get_random_instance_usize(data, colors.len(), 0);
                colors[index]
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
}
