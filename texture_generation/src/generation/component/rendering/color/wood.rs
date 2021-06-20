use crate::generation::data::Data;
use crate::generation::random::Random;
use crate::math::color::Color;
use crate::math::point::Point;
use anyhow::{bail, Result};
use noise::{NoiseFn, Perlin, Seedable};

#[derive(Clone, Debug, PartialEq)]
pub struct WoodRing {
    pub color: Color,
    pub color_variation: f32,
    pub ring_size: u32,
    pub ring_size_variation: u32,
}

impl WoodRing {
    pub fn new(
        color: Color,
        color_variation: f32,
        ring_size: u32,
        ring_size_variation: u32,
    ) -> Result<WoodRing> {
        if color_variation <= 0.0 {
            bail!("Argument 'color_variation' needs to be greater than 0");
        } else if color_variation > 0.5 {
            bail!("Argument 'color_variation' needs to be less than 0.5");
        }

        Ok(WoodRing {
            color,
            color_variation,
            ring_size,
            ring_size_variation,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WoodFactory {
    early_wood: WoodRing,
    late_wood: WoodRing,
    noise_amplitude: f32,
    noise_scale: f64,
}

impl WoodFactory {
    pub fn new(
        early_wood: WoodRing,
        late_wood: WoodRing,
        noise_amplitude: f32,
        noise_scale: f64,
    ) -> WoodFactory {
        WoodFactory {
            early_wood,
            late_wood,
            noise_amplitude,
            noise_scale,
        }
    }

    pub fn create(&self, data: &Data) -> WoodSelector {
        let aabbs = data.get_aabbs_in_texture_space();
        let aabb = aabbs.get_inner();
        let center = aabb.center();
        let noise = Perlin::new().set_seed(data.get_instance_id() as u32);
        let diff = aabb.end() - center;
        let ring_sizes = self.calculate_ring_sizes(data, (diff.x + diff.y) as u32);

        WoodSelector {
            ring_sizes,
            early_wood_color: self.early_wood.color,
            late_wood_color: self.late_wood.color,
            noise: Box::new(noise),
            noise_amplitude: self.noise_amplitude,
            noise_scale: self.noise_scale,
        }
    }

    pub fn calculate_ring_sizes(&self, data: &Data, max_distance: u32) -> Vec<(f32, f32)> {
        let random = Random::Hash;
        let mut ring_sizes = Vec::new();

        let mut i = 0;
        let mut distance = 0;
        let mut is_early = true;

        while distance < max_distance {
            let definition = if is_early {
                &self.early_wood
            } else {
                &self.late_wood
            };
            let color_variation =
                random.get_random_instance_f32(data, definition.color_variation, i);
            let mut ring_size = definition.ring_size;

            if definition.ring_size_variation > 0 {
                ring_size +=
                    random.get_random_instance_u32(data, definition.ring_size_variation, i);
            }

            distance += ring_size;
            ring_sizes.push((color_variation, ring_size as f32));
            is_early = !is_early;
            i += 1;
        }

        ring_sizes
    }
}

#[derive(Clone, Debug)]
pub struct WoodSelector {
    ring_sizes: Vec<(f32, f32)>,
    early_wood_color: Color,
    late_wood_color: Color,
    noise: Box<Perlin>,
    noise_amplitude: f32,
    noise_scale: f64,
}

impl WoodSelector {
    pub fn select(&self, point: &Point, distance: f32) -> Color {
        let n = get_noise(&self.noise, point, self.noise_scale, self.noise_amplitude);
        let distance = distance + n;
        let mut ring_start = 0.0;
        let mut is_early = true;
        let mut factor = 0.0;

        for (color_variation, ring_size) in &self.ring_sizes {
            let ring_end = ring_start + *ring_size;

            if distance < ring_end {
                if is_early {
                    factor = *color_variation;
                } else {
                    factor = 1.0 - *color_variation;
                }
                break;
            }

            ring_start = ring_end;
            is_early = !is_early;
        }

        self.early_wood_color.lerp(&self.late_wood_color, factor)
    }
}

fn get_noise(noise: &Box<Perlin>, point: &Point, scale: f64, amplitude: f32) -> f32 {
    let x = point.x as f64 / scale;
    let y = point.y as f64 / scale;
    noise.get([x, y]) as f32 * amplitude
}
