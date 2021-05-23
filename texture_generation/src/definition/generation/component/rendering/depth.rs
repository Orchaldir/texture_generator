use crate::generation::component::rendering::depth_factory::DepthFactory;
use crate::generation::random::Random;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DepthDefinition {
    Uniform(u8),
    InterpolateTwo {
        center: u8,
        border: u8,
    },
    InterpolateMany(Vec<(f32, u8)>),
    Cylinder {
        is_horizontal: bool,
        center_depth: u8,
        border_depth: u8,
    },
    Dome {
        center: u8,
        border: u8,
    },
    Gradient {
        start: u8,
        end: u8,
    },
}

impl DepthDefinition {
    pub fn convert(&self) -> Result<DepthFactory> {
        match self {
            DepthDefinition::Uniform(depth) => Ok(DepthFactory::Uniform(*depth)),
            DepthDefinition::InterpolateTwo { center, border } => {
                Ok(DepthFactory::new_interpolate_two(*center, *border))
            }
            DepthDefinition::InterpolateMany(data) => {
                DepthFactory::new_interpolate_many(data.clone())
            }
            DepthDefinition::Cylinder {
                is_horizontal,
                center_depth,
                border_depth,
            } => Ok(DepthFactory::Cylinder {
                is_horizontal: *is_horizontal,
                center_depth: *center_depth,
                border_depth: *border_depth,
            }),
            DepthDefinition::Dome { center, border } => {
                Ok(DepthFactory::new_dome(*center, *border))
            }
            DepthDefinition::Gradient { start, end } => Ok(DepthFactory::Gradient {
                random: Random::Hash,
                start: *start,
                end: *end,
            }),
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
            DepthFactory::Uniform(42)
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
            DepthFactory::new_interpolate_two(100, 200)
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
            DepthFactory::new_dome(100, 200)
        );
    }
}
