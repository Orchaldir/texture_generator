use anyhow::{bail, Result};

#[derive(Clone, Debug, PartialEq)]
/// Calculates the depth for each pixel.
pub enum DepthCalculator {
    /// All pixels have the same depth value.
    Uniform(u8),
    /// A linear interpolation between 2 depth values at the center & the border.
    InterpolateTwo { center: f32, diff: f32 },
    /// A linear interpolation between many depth values
    InterpolateMany(Vec<(f32, f32)>),
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

    pub fn new_interpolate_many(data: Vec<(f32, u8)>) -> Result<DepthCalculator> {
        if data.len() < 2 {
            bail!("InterpolateMany requires 2 or more entries");
        }

        Ok(DepthCalculator::InterpolateMany(
            data.into_iter().map(|(a, b)| (a, b as f32)).collect(),
        ))
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
            DepthCalculator::InterpolateMany(data) => {
                let (mut last_pos, mut last_depth) = data[0];

                if factor <= last_pos {
                    return last_depth as u8;
                }

                for &(pos, depth) in data {
                    if factor <= pos {
                        return interpolate(factor, last_pos, pos, last_depth, depth);
                    }
                    last_pos = pos;
                    last_depth = depth;
                }

                last_depth as u8
            }
            DepthCalculator::Dome { center, diff } => {
                let factor = 1.0 - (1.0 - factor * factor).sqrt();
                (*center + factor * (*diff)) as u8
            }
        }
    }
}

fn interpolate(factor: f32, pos0: f32, pos1: f32, depth0: f32, depth1: f32) -> u8 {
    let factor = (factor - pos0) / (pos1 - pos0);
    (depth0 + factor * (depth1 - depth0)) as u8
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
    fn test_interpolate_many() {
        let calculator =
            DepthCalculator::new_interpolate_many(vec![(0.3, 100), (0.7, 200)]).unwrap();

        assert_eq!(calculator.calculate(0.0), 100);
        assert_eq!(calculator.calculate(0.1), 100);
        assert_eq!(calculator.calculate(0.3), 100);
        assert_eq!(calculator.calculate(0.4), 125);
        assert_eq!(calculator.calculate(0.5), 150);
        assert_eq!(calculator.calculate(0.6), 175);
        assert_eq!(calculator.calculate(0.7), 200);
        assert_eq!(calculator.calculate(0.8), 200);
        assert_eq!(calculator.calculate(1.0), 200);
    }

    #[test]
    fn test_interpolate_many_decreasing() {
        let calculator =
            DepthCalculator::new_interpolate_many(vec![(0.3, 200), (0.7, 100)]).unwrap();

        assert_eq!(calculator.calculate(0.0), 200);
        assert_eq!(calculator.calculate(0.1), 200);
        assert_eq!(calculator.calculate(0.3), 200);
        assert_eq!(calculator.calculate(0.4), 175);
        assert_eq!(calculator.calculate(0.5), 150);
        assert_eq!(calculator.calculate(0.6), 124);
        assert_eq!(calculator.calculate(0.7), 100);
        assert_eq!(calculator.calculate(0.8), 100);
        assert_eq!(calculator.calculate(1.0), 100);
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
