use crate::math::color::Color;
use crate::math::point::Point;
use crate::math::size::Size;

pub mod rendering;

pub trait RuntimeData {
    /// Set the [``Color`] at the point.
    fn set(&mut self, point: &Point, color: &Color);

    /// Get all color values.
    fn get_colors(&self) -> &[u8];
}

pub struct RuntimeDataImpl {
    size: Size,
    colors: Vec<u8>,
}

impl RuntimeDataImpl {
    pub fn new(size: Size, default: Color) -> RuntimeDataImpl {
        let n = size.get_number_of_cells();
        let colors = vec![default.r(), default.g(), default.b()]
            .into_iter()
            .cycle()
            .take(n * 3)
            .collect();

        RuntimeDataImpl { size, colors }
    }
}

impl RuntimeData for RuntimeDataImpl {
    fn set(&mut self, point: &Point, color: &Color) {
        let index = self.size.to_index_risky(point) * 3;

        self.colors[index] = color.r();
        self.colors[index + 1] = color.g();
        self.colors[index + 2] = color.b();
    }

    fn get_colors(&self) -> &[u8] {
        &self.colors
    }
}
