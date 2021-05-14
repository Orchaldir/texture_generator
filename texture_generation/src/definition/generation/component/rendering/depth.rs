use crate::generation::component::rendering::depth::DepthCalculator;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DepthDefinition {
    Uniform(u8),
    InterpolateTwo { center: u8, border: u8 },
    InterpolateMany(Vec<(f32, u8)>),
    Dome { center: u8, border: u8 },
}

impl DepthDefinition {
    pub fn convert(&self) -> Result<DepthCalculator> {
        match self {
            DepthDefinition::Uniform(depth) => Ok(DepthCalculator::Uniform(*depth)),
            DepthDefinition::InterpolateTwo { center, border } => {
                Ok(DepthCalculator::new_interpolate_two(*center, *border))
            }
            DepthDefinition::InterpolateMany(data) => {
                DepthCalculator::new_interpolate_many(data.clone())
            }
            DepthDefinition::Dome { center, border } => {
                Ok(DepthCalculator::new_dome(*center, *border))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_uniform() {
        assert_eq!(
            DepthDefinition::Uniform(42).convert().unwrap(),
            DepthCalculator::Uniform(42)
        );
    }

    #[test]
    fn test_convert_interpolate_two() {
        assert_eq!(
            DepthDefinition::InterpolateTwo {
                center: 100,
                border: 200,
            }
            .convert()
            .unwrap(),
            DepthCalculator::new_interpolate_two(100, 200)
        );
    }

    #[test]
    fn test_convert_dome() {
        assert_eq!(
            DepthDefinition::Dome {
                center: 100,
                border: 200,
            }
            .convert()
            .unwrap(),
            DepthCalculator::new_dome(100, 200)
        );
    }
}
