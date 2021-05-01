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
use std::path::{Path, PathBuf};
use std::rc::Rc;
use structopt::StructOpt;
use texture_generation::generation::data::Data;
use texture_generation::generation::process::ambient_occlusion::AmbientOcclusion;
use texture_generation::generation::process::lighting::Lighting;
use texture_generation::generation::process::PostProcess;
use texture_generation::math::vector3::Vector3;
use texture_generation::utils::logging::init_logging;
use tilemap::tilemap::border::Border;
use tilemap::tilemap::selector::Selector;
use tilemap::tilemap::tile::Tile;
use tilemap::tilemap::tilemap2d::Tilemap2d;
use tilemap_io::rendering::resource::ResourceDefinitions;
use tilemap_io::tilemap::{load, save};
use texture_generation::math::color::convert;

#[derive(Copy, Clone)]
enum Mode {
    Tile,
    Wall,
    Door,
}

impl Mode {
    pub fn get_resource_type(&self) -> usize {
        match self {
            Mode::Tile => 0,
            Mode::Wall => 1,
            Mode::Door => 2,
        }
    }
}

#[derive(StructOpt)]
#[structopt(name = "texture_generator")]
/// The arguments of the application.
struct Cli {
    /// The path of the resource definitions.
    #[structopt(parse(from_os_str), default_value = "resources/")]
    resource_path: PathBuf,

    /// The path of the resource definitions.
    #[structopt(parse(from_os_str), default_value = "resources/tilemaps/example.tm")]
    tilemap_path: PathBuf,

    /// The width of the tilemap.
    #[structopt(default_value = "7")]
    width: u32,

    /// The height of the tilemap.
    #[structopt(default_value = "5")]
    height: u32,

    /// The size of a tile for the preview.
    #[structopt(default_value = "128")]
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
    selector: Selector,
    has_changed: bool,
    mode: Mode,
    resource_ids: Vec<usize>,
}

impl TilemapEditor {
    pub fn new(
        renderer: tilemap::rendering::Renderer,
        preview_renderer: tilemap::rendering::Renderer,
        tilemap: Tilemap2d,
    ) -> TilemapEditor {
        let selector = Selector::new(preview_renderer.get_tile_size());

        TilemapEditor {
            font_id: 0,
            preview_id: 0,
            renderer,
            preview_renderer,
            tilemap,
            selector,
            has_changed: true,
            mode: Mode::Tile,
            resource_ids: vec![0; 3],
        }
    }

    fn get_resource_id(&self, mode: Mode) -> usize {
        self.resource_ids[mode.get_resource_type()]
    }

    fn set_resource_id(&mut self, mode: Mode, id: usize) {
        self.resource_ids[mode.get_resource_type()] = id;
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

    fn paint_tile(&mut self, button: MouseButton, index: usize) {
        let texture_id = self.get_resource_id(Mode::Tile);

        let tile = match button {
            MouseButton::Left => Tile::Floor(texture_id),
            MouseButton::Middle => Tile::Empty,
            MouseButton::Right => Tile::Solid(texture_id),
        };

        info!("Set tile {} to {:?}", index, tile);

        self.tilemap.set_tile(index, tile);
        self.has_changed = true;
    }

    fn paint_wall(&mut self, button: MouseButton, point: (u32, u32), index: usize) {
        if let Some(side) = self
            .selector
            .get_side(&self.tilemap, point.0, point.1, index)
        {
            let border = match button {
                MouseButton::Left => Border::Wall(self.get_resource_id(Mode::Wall)),
                _ => Border::Empty,
            };

            info!("Set {:?} border of tile {} to {:?}", side, index, border);

            self.tilemap.set_border(index, side, border);
            self.has_changed = true;
        }
    }

    fn paint_door(&mut self, button: MouseButton, point: (u32, u32), index: usize) {
        if let Some(side) = self
            .selector
            .get_side(&self.tilemap, point.0, point.1, index)
        {
            let old_border = self.tilemap.get_border(index, side);

            let border = match button {
                MouseButton::Left => match old_border {
                    Border::Door { .. } => old_border.switch_is_front(),
                    _ => Border::new_door(
                        self.get_resource_id(Mode::Wall),
                        self.get_resource_id(Mode::Door),
                        true,
                    ),
                },
                _ => old_border.reduce(),
            };

            if old_border != border {
                info!("Set {:?} border of tile {} to {:?}", side, index, border);

                self.tilemap.set_border(index, side, border);
                self.has_changed = true;
            }
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
            self.set_resource_id(self.mode, number);
        } else if key == KeyCode::F1 {
            self.mode = Mode::Tile;
        } else if key == KeyCode::F2 {
            self.mode = Mode::Wall;
        } else if key == KeyCode::F3 {
            self.mode = Mode::Door;
        } else if key == KeyCode::S {
            save(&self.tilemap, "tilemap.tm").unwrap();
        } else if key == KeyCode::L {
            match load(&Path::new("tilemap.tm")) {
                Ok(new_tilemap) => {
                    self.tilemap = new_tilemap;
                    self.has_changed = true;
                }
                Err(e) => eprintln!("Error: {:?}", e),
            }
        }
    }

    fn on_button_released(&mut self, button: MouseButton, point: (u32, u32)) {
        let index = self
            .selector
            .get_tile_index(&self.tilemap, point.0, point.1);

        match self.mode {
            Mode::Tile => self.paint_tile(button, index),
            Mode::Wall => self.paint_wall(button, point, index),
            Mode::Door => self.paint_door(button, point, index),
        }
    }
}

fn main() {
    init_logging();

    let args = Cli::from_args();

    info!("Load definitions from {:?}", args.resource_path);

    let definitions = ResourceDefinitions::load(&args.resource_path);

    let tilemap2d = load(&args.tilemap_path).unwrap();

    info!(
        "Load tilemap: width={} height={}",
        tilemap2d.get_size().width(),
        tilemap2d.get_size().height()
    );

    info!(
        "Init renderer: tile_size={} wall_height={}",
        args.tile_size, args.wall_height
    );

    let ambient_occlusion = AmbientOcclusion::new(3, -150.0, -0.5);
    let resources = definitions.convert(
        vec![
            PostProcess::AmbientOcclusion(ambient_occlusion),
            PostProcess::Lighting(Lighting::new(Vector3::new(1.0, 0.0, 2.0), 10, 32)),
        ],
        args.tile_size,
    );
    let renderer = tilemap::rendering::Renderer::new(args.tile_size, args.wall_height, resources);

    info!("Init preview renderer: tile_size={}", args.preview_size);

    let preview_resources = definitions.convert(Vec::default(), args.preview_size);
    let preview_renderer =
        tilemap::rendering::Renderer::new(args.preview_size, args.wall_height, preview_resources);

    let window_size = (
        args.width * args.preview_size,
        args.height * args.preview_size,
    );
    let mut window = GliumWindow::new("Tilemap Editor", window_size);
    let editor = TilemapEditor::new(renderer, preview_renderer, tilemap2d);
    let app = Rc::new(RefCell::new(editor));

    window.run(app);
}
