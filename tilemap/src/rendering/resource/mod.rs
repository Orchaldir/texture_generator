use crate::rendering::style::door::DoorStyle;
use crate::rendering::style::furniture::FurnitureStyle;
use crate::rendering::style::node::NodeStyle;
use crate::rendering::style::wall::WallStyle;
use crate::rendering::style::window::WindowStyle;
use texture_generation::generation::process::PostProcess;
use texture_generation::generation::TextureGenerator;
use texture_generation::utils::resource::ResourceManager;

pub struct Resources {
    pub door_styles: ResourceManager<DoorStyle>,
    pub furniture_styles: ResourceManager<FurnitureStyle>,
    pub node_styles: ResourceManager<NodeStyle>,
    pub textures: ResourceManager<TextureGenerator>,
    pub wall_styles: ResourceManager<WallStyle>,
    pub window_styles: ResourceManager<WindowStyle>,
    pub post_processes: Vec<PostProcess>,
}

impl Resources {
    pub fn empty() -> Resources {
        Self::new(
            ResourceManager::default(),
            ResourceManager::default(),
            ResourceManager::default(),
            ResourceManager::default(),
            ResourceManager::default(),
            ResourceManager::default(),
            Vec::default(),
        )
    }

    pub fn new(
        door_styles: ResourceManager<DoorStyle>,
        furniture_styles: ResourceManager<FurnitureStyle>,
        node_styles: ResourceManager<NodeStyle>,
        textures: ResourceManager<TextureGenerator>,
        wall_styles: ResourceManager<WallStyle>,
        window_styles: ResourceManager<WindowStyle>,
        post_processes: Vec<PostProcess>,
    ) -> Self {
        Resources {
            door_styles,
            furniture_styles,
            node_styles,
            textures,
            wall_styles,
            window_styles,
            post_processes,
        }
    }
}
