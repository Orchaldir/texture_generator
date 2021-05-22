use crate::rendering::resource::ResourceDefinitions;
use serde::{Deserialize, Serialize};
use std::path::Path;
use texture_generation::definition::read_resources;

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceLookup {
    doors: Vec<String>,
    nodes: Vec<String>,
    textures: Vec<String>,
    walls: Vec<String>,
    windows: Vec<String>,
}

impl ResourceLookup {
    pub fn convert(&self, path: &Path) -> ResourceDefinitions {
        info!("Load definitions from {:?}", path);

        let textures = read_resources(&path.join("textures"), &self.textures);

        info!("Loaded {} texture definitions", textures.len());

        let style_path = path.join("styles");

        let doors = read_resources(&style_path.join("doors"), &self.doors);

        info!("Loaded {} door definitions", doors.len());

        let nodes = read_resources(&style_path.join("nodes"), &self.nodes);

        info!("Loaded {} node definitions", nodes.len());

        let walls = read_resources(&style_path.join("walls"), &self.walls);

        info!("Loaded {} wall definitions", walls.len());

        let windows = read_resources(&style_path.join("windows"), &self.windows);

        info!("Loaded {} window definitions", windows.len());

        ResourceDefinitions {
            doors,
            nodes,
            textures,
            walls,
            windows,
        }
    }
}
