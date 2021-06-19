use crate::math::color::Color;
use crate::math::point::Point;
use noise::{NoiseFn, Perlin};
use std::f32::consts::PI;

#[derive(Clone, Debug)]
pub enum ColorSelector {
    /// The same color for all pixel.
    ConstantColor(Color),
    /// Uses a noise function to interpolate between 2 colors.
    Noise {
        color0: Color,
        color1: Color,
        noise: Box<Perlin>,
        scale: f64,
    },
    WoodRings {
        center: Point,
        ring_size: f32,
        wood: Color,
        growth_ring: Color,
        noise: Box<Perlin>,
        noise_amplitude: f32,
        noise_scale: f64,
    },
}

impl ColorSelector {
    pub fn select(&self, point: &Point) -> Color {
        match self {
            ColorSelector::ConstantColor(color) => *color,
            ColorSelector::Noise {
                color0,
                color1,
                noise,
                scale,
            } => {
                let x = point.x as f64 / scale;
                let y = point.y as f64 / scale;
                let factor = noise.get([x, y]);
                color0.lerp(color1, factor as f32)
            }
            ColorSelector::WoodRings {
                center,
                ring_size,
                wood,
                growth_ring,
                noise,
                noise_amplitude,
                noise_scale,
            } => {
                let x = point.x as f64 / *noise_scale;
                let y = point.y as f64 / *noise_scale;
                let n = noise.get([x, y]) as f32 * *noise_amplitude;
                let distance = center.calculate_distance(point);
                let factor = (1.0 + (2.0 * PI * (distance / *ring_size + n)).sin()) / 2.0;
                wood.lerp(growth_ring, factor)
            }
        }
    }
}

impl PartialEq for ColorSelector {
    fn eq(&self, other: &Self) -> bool {
        match self {
            ColorSelector::ConstantColor(color) => match other {
                ColorSelector::ConstantColor(other_color) => color.eq(other_color),
                _ => false,
            },
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::color::RED;

    #[test]
    fn test_constant_color() {
        assert_eq!(
            ColorSelector::ConstantColor(RED).select(&Point::new(1, 2)),
            RED
        );
    }
}
