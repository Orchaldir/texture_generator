use crate::math::color::Color;
use crate::math::point::Point;
use crate::math::size::Size;

/// A trait used to store the data during the generation of the texture.
pub trait Data {
    /// Gets the [`Size`] of the textures.
    fn get_size(&self) -> &Size;

    /// Sets the [`Color`] at the [`Point`].
    fn set(&mut self, point: &Point, color: &Color);

    /// Gets all the r, g & b values.
    fn get_color_data(&self) -> &[u8];
}

/// An implementation of [`Data`] for the actual usage.
pub struct RuntimeData {
    size: Size,
    colors: Vec<u8>,
}

impl RuntimeData {
    pub fn new(size: Size, default: Color) -> RuntimeData {
        let n = size.get_number_of_cells();
        let mut colors = Vec::with_capacity(n * 3);

        for _ in 0..n {
            colors.push(default.r());
            colors.push(default.g());
            colors.push(default.b());
        }

        RuntimeData { size, colors }
    }
}

impl Data for RuntimeData {
    fn get_size(&self) -> &Size {
        &self.size
    }

    fn set(&mut self, point: &Point, color: &Color) {
        let index = self.size.to_index_risky(point) * 3;

        self.colors[index] = color.r();
        self.colors[index + 1] = color.g();
        self.colors[index + 2] = color.b();
    }

    fn get_color_data(&self) -> &[u8] {
        &self.colors
    }
}

/// An implementation of [`Data`] for testing.
pub struct TestData {
    size: Size,
    colors: Vec<Color>,
}

impl TestData {
    pub fn new(size: Size, default: Color) -> TestData {
        let n = size.get_number_of_cells();
        let colors = vec![default].into_iter().cycle().take(n).collect();

        TestData { size, colors }
    }

    pub fn get_colors(&self) -> &[Color] {
        &self.colors
    }
}

impl Data for TestData {
    fn get_size(&self) -> &Size {
        &self.size
    }

    fn set(&mut self, point: &Point, color: &Color) {
        let index = self.size.to_index_risky(point);
        self.colors[index] = *color;
    }

    fn get_color_data(&self) -> &[u8] {
        unimplemented!()
    }
}
