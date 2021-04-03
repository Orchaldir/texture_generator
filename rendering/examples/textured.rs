extern crate glium;
extern crate rendering;

use rendering::implementation::window::GliumWindow;
use rendering::interface::app::App;
use rendering::interface::rendering::{Initialization, Renderer};
use rendering::interface::window::Window;
use rendering::interface::{TextureId, BLUE, RED};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct TextureExample {
    texture_id: TextureId,
}

impl App for TextureExample {
    fn init(&mut self, initialization: &mut dyn Initialization) {
        self.texture_id = initialization.load_texture("ascii.png");
    }

    fn render(&mut self, renderer: &mut dyn Renderer) {
        renderer.start(BLUE);
        renderer
            .get_texture_renderer(self.texture_id)
            .render_rectangle((200.0, 100.0), (400.0, 400.0), (0.0, 0.0), (1.0, 1.0), RED);
        renderer.finish();
    }
}

fn main() {
    let mut window = GliumWindow::default_size("Example with a texture");
    let app = Rc::new(RefCell::new(TextureExample::default()));

    window.run(app.clone());
}
