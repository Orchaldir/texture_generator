use crate::rendering::style::door::DoorDefinition;
use crate::rendering::style::node::NodeDefinition;
use crate::rendering::style::wall::WallDefinition;
use crate::rendering::style::window::WindowDefinition;
use std::path::PathBuf;
use texture_generation::definition::generation::TextureDefinition;
use texture_generation::definition::read_dir;
use texture_generation::generation::process::PostProcess;
use texture_generation::utils::resource::into_manager;
use tilemap::rendering::resource::Resources;

pub struct ResourceDefinitions {
    doors: Vec<DoorDefinition>,
    nodes: Vec<NodeDefinition>,
    textures: Vec<TextureDefinition>,
    walls: Vec<WallDefinition>,
    windows: Vec<WindowDefinition>,
}

impl ResourceDefinitions {
    pub fn load(path: &PathBuf) -> Self {
        let textures: Vec<TextureDefinition> = read_dir(&path.join("textures"));

        info!("Loaded {} texture definitions", textures.len());

        let style_path = path.join("styles");
        let nodes: Vec<NodeDefinition> = read_dir(&style_path.join("nodes"));

        info!("Loaded {} node definitions", nodes.len());

        let doors: Vec<DoorDefinition> = read_dir(&style_path.join("doors"));

        info!("Loaded {} door definitions", doors.len());

        let walls: Vec<WallDefinition> = read_dir(&style_path.join("walls"));

        info!("Loaded {} wall definitions", walls.len());

        let windows: Vec<WindowDefinition> = read_dir(&style_path.join("windows"));

        info!("Loaded {} window definitions", windows.len());

        ResourceDefinitions {
            doors,
            nodes,
            textures,
            walls,
            windows,
        }
    }

    pub fn convert(&self, post_processes: Vec<PostProcess>, size: u32) -> Resources {
        Resources::new(
            into_manager(&self.doors, size),
            into_manager(&self.nodes, size),
            into_manager(&self.textures, size),
            into_manager(&self.walls, size),
            into_manager(&self.windows, size),
            post_processes,
        )
    }
}
