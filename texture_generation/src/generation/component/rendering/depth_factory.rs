use crate::generation::component::rendering::depth::DepthCalculator;
use anyhow::{bail, Result};

#[derive(Clone, Debug, PartialEq)]
/// Calculates the depth for each pixel.
pub enum DepthFactory {
    /// All pixels have the same depth value.
    Uniform(u8),
    /// A linear interpolation between 2 depth values at the center & the border.
    InterpolateTwo { center: f32, diff: f32 },
    /// A linear interpolation between many depth values
    InterpolateMany(Vec<(f32, f32)>),
    /// Creates a dome.
    Dome { center: f32, diff: f32 },
}

impl DepthFactory {
    pub fn new_interpolate_two(center: u8, border: u8) -> DepthFactory {
        let diff = border as f32 - center as f32;
        DepthFactory::InterpolateTwo {
            center: center as f32,
            diff,
        }
    }

    pub fn new_interpolate_many(data: Vec<(f32, u8)>) -> Result<DepthFactory> {
        Ok(DepthFactory::InterpolateMany(convert_many(data)?))
    }

    pub fn new_dome(center: u8, border: u8) -> DepthFactory {
        let diff = border as f32 - center as f32;
        DepthFactory::Dome {
            center: center as f32,
            diff,
        }
    }

    /// Creates a ['DepthCalculator'] from the factory.
    pub fn create(&self) -> DepthCalculator {
        match self {
            DepthFactory::Uniform(depth) => DepthCalculator::Uniform(*depth),
            DepthFactory::InterpolateTwo { center, diff } => DepthCalculator::InterpolateTwo {
                center: *center,
                diff: *diff,
            },
            DepthFactory::InterpolateMany(data) => DepthCalculator::InterpolateMany(data.clone()),
            DepthFactory::Dome { center, diff } => DepthCalculator::Dome {
                center: *center,
                diff: *diff,
            },
        }
    }
}

pub fn convert_many(data: Vec<(f32, u8)>) -> Result<Vec<(f32, f32)>> {
    if data.len() < 2 {
        bail!("InterpolateMany requires 2 or more entries");
    }

    let mut converted_data = Vec::with_capacity(data.len());
    let mut last_pos = -0.00001;

    for (i, (pos, depth)) in data.into_iter().enumerate() {
        if pos <= last_pos {
            bail!(
                "{}.position of InterpolateMany is below {}",
                i + 1,
                if i == 0 { "zero" } else { "previous one" }
            );
        } else if pos > 1.0 {
            bail!("{}.position of InterpolateMany is above 1", i + 1);
        }

        converted_data.push((pos, depth as f32));
        last_pos = pos;
    }

    Ok(converted_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_interpolate_many_with_too_few_entries() {
        assert!(DepthFactory::new_interpolate_many(Vec::new()).is_err());
        assert!(DepthFactory::new_interpolate_many(vec![(0.5, 100)]).is_err());
    }

    #[test]
    #[should_panic]
    fn test_new_interpolate_many_with_pos_below_zero() {
        DepthFactory::new_interpolate_many(vec![(-0.3, 100), (0.7, 200)]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_new_interpolate_many_with_pos_below_previous() {
        DepthFactory::new_interpolate_many(vec![(0.3, 100), (0.2, 200)]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_new_interpolate_many_with_pos_above_one() {
        DepthFactory::new_interpolate_many(vec![(0.3, 100), (1.7, 200)]).unwrap();
    }
}
