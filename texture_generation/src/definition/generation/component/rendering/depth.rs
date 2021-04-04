use crate::generation::component::rendering::depth::DepthCalculator;
use crate::utils::error::DefinitionError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum DepthDefinition {
    Uniform(u8),
    Linear { center: u8, border: u8 },
}

impl TryFrom<DepthDefinition> for DepthCalculator {
    type Error = DefinitionError;

    fn try_from(definition: DepthDefinition) -> Result<Self, Self::Error> {
        match definition {
            DepthDefinition::Uniform(depth) => Ok(DepthCalculator::Uniform(depth)),
            DepthDefinition::Linear { center, border } => {
                Ok(DepthCalculator::new_linear(center, border))
            }
        }
    }
}

impl From<&DepthCalculator> for DepthDefinition {
    fn from(component: &DepthCalculator) -> Self {
        match component {
            DepthCalculator::Uniform(depth) => DepthDefinition::Uniform(*depth),
            DepthCalculator::Linear { center, diff } => {
                let border = (*diff + *center) as u8;
                DepthDefinition::Linear {
                    center: *center as u8,
                    border,
                }
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
        assert_convert(DepthDefinition::Uniform(42));
    }

    #[test]
    fn test_convert_linear() {
        assert_convert(DepthDefinition::Linear {
            center: 100,
            border: 200,
        });
    }

    #[test]
    fn test_convert_linear_decreasing() {
        assert_convert(DepthDefinition::Linear {
            center: 200,
            border: 0,
        });
    }

    fn assert_convert(definition: DepthDefinition) {
        let shape: DepthCalculator = definition.clone().try_into().unwrap();
        let result: DepthDefinition = (&shape).into();

        assert_eq!(result, definition)
    }
}
