use crate::generation::process::PostProcess;
use crate::math::color::Color;
use crate::math::point::Point;
use crate::math::size::Size;

/// A trait used to store the data during the generation of the texture.
pub trait Data {
    /// Gets the [`Size`] of the textures.
    fn get_size(&self) -> &Size;

    /// Sets the [`Color`] & depth at the [`Point`].
    fn set(&mut self, point: &Point, color: &Color, depth: u8);

    /// Gets all the colors.
    fn get_color_data(&self) -> &[Color];

    /// Gets all the colors mutable.
    fn get_color_data_mut(&mut self) -> &mut [Color];

    /// Gets all the depth values.
    fn get_depth_data(&self) -> &[u8];

    /// Gets the base depth for the current tile of the tilemap.
    fn get_base_depth(&self) -> u8;
}

/// An implementation of [`Data`] for the actual usage.
pub struct RuntimeData {
    size: Size,
    colors: Vec<Color>,
    depth: Vec<u8>,
    base_depth: u8,
}

impl RuntimeData {
    pub fn new(size: Size, default: Color) -> RuntimeData {
        RuntimeData::with_base_depth(size, default, 0)
    }

    pub fn with_base_depth(size: Size, default: Color, base_depth: u8) -> RuntimeData {
        let n = size.get_number_of_cells();
        let colors = vec![default; n];
        let depth = vec![0; n];

        RuntimeData {
            size,
            colors,
            depth,
            base_depth,
        }
    }

    pub fn set_base_depth(&mut self, depth: u8) {
        self.base_depth = depth;
    }

    /// Applies the post processes.
    pub fn apply(&mut self, post_processes: &[PostProcess]) {
        for post_process in post_processes.iter() {
            post_process.process(self);
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
        self.colors[index] = *color;
    }

    fn get_color_data(&self) -> &[Color] {
        &self.colors
    }

    fn get_color_data_mut(&mut self) -> &mut [Color] {
        &mut self.colors
    }

    fn get_depth_data(&self) -> &[u8] {
        &self.depth
    }

    fn get_base_depth(&self) -> u8 {
        self.base_depth
    }
}

pub fn convert(colors: &[Color]) -> Vec<u8> {
    let n = colors.len();
    let mut data = Vec::with_capacity(n * 3);

    for color in colors {
        data.push(color.r());
        data.push(color.g());
        data.push(color.b());
    }

    data
}
