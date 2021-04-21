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
use texture_generation::generation::component::layout::LayoutComponent;
use texture_generation::generation::component::rendering::RenderingComponent;
use texture_generation::generation::component::Component;
use texture_generation::generation::data::{convert, Data};
use texture_generation::generation::process::ambient_occlusion::AmbientOcclusion;
use texture_generation::generation::process::PostProcess;
use texture_generation::math::color::{Color, BLUE, CYAN};
use texture_generation::math::shape_factory::ShapeFactory;
use texture_generation::math::size::Size;
use texture_generation::utils::logging::init_logging;
use texture_generation::utils::resource::ResourceManager;
use tilemap::rendering::style::door::DoorStyle;
use tilemap::rendering::style::edge::EdgeStyle;
use tilemap::rendering::style::node::NodeStyle;
use tilemap::rendering::style::wall::WallStyle;
use tilemap::rendering::style::window::WindowStyle;
use tilemap::rendering::Resources;
use tilemap::tilemap::border::Border;
use tilemap::tilemap::selector::Selector;
use tilemap::tilemap::tile::Tile;
use tilemap::tilemap::tilemap2d::Tilemap2d;
use tilemap::tilemap::Side::*;

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
            MouseButton::Right => Tile::Full(texture_id),
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
    tilemap2d.set_border(2, Bottom, Border::new_window(0, 0));
    tilemap2d.set_border(3, Bottom, Border::Wall(0));
    tilemap2d.set_border(15, Bottom, Border::Wall(0));
    tilemap2d.set_border(16, Bottom, Border::new_door(0, 0, false));
    tilemap2d.set_border(17, Bottom, Border::Wall(0));
    tilemap2d.set_border(17, Right, Border::Wall(1));

    info!(
        "Init renderer: tile_size={} wall_height={}",
        args.tile_size, args.wall_height
    );

    let ambient_occlusion = AmbientOcclusion::new(3, -150.0, -0.5);
    let resources = Resources::new(
        texture_mgr,
        crate_wall_styles(8),
        crate_door_styles(8),
        crate_window_styles(8),
        vec![PostProcess::AmbientOcclusion(ambient_occlusion)],
    );
    let renderer = tilemap::rendering::Renderer::new(args.tile_size, args.wall_height, resources);

    info!("Init preview renderer: tile_size={}", args.preview_size);

    let preview_resources = Resources::new(
        preview_texture_mgr,
        crate_wall_styles(1),
        crate_door_styles(1),
        crate_window_styles(1),
        Vec::default(),
    );
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

fn crate_wall_styles(factor: u32) -> ResourceManager<WallStyle<NodeStyle>> {
    let style0 = crate_wall_style("stone", Color::gray(100), BLUE, 10 * factor, 16 * factor);
    let brown = Color::convert(&"#8B4513").unwrap();
    let style1 = crate_wall_style("wood", brown, brown, 6 * factor, 10 * factor);
    let default_node = NodeStyle::default_with_size(16 * factor);
    let default_wall = WallStyle::default(10 * factor, default_node);
    ResourceManager::new(vec![style0, style1], default_wall)
}

fn crate_wall_style(
    name: &str,
    edge: Color,
    node: Color,
    thickness: u32,
    node_size: u32,
) -> WallStyle<NodeStyle> {
    let edge_rendering =
        RenderingComponent::new_shape("wall", ShapeFactory::RoundedRectangle(0.5), edge, 250);
    let edge_component = Component::Rendering(Box::new(edge_rendering));
    let edge_layout = LayoutComponent::new_repeat_x(thickness * 2, edge_component).unwrap();
    let edge_style = EdgeStyle::new_layout(thickness, edge_layout);
    let node_component = RenderingComponent::new_fill_area("node", node, 250);
    let node_style = NodeStyle::new(node_size, node_component);
    WallStyle::new(name, edge_style, None, node_style)
}

fn crate_door_styles(factor: u32) -> ResourceManager<DoorStyle> {
    let brown = Color::convert(&"#B8860B").unwrap();
    let style = crate_door_style("wooden", brown, 6 * factor);
    ResourceManager::new(vec![style], DoorStyle::default(6 * factor))
}

fn crate_door_style(name: &str, color: Color, thickness: u32) -> DoorStyle {
    let edge_component = RenderingComponent::new_fill_area("door", color, 220);
    let edge_style = EdgeStyle::new_solid(thickness, edge_component);
    DoorStyle::new(name, edge_style, false)
}

fn crate_window_styles(factor: u32) -> ResourceManager<WindowStyle> {
    let style = crate_window_style("glass", CYAN, 2 * factor, Color::gray(100), 16 * factor);
    ResourceManager::new(vec![style], WindowStyle::default(6 * factor))
}

fn crate_window_style(
    name: &str,
    pane_color: Color,
    pane_thickness: u32,
    stool_color: Color,
    stool_thickness: u32,
) -> WindowStyle {
    let pane_component = RenderingComponent::new_fill_area("pane", pane_color, 100);
    let pane_style = EdgeStyle::new_solid(pane_thickness, pane_component);
    let stool_component = RenderingComponent::new_fill_area("stool", stool_color, 100);
    let stool_style = EdgeStyle::new_solid(stool_thickness, stool_component);
    WindowStyle::new(name, pane_style, stool_style)
}
