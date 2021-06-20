use crate::math::color::Color;
use crate::math::point::Point;
use noise::{NoiseFn, Perlin};

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
        ring_sizes: Vec<f32>,
        early_wood_color: Color,
        late_wood_color: Color,
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
                ring_sizes,
                early_wood_color,
                late_wood_color,
                noise,
                noise_amplitude,
                noise_scale,
            } => {
                let distance = center.calculate_distance(point);
                let mut ring_start = 0.0;
                let mut is_early = true;

                for ring_size in ring_sizes {
                    let ring_end = ring_start + *ring_size;

                    if distance < ring_end {
                        return if is_early {
                            *early_wood_color
                        } else {
                            *late_wood_color
                        };
                    }

                    ring_start = ring_end;
                    is_early = !is_early;
                }

                *early_wood_color
            }
        }
    }
}

fn get_noise(noise: &Box<Perlin>, point: &Point, scale: f64, amplitude: f32) -> f32 {
    let x = point.x as f64 / scale;
    let y = point.y as f64 / scale;
    noise.get([x, y]) as f32 * amplitude
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
