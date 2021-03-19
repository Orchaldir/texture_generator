use crate::definition::generation::process::lighting::LightingDefinition;
use crate::generation::process::PostProcess;
use serde::{Deserialize, Serialize};

pub mod lighting;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PostProcessDefinition {
    Lighting(LightingDefinition),
    Mock(u8),
}

impl From<PostProcessDefinition> for PostProcess {
    fn from(definition: PostProcessDefinition) -> Self {
        match definition {
            PostProcessDefinition::Lighting(definition) => PostProcess::Lighting(definition.into()),
            PostProcessDefinition::Mock(id) => PostProcess::Mock(id),
        }
    }
}

impl From<&PostProcess> for PostProcessDefinition {
    fn from(process: &PostProcess) -> Self {
        match process {
            PostProcess::Lighting(lighting) => PostProcessDefinition::Lighting(lighting.into()),
            PostProcess::Mock(id) => PostProcessDefinition::Mock(*id),
        }
    }
}
