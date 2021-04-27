use crate::generation::component::rendering::depth::DepthCalculator;
use crate::utils::error::ValueError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum DepthDefinition {
    Uniform(u8),
    Linear { center: u8, border: u8 },
    Dome { center: u8, border: u8 },
}

impl TryFrom<DepthDefinition> for DepthCalculator {
    type Error = ValueError;

    fn try_from(definition: DepthDefinition) -> Result<Self, Self::Error> {
        match definition {
            DepthDefinition::Uniform(depth) => Ok(DepthCalculator::Uniform(depth)),
            DepthDefinition::Linear { center, border } => {
                Ok(DepthCalculator::new_linear(center, border))
            }
            DepthDefinition::Dome { center, border } => {
                Ok(DepthCalculator::new_dome(center, border))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn test_convert_uniform() {
        assert_eq!(
            DepthDefinition::Uniform(42).try_into(),
            Ok(DepthCalculator::Uniform(42))
        );
    }

    #[test]
    fn test_convert_linear() {
        assert_eq!(
            DepthDefinition::Linear {
                center: 100,
                border: 200,
            }
            .try_into(),
            Ok(DepthCalculator::new_linear(100, 200))
        );
    }

    #[test]
    fn test_convert_dome() {
        assert_eq!(
            DepthDefinition::Dome {
                center: 100,
                border: 200,
            }
            .try_into(),
            Ok(DepthCalculator::new_dome(100, 200))
        );
    }
}
