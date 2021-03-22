use crate::math::color::Color;
use rand::Rng;

#[derive(Clone, Debug, PartialEq)]
pub enum ColorSelector {
    /// Everything has the same color.
    ConstantColor(Color),
    /// Randomly select from a list of colors with equal probability.
    UniformDistribution(Vec<Color>),
}

impl ColorSelector {
    pub fn select(&self, random: &mut dyn rand::RngCore) -> Color {
        match self {
            ColorSelector::ConstantColor(color) => *color,
            ColorSelector::UniformDistribution(colors) => {
                let index = random.gen::<usize>() % colors.len();
                colors[index]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::color::{BLUE, GREEN, RED};
    use rand::rngs::mock::StepRng;
    use rand::RngCore;

    #[test]
    fn test_constant() {
        let mut random = StepRng::new(0, 1);
        let selector = ColorSelector::ConstantColor(RED);

        assert_eq!(selector.select(&mut random), RED);
        assert_eq!(random.next_u32(), 0);
    }

    #[test]
    fn test_uniform() {
        let mut random = StepRng::new(0, 1);
        let selector = ColorSelector::UniformDistribution(vec![RED, GREEN, BLUE]);

        assert_eq!(selector.select(&mut random), RED);
        assert_eq!(selector.select(&mut random), GREEN);
        assert_eq!(selector.select(&mut random), BLUE);
    }
}
