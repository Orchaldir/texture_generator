use crate::definition::generation::process::lighting::LightingDefinition;
use crate::generation::process::PostProcess;
use crate::utils::error::DefinitionError;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub mod lighting;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PostProcessDefinition {
    Lighting(LightingDefinition),
    Mock(u8),
}

impl PostProcessDefinition {
    pub fn read(path: &Path) -> Result<Vec<PostProcessDefinition>, DefinitionError> {
        let string = fs::read_to_string(path)?;
        let data: Vec<PostProcessDefinition> = serde_yaml::from_str(&string)?;
        Ok(data)
    }
}

impl From<PostProcessDefinition> for PostProcess {
    fn from(definition: PostProcessDefinition) -> Self {
        match definition {
            PostProcessDefinition::Lighting(definition) => PostProcess::Lighting(definition.into()),
            PostProcessDefinition::Mock(id) => PostProcess::Mock(id),
        }
    }
}
