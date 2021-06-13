use std::path::PathBuf;
use structopt::StructOpt;
use texture_generation::definition::read;
use texture_generation::generation::process::ambient_occlusion::AmbientOcclusion;
use texture_generation::generation::process::lighting::Lighting;
use texture_generation::generation::process::PostProcess;
use texture_generation::math::point::Point;
use texture_generation::math::size::Size;
use texture_generation::math::vector3::Vector3;
use texture_generation::utils::error::ResourceError;
use tilemap::rendering::Renderer;
use tilemap::tilemap::furniture::map2d::FurnitureMap2d;
use tilemap::tilemap::furniture::Furniture;
use tilemap::tilemap::tilemap2d::Tilemap2d;
use tilemap::tilemap::Side::*;
use tilemap_io::rendering::resource::lookup::ResourceLookup;
use tilemap_io::tilemap::load;

#[derive(StructOpt)]
#[structopt(name = "texture_generator")]
pub struct ResourceInfo {
    /// The path of the resource lookup.
    #[structopt(parse(from_os_str), default_value = "resources/lookup.yaml")]
    lookup_path: PathBuf,

    /// The path of the resource definitions.
    #[structopt(parse(from_os_str), default_value = "resources")]
    resource_path: PathBuf,

    /// The path of the resource definitions.
    #[structopt(parse(from_os_str), default_value = "resources/tilemaps/example.tm")]
    tilemap_path: PathBuf,

    /// The size of a tile for the preview.
    #[structopt(default_value = "128")]
    preview_tile_size: u32,

    /// The size of a tile.
    #[structopt(default_value = "512")]
    render_tile_size: u32,

    /// The starting height for wall tiles.
    #[structopt(default_value = "200")]
    wall_height: u8,
}

impl ResourceInfo {
    /// Loads the needed [`Resource`]s and creates a normal & a preview [`Renderer`].
    pub fn create_renderers(&self) -> (Renderer, Renderer) {
        info!("Load lookup from {:?}", self.lookup_path);

        let lookup: Result<ResourceLookup, ResourceError> = read(&self.lookup_path);
        let lookup = match lookup {
            Ok(lookup) => lookup,
            Err(error) => {
                warn!("Couldn't read the lookup, because of {:?}", error);
                ResourceLookup::default()
            }
        };

        let definitions = lookup.convert(&self.resource_path);

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

    pub fn load_tilemap(&self) -> (Tilemap2d, FurnitureMap2d) {
        let tilemap = load(&self.tilemap_path).unwrap();

        info!(
            "Loaded tilemap: width={} height={}",
            tilemap.get_size().width(),
            tilemap.get_size().height()
        );

        let mut furniture_map = FurnitureMap2d::empty(tilemap.get_size());
        furniture_map.add(Furniture::new(2, Point::new(2, 2), Size::new(3, 2), Bottom));
        furniture_map.add(Furniture::new(3, Point::new(5, 2), Size::new(1, 2), Right));
        furniture_map.add(Furniture::new(1, Point::new(2, 7), Size::new(6, 1), Top));

        (tilemap, furniture_map)
    }
}
