use crate::generation::process::lighting::Lighting;
use crate::math::vector3::Vector3;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        let light_direction = Vector3::new(1.0, 0.0, 0.0);
        let definition = LightingDefinition {
            light_direction,
            normal_z: 10,
            shininess: 32,
        };
        let result = Lighting::new(light_direction, 10, 32);

        assert_eq!(result, definition.into())
    }
}
