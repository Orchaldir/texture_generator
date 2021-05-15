use crate::generation::component::rendering::depth_factory::convert_many;
use crate::math::point::Point;
use anyhow::Result;

#[derive(Clone, Debug, PartialEq)]
/// Calculates the depth for each pixel.
pub enum DepthCalculator {
    /// All pixels have the same depth value.
    Uniform(u8),
    /// A linear interpolation between 2 depth values at the center & the border.
    InterpolateTwo { center: f32, diff: f32 },
    /// A linear interpolation between many depth values.
    InterpolateMany(Vec<(f32, f32)>),
    /// Creates a dome.
    Dome { center: f32, diff: f32 },
    /// A gradient along the x-axis.
    GradientX {
        start_x: f32,
        diff_x: f32,
        start_depth: f32,
        diff_depth: f32,
    },
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
        Ok(DepthCalculator::InterpolateMany(convert_many(data)?))
    }

    pub fn new_dome(center: u8, border: u8) -> DepthCalculator {
        let diff = border as f32 - center as f32;
        DepthCalculator::Dome {
            center: center as f32,
            diff,
        }
    }

    /// Calculates the depth value based on a factor between 0 & 1.
    pub fn calculate(&self, point: &Point, factor: f32) -> u8 {
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
            DepthCalculator::GradientX {
                start_x,
                diff_x,
                start_depth,
                diff_depth,
            } => {
                let factor = (point.x as f32 - *start_x) / *diff_x;
                (*start_depth + factor * (*diff_depth)) as u8
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

    const POINT: Point = Point::new(i32::max_value(), i32::max_value());

    #[test]
    fn test_new_interpolate_many_with_too_few_entries() {
        assert!(DepthCalculator::new_interpolate_many(Vec::new()).is_err());
        assert!(DepthCalculator::new_interpolate_many(vec![(0.5, 100)]).is_err());
    }

    #[test]
    #[should_panic]
    fn test_new_interpolate_many_with_pos_below_zero() {
        DepthCalculator::new_interpolate_many(vec![(-0.3, 100), (0.7, 200)]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_new_interpolate_many_with_pos_below_previous() {
        DepthCalculator::new_interpolate_many(vec![(0.3, 100), (0.2, 200)]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_new_interpolate_many_with_pos_above_one() {
        DepthCalculator::new_interpolate_many(vec![(0.3, 100), (1.7, 200)]).unwrap();
    }

    #[test]
    fn test_uniform() {
        let calculator = DepthCalculator::Uniform(99);

        for i in 0..100 {
            let distance = i as f32 / 100.0;
            assert(&calculator, distance, 99);
        }
    }

    #[test]
    fn test_uniform_below_0() {
        let calculator = DepthCalculator::Uniform(99);

        assert(&calculator, -1.0, 99);
    }

    #[test]
    fn test_uniform_above_1() {
        let calculator = DepthCalculator::Uniform(99);

        assert(&calculator, 2.0, 99);
    }

    #[test]
    fn test_interpolate_two() {
        let calculator = DepthCalculator::new_interpolate_two(100, 200);

        assert(&calculator, 0.0, 100);
        assert(&calculator, 0.25, 125);
        assert(&calculator, 0.5, 150);
        assert(&calculator, 0.75, 175);
        assert(&calculator, 1.0, 200);
    }

    #[test]
    fn test_interpolate_two_decreasing() {
        let calculator = DepthCalculator::new_interpolate_two(200, 0);

        assert(&calculator, 0.0, 200);
        assert(&calculator, 0.25, 150);
        assert(&calculator, 0.5, 100);
        assert(&calculator, 0.75, 50);
        assert(&calculator, 1.0, 0);
    }

    #[test]
    fn test_interpolate_many() {
        let calculator =
            DepthCalculator::new_interpolate_many(vec![(0.3, 100), (0.7, 200)]).unwrap();

        assert(&calculator, 0.0, 100);
        assert(&calculator, 0.1, 100);
        assert(&calculator, 0.3, 100);
        assert(&calculator, 0.4, 125);
        assert(&calculator, 0.5, 150);
        assert(&calculator, 0.6, 175);
        assert(&calculator, 0.7, 200);
        assert(&calculator, 0.8, 200);
        assert(&calculator, 1.0, 200);
    }

    #[test]
    fn test_interpolate_many_decreasing() {
        let calculator =
            DepthCalculator::new_interpolate_many(vec![(0.3, 200), (0.7, 100)]).unwrap();

        assert(&calculator, 0.0, 200);
        assert(&calculator, 0.1, 200);
        assert(&calculator, 0.3, 200);
        assert(&calculator, 0.4, 175);
        assert(&calculator, 0.5, 150);
        assert(&calculator, 0.6, 124);
        assert(&calculator, 0.7, 100);
        assert(&calculator, 0.8, 100);
        assert(&calculator, 1.0, 100);
    }

    #[test]
    fn test_dome() {
        let calculator = DepthCalculator::new_dome(100, 200);

        assert(&calculator, 0.0, 100);
        assert(&calculator, 0.5, 113);
        assert(&calculator, 1.0, 200);
    }

    #[test]
    fn test_dome_decreasing() {
        let calculator = DepthCalculator::new_dome(200, 0);

        assert(&calculator, 0.0, 200);
        assert(&calculator, 0.5, 173);
        assert(&calculator, 1.0, 0);
    }

    #[test]
    fn test_gradient_x() {
        let calculator = DepthCalculator::GradientX {
            start_x: 10.0,
            diff_x: 4.0,
            start_depth: 100.0,
            diff_depth: 100.0,
        };

        assert_point(&calculator, &Point::new(10, 0), 100);
        assert_point(&calculator, &Point::new(11, 10), 125);
        assert_point(&calculator, &Point::new(12, 20), 150);
        assert_point(&calculator, &Point::new(13, 30), 175);
        assert_point(&calculator, &Point::new(14, -30), 200);
    }

    fn assert(calculator: &DepthCalculator, factor: f32, result: u8) {
        assert_eq!(calculator.calculate(&POINT, factor), result);
    }

    fn assert_point(calculator: &DepthCalculator, point: &Point, result: u8) {
        assert_eq!(calculator.calculate(point, 1000.0), result);
    }
}
