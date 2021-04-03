extern crate glium;
extern crate rendering;

use rendering::implementation::window::GliumWindow;
use rendering::interface::app::App;
use rendering::interface::rendering::Renderer;
use rendering::interface::window::Window;
use rendering::interface::{BLUE, GREEN, RED, YELLOW};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct ColoredExample {}

impl App for ColoredExample {
    fn render(&mut self, renderer: &mut dyn Renderer) {
        renderer.start(BLUE);

        let color_renderer = renderer.get_color_renderer();
        color_renderer.render_triangle((400.0, 300.0), (600.0, 300.0), (500.0, 400.0), GREEN);
        color_renderer.render_triangle((100.0, 300.0), (300.0, 300.0), (200.0, 400.0), RED);
        color_renderer.render_rectangle((300.0, 40.0), (140.0, 50.0), YELLOW);
        color_renderer.render_rectangle((300.0, 500.0), (140.0, 50.0), GREEN);

        renderer.finish();
    }
}

fn main() {
    let mut window = GliumWindow::default_size("Example with colored Polygons");
    let app = Rc::new(RefCell::new(ColoredExample::default()));

    window.run(app);
}
