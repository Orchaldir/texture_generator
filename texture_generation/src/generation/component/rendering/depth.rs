#[derive(Clone, Debug, PartialEq)]
/// Calculates the depth for each pixel.
pub enum DepthCalculator {
    /// All pixels have the same depth value.
    Uniform(u8),
    /// A linear interpolation between the center & the border.
    Linear { center: f32, diff: f32 },
}

impl DepthCalculator {
    pub fn new_linear(center: u8, border: u8) -> DepthCalculator {
        let diff = border as f32 - center as f32;
        DepthCalculator::Linear {
            center: center as f32,
            diff,
        }
    }

    /// Calculates the depth value based on a distance between 0 & 1.
    pub fn calculate(&self, distance: f32) -> u8 {
        match self {
            DepthCalculator::Uniform(depth) => *depth,
            DepthCalculator::Linear { center, diff } => (*center + distance * (*diff)) as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniform() {
        let calculator = DepthCalculator::Uniform(99);

        for i in 0..100 {
            let distance = i as f32 / 100.0;
            assert_eq!(calculator.calculate(distance), 99);
        }
    }

    #[test]
    fn test_uniform_below_0() {
        let calculator = DepthCalculator::Uniform(99);

        assert_eq!(calculator.calculate(-1.0), 99);
    }

    #[test]
    fn test_uniform_above_1() {
        let calculator = DepthCalculator::Uniform(99);

        assert_eq!(calculator.calculate(2.0), 99);
    }

    #[test]
    fn test_linear() {
        let calculator = DepthCalculator::new_linear(100, 200);

        assert_eq!(calculator.calculate(0.0), 100);
        assert_eq!(calculator.calculate(0.25), 125);
        assert_eq!(calculator.calculate(0.5), 150);
        assert_eq!(calculator.calculate(0.75), 175);
        assert_eq!(calculator.calculate(1.0), 200);
    }

    #[test]
    fn test_linear_decreasing() {
        let calculator = DepthCalculator::new_linear(200, 0);

        assert_eq!(calculator.calculate(0.0), 200);
        assert_eq!(calculator.calculate(0.25), 150);
        assert_eq!(calculator.calculate(0.5), 100);
        assert_eq!(calculator.calculate(0.75), 50);
        assert_eq!(calculator.calculate(1.0), 0);
    }
}
