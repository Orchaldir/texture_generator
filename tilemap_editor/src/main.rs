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
use texture_generation::definition::generation::{into_manager, TextureDefinition};
use texture_generation::generation::component::rendering::RenderingComponent;
use texture_generation::generation::data::{convert, Data};
use texture_generation::generation::process::ambient_occlusion::AmbientOcclusion;
use texture_generation::generation::process::PostProcess;
use texture_generation::math::color::{Color, BLUE};
use texture_generation::math::size::Size;
use texture_generation::utils::logging::init_logging;
use texture_generation::utils::resource::ResourceManager;
use tilemap::rendering::wall::{EdgeStyle, NodeStyle, WallStyle};
use tilemap::tilemap::border::Border;
use tilemap::tilemap::tile::Tile;
use tilemap::tilemap::tilemap2d::Tilemap2d;
use tilemap::tilemap::Side::*;

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
    is_tile_mode: bool,
    current_id: usize,
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
            is_tile_mode: true,
            current_id: 0,
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

    fn paint_tile(&mut self, button: MouseButton, index: usize) {
        let tile = match button {
            MouseButton::Left => Tile::Floor(self.current_id),
            MouseButton::Middle => Tile::Empty,
            MouseButton::Right => Tile::Full(self.current_id),
        };

        info!("Set tile {} to {:?}", index, tile);

        self.tilemap.set_tile(index, tile);
    }

    fn paint_border(&mut self, button: MouseButton, point: (u32, u32), index: usize) {
        let tile_size = self.preview_renderer.get_tile_size();
        let start = self.tilemap.get_size().to_point(index);
        let x = (point.0 - start.x as u32 * tile_size) as f32 / tile_size as f32;
        let y = (point.1 - start.y as u32 * tile_size) as f32 / tile_size as f32;
        let border = 0.1;
        let is_top = y < border;
        let is_left = x < border;
        let is_bottom = y > (1.0 - border);
        let is_right = x > (1.0 - border);

        let side = if is_top && !is_left && !is_right {
            Top
        } else if is_left && !is_top && !is_bottom {
            Left
        } else if is_bottom && !is_left && !is_right {
            Bottom
        } else if is_right && !is_top && !is_bottom {
            Right
        } else {
            return;
        };

        let border = match button {
            MouseButton::Left => Border::Wall(self.current_id),
            _ => Border::Empty,
        };

        info!("Set {:?} border of tile {} to {:?}", side, index, border);

        self.tilemap.set_border(index, side, border);
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
            self.current_id = number;
        } else if key == KeyCode::F1 {
            self.is_tile_mode = true;
            self.current_id = 0;
        } else if key == KeyCode::F2 {
            self.is_tile_mode = false;
            self.current_id = 0;
        }
    }

    fn on_button_released(&mut self, button: MouseButton, point: (u32, u32)) {
        let index = self
            .preview_renderer
            .get_tile_index(&self.tilemap, point.0, point.1);

        if self.is_tile_mode {
            self.paint_tile(button, index);
        } else {
            self.paint_border(button, point, index);
        }

        self.has_changed = true;
    }
}

fn main() {
    init_logging();

    let args = Cli::from_args();

    info!("Load texture definitions from {:?}", args.texture_path);

    let definitions = TextureDefinition::read_dir("resources/textures/".as_ref());

    info!("Loaded {} texture definitions", definitions.len());

    let texture_mgr = into_manager(&definitions, args.tile_size);

    if texture_mgr.is_empty() {
        panic!("Not enough textures!");
    }

    let preview_texture_mgr = into_manager(&definitions, args.preview_size);

    info!("Converted {} textures", texture_mgr.len());

    info!("Init tilemap: width={} height={}", args.width, args.height);

    let tiles = Size::new(args.width, args.height);
    let mut tilemap2d = Tilemap2d::default(tiles, Tile::Empty);

    tilemap2d.set_tile(0, Tile::Full(0));
    tilemap2d.set_tile(1, Tile::Floor(0));
    tilemap2d.set_tile(2, Tile::Floor(1));
    tilemap2d.set_border(1, Bottom, Border::Wall(0));
    tilemap2d.set_border(2, Bottom, Border::Wall(0));
    tilemap2d.set_border(3, Bottom, Border::Wall(0));
    tilemap2d.set_border(15, Bottom, Border::Wall(0));
    tilemap2d.set_border(16, Bottom, Border::Wall(0));
    tilemap2d.set_border(17, Bottom, Border::Wall(0));
    tilemap2d.set_border(17, Right, Border::Wall(1));

    info!(
        "Init renderer: tile_size={} wall_height={}",
        args.tile_size, args.wall_height
    );

    let ambient_occlusion = AmbientOcclusion::new(50, -200.0, -1.0);
    let renderer = tilemap::rendering::Renderer::new(
        args.tile_size,
        args.wall_height,
        texture_mgr,
        crate_wall_styles(8),
        vec![PostProcess::AmbientOcclusion(ambient_occlusion)],
    );

    info!("Init preview renderer: tile_size={}", args.preview_size);

    let preview_renderer = tilemap::rendering::Renderer::new(
        args.preview_size,
        args.wall_height,
        preview_texture_mgr,
        crate_wall_styles(1),
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

fn crate_wall_styles(factor: u32) -> ResourceManager<WallStyle<NodeStyle>> {
    let style0 = crate_wall_style("stone", Color::gray(100), BLUE, 10 * factor, 16 * factor);
    let brown = Color::convert(&"#8B4513").unwrap();
    let style1 = crate_wall_style("wood", brown, brown, 6 * factor, 10 * factor);
    ResourceManager::new(vec![style0, style1])
}

fn crate_wall_style(
    name: &str,
    edge: Color,
    node: Color,
    thickness: u32,
    node_size: u32,
) -> WallStyle<NodeStyle> {
    let edge_component = RenderingComponent::new_fill_area("wall", edge, 0);
    let edge_style = EdgeStyle::new_solid(thickness, edge_component);
    let node_component = RenderingComponent::new_fill_area("node", node, 20);
    let node_style = NodeStyle::new(node_size, node_component);
    WallStyle::new(name, edge_style, None, node_style)
}
