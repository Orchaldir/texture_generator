#[macro_use]
extern crate log;
extern crate rendering;
extern crate texture_generation;
extern crate tilemap;

use rendering::implementation::window::GliumWindow;
use rendering::interface::app::App;
use rendering::interface::input::KeyCode;
use rendering::interface::rendering::{Initialization, Renderer};
use rendering::interface::window::Window;
use rendering::interface::{TextureId, BLACK};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use structopt::StructOpt;
use texture_generation::definition::generation::TextureDefinition;
use texture_generation::generation::TextureGenerator;
use texture_generation::math::size::Size;
use texture_generation::utils::logging::init_logging;
use tilemap::rendering::texture::TextureManager;
use tilemap::tilemap::tile::Tile;
use tilemap::tilemap::tilemap2d::Tilemap2d;

#[derive(StructOpt)]
#[structopt(name = "texture_generator")]
/// The arguments of the application.
struct Cli {
    /// The path of the texture definitions.
    #[structopt(parse(from_os_str), default_value = "resources/textures/")]
    texture_path: PathBuf,

    /// The width of the tilemap.
    #[structopt(default_value = "14")]
    width: u32,

    /// The height of the tilemap.
    #[structopt(default_value = "10")]
    height: u32,

    /// The size of a tile.
    #[structopt(default_value = "64")]
    tile_size: u32,

    /// The starting height for wall tiles.
    #[structopt(default_value = "200")]
    wall_height: u8,
}

pub struct TilemapEditor {
    font_id: TextureId,
    preview_id: TextureId,
    renderer: tilemap::rendering::Renderer,
    tilemap: Tilemap2d,
}

impl TilemapEditor {
    pub fn new(renderer: tilemap::rendering::Renderer, tilemap: Tilemap2d) -> TilemapEditor {
        TilemapEditor {
            font_id: 0,
            preview_id: 0,
            renderer,
            tilemap,
        }
    }

    fn render_preview(&mut self) {
        let data = self.renderer.render(&self.tilemap);
    }
}

impl App for TilemapEditor {
    fn init(&mut self, initialization: &mut dyn Initialization) {
        self.font_id = initialization.load_texture("ascii.png");
    }

    fn render(&mut self, renderer: &mut dyn Renderer) {
        renderer.start(BLACK);

        renderer.finish();
    }

    fn on_key_released(&mut self, key: KeyCode) {
        if key == KeyCode::Space {
            info!("Save tilemap")
        }
    }
}

fn main() {
    init_logging();

    let args = Cli::from_args();

    info!("Load texture definitions from {:?}", args.texture_path);

    let definitions = TextureDefinition::read_dir("resources/textures/".as_ref());

    info!("Loaded {} texture definitions", definitions.len());

    let textures: Vec<TextureGenerator> = definitions
        .into_iter()
        .filter_map(|d| d.convert(args.tile_size).ok())
        .collect();

    if textures.is_empty() {
        panic!("Not enough textures!");
    }

    info!("Loaded {} textures", textures.len());

    info!("Init tilemap: width={} height={}", args.width, args.height);

    let tiles = Size::new(args.width, args.height);
    let tilemap = Tilemap2d::default(tiles, Tile::Empty);

    info!(
        "Init renderer: tile_size={} wall_height={} ",
        args.tile_size, args.wall_height
    );

    let texture_mgr = TextureManager::new(textures);
    let renderer = tilemap::rendering::Renderer::new(
        args.tile_size,
        args.wall_height,
        texture_mgr,
        Vec::default(),
    );

    let mut window = GliumWindow::default_size("Tilemap Editor");
    let editor = TilemapEditor::new(renderer, tilemap);
    let app = Rc::new(RefCell::new(editor));

    window.run(app);
}
