#[macro_use]
extern crate log;
extern crate rendering;

use rendering::implementation::window::GliumWindow;
use rendering::interface::app::App;
use rendering::interface::input::KeyCode;
use rendering::interface::rendering::{Initialization, Renderer};
use rendering::interface::window::Window;
use rendering::interface::{TextureId, BLACK};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct TilemapEditor {
    font_id: TextureId,
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
    let mut window = GliumWindow::default_size("Tilemap Editor");
    let app = Rc::new(RefCell::new(TilemapEditor::default()));

    window.run(app);
}
