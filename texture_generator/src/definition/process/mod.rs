use crate::definition::process::lighting::LightingDefinition;
use crate::process::PostProcess;
use serde::{Deserialize, Serialize};

pub mod lighting;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PostProcessDefinition {
    Lighting(LightingDefinition),
}

impl From<PostProcessDefinition> for PostProcess {
    fn from(definition: PostProcessDefinition) -> Self {
        match definition {
            PostProcessDefinition::Lighting(definition) => PostProcess::Lighting(definition.into()),
        }
    }
}

impl From<&PostProcess> for PostProcessDefinition {
    fn from(process: &PostProcess) -> Self {
        match process {
            PostProcess::Lighting(lighting) => PostProcessDefinition::Lighting(lighting.into()),
        }
    }
}
