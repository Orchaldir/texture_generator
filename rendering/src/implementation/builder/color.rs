use crate::implementation::renderer::get_other_corners;
use crate::implementation::vertex::ColoredVertex;
use crate::interface::rendering::ColorRenderer;
use crate::interface::{Color3f, Point2f};

#[derive(Default)]
pub struct ColorBuilder {
    pub vertices: Vec<ColoredVertex>,
}

impl ColorBuilder {
    fn add(&mut self, position: Point2f, color: Color3f) {
        self.vertices.push(ColoredVertex { position, color });
    }
}

impl ColorRenderer for ColorBuilder {
    fn render_triangle(&mut self, a: Point2f, b: Point2f, c: Point2f, color: Color3f) {
        self.add(a, color);
        self.add(b, color);
        self.add(c, color);
    }

    fn render_rectangle(&mut self, position: Point2f, size: Point2f, color: Color3f) {
        let [c10, c01, c11] = get_other_corners(position, size);

        self.render_triangle(position, c10, c11, color);
        self.render_triangle(position, c11, c01, color);
    }
}
