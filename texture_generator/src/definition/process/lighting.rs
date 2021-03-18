use crate::math::vector3::Vector3;
use crate::process::lighting::Lighting;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LightingDefinition {
    light_direction: Vector3,
    normal_z: u32,
    shininess: u32,
}

impl From<LightingDefinition> for Lighting {
    fn from(definition: LightingDefinition) -> Self {
        Lighting::new(
            definition.light_direction,
            definition.normal_z,
            definition.shininess,
        )
    }
}

impl From<&Lighting> for LightingDefinition {
    fn from(lighting: &Lighting) -> Self {
        LightingDefinition {
            light_direction: lighting.light_direction,
            normal_z: lighting.normal_z as u32,
            shininess: lighting.shininess as u32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        assert_convert(Lighting::new(Vector3::new(1.0, 0.0, 0.0), 10, 32));
    }

    fn assert_convert(lighting: Lighting) {
        let definition: LightingDefinition = (&lighting).into();
        let result: Lighting = definition.clone().into();

        assert_eq!(result, lighting)
    }
}
