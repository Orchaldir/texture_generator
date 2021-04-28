#[derive(Clone, Debug, PartialEq)]
/// Calculates the depth for each pixel.
pub enum DepthCalculator {
    /// All pixels have the same depth value.
    Uniform(u8),
    /// A linear interpolation between 2 depth values at the center & the border.
    InterpolateTwo { center: f32, diff: f32 },
    /// Creates a dome.
    Dome { center: f32, diff: f32 },
}

impl DepthCalculator {
    pub fn new_interpolate_two(center: u8, border: u8) -> DepthCalculator {
        let diff = border as f32 - center as f32;
        DepthCalculator::InterpolateTwo {
            center: center as f32,
            diff,
        }
    }

    pub fn new_dome(center: u8, border: u8) -> DepthCalculator {
        let diff = border as f32 - center as f32;
        DepthCalculator::Dome {
            center: center as f32,
            diff,
        }
    }

    /// Calculates the depth value based on a factor between 0 & 1.
    pub fn calculate(&self, factor: f32) -> u8 {
        match self {
            DepthCalculator::Uniform(depth) => *depth,
            DepthCalculator::InterpolateTwo { center, diff } => (*center + factor * (*diff)) as u8,
            DepthCalculator::Dome { center, diff } => {
                let factor = 1.0 - (1.0 - factor * factor).sqrt();
                (*center + factor * (*diff)) as u8
            }
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
    fn test_interpolate_two() {
        let calculator = DepthCalculator::new_interpolate_two(100, 200);

        assert_eq!(calculator.calculate(0.0), 100);
        assert_eq!(calculator.calculate(0.25), 125);
        assert_eq!(calculator.calculate(0.5), 150);
        assert_eq!(calculator.calculate(0.75), 175);
        assert_eq!(calculator.calculate(1.0), 200);
    }

    #[test]
    fn test_interpolate_two_decreasing() {
        let calculator = DepthCalculator::new_interpolate_two(200, 0);

        assert_eq!(calculator.calculate(0.0), 200);
        assert_eq!(calculator.calculate(0.25), 150);
        assert_eq!(calculator.calculate(0.5), 100);
        assert_eq!(calculator.calculate(0.75), 50);
        assert_eq!(calculator.calculate(1.0), 0);
    }

    #[test]
    fn test_dome() {
        let calculator = DepthCalculator::new_dome(100, 200);

        assert_eq!(calculator.calculate(0.0), 100);
        assert_eq!(calculator.calculate(0.5), 113);
        assert_eq!(calculator.calculate(1.0), 200);
    }

    #[test]
    fn test_dome_decreasing() {
        let calculator = DepthCalculator::new_dome(200, 0);

        assert_eq!(calculator.calculate(0.0), 200);
        assert_eq!(calculator.calculate(0.5), 173);
        assert_eq!(calculator.calculate(1.0), 0);
    }
}
