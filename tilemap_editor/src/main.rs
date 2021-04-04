#[macro_use]
extern crate log;
extern crate rendering;
extern crate texture_generation;
extern crate tilemap;

use rendering::implementation::window::GliumWindow;
use rendering::interface::app::App;
use rendering::interface::input::{get_number, KeyCode, MouseButton};
use rendering::interface::rendering::{Initialization, Renderer};
use rendering::interface::window::Window;
use rendering::interface::{TextureId, BLACK, WHITE};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use structopt::StructOpt;
use texture_generation::definition::generation::TextureDefinition;
use texture_generation::generation::data::{convert, Data};
use texture_generation::generation::process::ambient_occlusion::AmbientOcclusion;
use texture_generation::generation::process::PostProcess;
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

    /// The size of a tile for the preview.
    #[structopt(default_value = "64")]
    preview_size: u32,

    /// The size of a tile.
    #[structopt(default_value = "512")]
    tile_size: u32,

    /// The starting height for wall tiles.
    #[structopt(default_value = "200")]
    wall_height: u8,
}

pub struct TilemapEditor {
    font_id: TextureId,
    preview_id: TextureId,
    renderer: tilemap::rendering::Renderer,
    preview_renderer: tilemap::rendering::Renderer,
    tilemap: Tilemap2d,
    has_changed: bool,
    current_texture: usize,
}

impl TilemapEditor {
    pub fn new(
        renderer: tilemap::rendering::Renderer,
        preview_renderer: tilemap::rendering::Renderer,
        tilemap: Tilemap2d,
    ) -> TilemapEditor {
        TilemapEditor {
            font_id: 0,
            preview_id: 0,
            renderer,
            preview_renderer,
            tilemap,
            has_changed: true,
            current_texture: 0,
        }
    }

    fn render_preview(&mut self, renderer: &mut dyn Renderer) {
        if self.has_changed {
            info!("Render preview");

            let data = self.preview_renderer.render(&self.tilemap);
            let rbg = convert(data.get_color_data());
            let size = data.get_size();

            self.preview_id = renderer.create_rgb(&rbg, (size.width(), size.height()));
            self.has_changed = false;

            info!("Finished rendering preview");
        }
    }
}

impl App for TilemapEditor {
    fn init(&mut self, initialization: &mut dyn Initialization) {
        self.font_id = initialization.load_texture("ascii.png");
    }

    fn render(&mut self, renderer: &mut dyn Renderer) {
        self.render_preview(renderer);

        renderer.start(BLACK);

        let window_size = renderer.get_size();
        let rectangle_size = (window_size.0 as f32, window_size.1 as f32);

        renderer
            .get_texture_renderer(self.preview_id)
            .render_rectangle((0.0, 0.0), rectangle_size, (0.0, 0.0), (1.0, 1.0), WHITE);

        renderer.finish();
    }

    fn on_key_released(&mut self, key: KeyCode) {
        if key == KeyCode::Space {
            info!("Generate tilemap images");
            let data = self.renderer.render(&self.tilemap);
            data.save_color_image("tilemap-color.png");
            data.save_depth_image("tilemap-depth.png");
            info!("Finished saving tilemap images");
        } else if let Some(number) = get_number(key) {
            self.current_texture = number;
        }
    }

    fn on_button_released(&mut self, button: MouseButton, point: (u32, u32)) {
        let tile_size = self.preview_renderer.get_tile_size();
        let tile_x = point.0 / tile_size;
        let tile_y = point.1 / tile_size;
        let index = self.tilemap.get_size().convert_x_y(tile_x, tile_y);

        let tile = match button {
            MouseButton::Left => Tile::Floor(self.current_texture),
            MouseButton::Middle => Tile::Empty,
            MouseButton::Right => Tile::Full(self.current_texture),
        };

        info!(
            "Set tile {} (x={} y={}) to {:?}",
            index, tile_x, tile_y, tile
        );

        self.tilemap.set_tile(index, tile);
        self.has_changed = true;
    }
}

fn main() {
    init_logging();

    let args = Cli::from_args();

    info!("Load texture definitions from {:?}", args.texture_path);

    let definitions = TextureDefinition::read_dir("resources/textures/".as_ref());

    info!("Loaded {} texture definitions", definitions.len());

    let textures: Vec<TextureGenerator> = definitions
        .clone()
        .into_iter()
        .filter_map(|d| d.convert(args.tile_size).ok())
        .collect();

    if textures.is_empty() {
        panic!("Not enough textures!");
    }

    let preview_textures: Vec<TextureGenerator> = definitions
        .into_iter()
        .filter_map(|d| d.convert(args.preview_size).ok())
        .collect();

    info!("Loaded {} textures", textures.len());

    info!("Init tilemap: width={} height={}", args.width, args.height);

    let tiles = Size::new(args.width, args.height);
    let mut tilemap2d = Tilemap2d::default(tiles, Tile::Empty);

    tilemap2d.set_tile(0, Tile::Full(0));
    tilemap2d.set_tile(1, Tile::Floor(0));
    tilemap2d.set_tile(2, Tile::Floor(1));

    info!(
        "Init renderer: tile_size={} wall_height={} ",
        args.tile_size, args.wall_height
    );

    let texture_mgr = TextureManager::new(textures);
    let post_process = AmbientOcclusion::new(50, -200.0, -1.0);
    let renderer = tilemap::rendering::Renderer::new(
        args.tile_size,
        args.wall_height,
        texture_mgr,
        vec![PostProcess::AmbientOcclusion(post_process)],
    );

    info!("Init preview renderer: tile_size={}", args.preview_size);

    let preview_texture_mgr = TextureManager::new(preview_textures);
    let preview_renderer = tilemap::rendering::Renderer::new(
        args.preview_size,
        args.wall_height,
        preview_texture_mgr,
        Vec::default(),
    );

    let window_size = (
        args.width * args.preview_size,
        args.height * args.preview_size,
    );
    let mut window = GliumWindow::new("Tilemap Editor", window_size);
    let editor = TilemapEditor::new(renderer, preview_renderer, tilemap2d);
    let app = Rc::new(RefCell::new(editor));

    window.run(app);
}
