use crate::rendering::style::door::DoorDefinition;
use crate::rendering::style::node::NodeDefinition;
use crate::rendering::style::wall::WallDefinition;
use crate::rendering::style::window::WindowDefinition;
use texture_generation::definition::generation::TextureDefinition;
use texture_generation::generation::process::PostProcess;
use texture_generation::utils::resource::into_manager;
use tilemap::rendering::resource::Resources;

pub mod lookup;

pub struct ResourceDefinitions {
    doors: Vec<Option<(String, DoorDefinition)>>,
    nodes: Vec<Option<(String, NodeDefinition)>>,
    textures: Vec<Option<(String, TextureDefinition)>>,
    walls: Vec<Option<(String, WallDefinition)>>,
    windows: Vec<Option<(String, WindowDefinition)>>,
}

impl ResourceDefinitions {
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
