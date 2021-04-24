use crate::rendering::style::door::DoorDefinition;
use crate::rendering::style::node::NodeDefinition;
use crate::rendering::style::wall::WallDefinition;
use crate::rendering::style::window::WindowDefinition;
use texture_generation::definition::generation::TextureDefinition;
use texture_generation::generation::process::PostProcess;
use texture_generation::utils::resource::into_manager;
use tilemap::rendering::resource::Resources;

pub mod style;

pub fn from_definitions(
    door_definitions: &[DoorDefinition],
    node_definitions: &[NodeDefinition],
    texture_definitions: &[TextureDefinition],
    wall_definitions: &[WallDefinition],
    window_definitions: &[WindowDefinition],
    post_processes: Vec<PostProcess>,
    size: u32,
) -> Resources {
    Resources::new(
        into_manager(&door_definitions, size),
        into_manager(&node_definitions, size),
        into_manager(&texture_definitions, size),
        into_manager(&wall_definitions, size),
        into_manager(&window_definitions, size),
        post_processes,
    )
}
