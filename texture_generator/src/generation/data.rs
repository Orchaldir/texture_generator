use crate::math::color::Color;
use crate::math::point::Point;
use crate::math::size::Size;

/// A trait used to store the data during the generation of the texture.
pub trait Data {
    /// Gets the [`Size`] of the textures.
    fn get_size(&self) -> &Size;

    /// Sets the [`Color`] & depth at the [`Point`].
    fn set(&mut self, point: &Point, color: &Color, depth: u8);

    /// Gets all the r, g & b values.
    fn get_color_data(&self) -> &[u8];

    /// Gets all the depth values.
    fn get_depth_data(&self) -> &[u8];
}

/// An implementation of [`Data`] for the actual usage.
pub struct RuntimeData {
    size: Size,
    colors: Vec<u8>,
    depth: Vec<u8>,
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

        let depth = vec![0; n];

        RuntimeData {
            size,
            colors,
            depth,
        }
    }
}

impl Data for RuntimeData {
    fn get_size(&self) -> &Size {
        &self.size
    }

    fn set(&mut self, point: &Point, color: &Color, depth: u8) {
        let index = self.size.to_index_risky(point);

        self.depth[index] = depth;

        let index = index * 3;

        self.colors[index] = color.r();
        self.colors[index + 1] = color.g();
        self.colors[index + 2] = color.b();
    }

    fn get_color_data(&self) -> &[u8] {
        &self.colors
    }

    fn get_depth_data(&self) -> &[u8] {
        &self.depth
    }
}

/// An implementation of [`Data`] for testing.
pub struct TestData {
    size: Size,
    colors: Vec<Color>,
    depth: Vec<u8>,
}

impl TestData {
    pub fn new(size: Size, default: Color) -> TestData {
        let n = size.get_number_of_cells();
        let colors = vec![default; n];
        let depth = vec![0; n];

        TestData {
            size,
            colors,
            depth,
        }
    }

    pub fn get_colors(&self) -> &[Color] {
        &self.colors
    }
}

impl Data for TestData {
    fn get_size(&self) -> &Size {
        &self.size
    }

    fn set(&mut self, point: &Point, color: &Color, depth: u8) {
        let index = self.size.to_index_risky(point);
        self.colors[index] = *color;
        self.depth[index] = depth;
    }

    fn get_color_data(&self) -> &[u8] {
        unimplemented!()
    }

    fn get_depth_data(&self) -> &[u8] {
        &self.depth
    }
}
