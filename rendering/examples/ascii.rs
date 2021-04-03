extern crate glium;
extern crate rendering;

use rendering::implementation::window::GliumWindow;
use rendering::interface::app::App;
use rendering::interface::input::KeyCode;
use rendering::interface::rendering::{Initialization, Renderer};
use rendering::interface::window::Window;
use rendering::interface::{TextureId, BLUE, GREEN, RED, WHITE, YELLOW};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct AsciiExample {
    texture_id: TextureId,
    take_screenshot: bool,
}

impl App for AsciiExample {
    fn init(&mut self, initialization: &mut dyn Initialization) {
        self.texture_id = initialization.load_texture("ascii.png");
    }

    fn render(&mut self, renderer: &mut dyn Renderer) {
        renderer.start(BLUE);

        let ascii_renderer = renderer.get_ascii_renderer(self.texture_id);
        ascii_renderer.render_u8((200.0, 200.0), (40.0, 40.0), b'a', RED);
        ascii_renderer.render_char((300.0, 200.0), (40.0, 40.0), 'b', GREEN);
        ascii_renderer.render_text((300.0, 500.0), (20.0, 20.0), "Test?", WHITE);
        ascii_renderer.render_text(
            (0.0, 50.0),
            (20.0, 20.0),
            "Non-Ascii Symbols are replaced with '🎉'!",
            YELLOW,
        );

        renderer.finish();

        if self.take_screenshot {
            println!("Take screenshot");
            renderer.take_screenshot("ascii.png");
            self.take_screenshot = false;
        }
    }

    fn on_key_released(&mut self, key: KeyCode) {
        if key == KeyCode::Snapshot {
            self.take_screenshot = true;
        }
    }
}

fn main() {
    let mut window = GliumWindow::default_size("Example with ascii");
    let app = Rc::new(RefCell::new(AsciiExample::default()));

    window.run(app.clone());
}
