use std::path::PathBuf;
use texture_generation::generation::process::ambient_occlusion::AmbientOcclusion;
use texture_generation::generation::process::lighting::Lighting;
use texture_generation::generation::process::PostProcess;
use texture_generation::math::vector3::Vector3;
use tilemap::rendering::Renderer;
use tilemap_io::rendering::resource::ResourceDefinitions;

pub struct ResourceInfo {
    resource_path: PathBuf,
    preview_tile_size: u32,
    render_tile_size: u32,
    wall_height: u8,
}

impl ResourceInfo {
    pub fn new(
        resource_path: PathBuf,
        preview_tile_size: u32,
        render_tile_size: u32,
        wall_height: u8,
    ) -> Self {
        Self {
            resource_path,
            preview_tile_size,
            render_tile_size,
            wall_height,
        }
    }

    pub fn load(&self) -> (Renderer, Renderer) {
        info!("Load definitions from {:?}", self.resource_path);

        let definitions = ResourceDefinitions::load(&self.resource_path);

        info!(
            "Init renderer: tile_size={} wall_height={}",
            self.render_tile_size, self.wall_height
        );

        let ambient_occlusion = AmbientOcclusion::new(3, -150.0, -0.75);
        let resources = definitions.convert(
            vec![
                PostProcess::AmbientOcclusion(ambient_occlusion),
                PostProcess::Lighting(Lighting::new(Vector3::new(1.0, 0.0, 2.0), 10, 32)),
            ],
            self.render_tile_size,
        );
        let renderer =
            tilemap::rendering::Renderer::new(self.render_tile_size, self.wall_height, resources);

        info!(
            "Init preview renderer: tile_size={}",
            self.preview_tile_size
        );

        let preview_resources = definitions.convert(Vec::default(), self.preview_tile_size);
        let preview_renderer = tilemap::rendering::Renderer::new(
            self.preview_tile_size,
            self.wall_height,
            preview_resources,
        );

        (renderer, preview_renderer)
    }
}
