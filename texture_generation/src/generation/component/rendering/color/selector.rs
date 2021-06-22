use crate::generation::component::rendering::color::wood::WoodSelector;
use crate::math::color::Color;
use crate::math::point::Point;
use noise::{NoiseFn, SuperSimplex};

#[derive(Clone, Debug)]
pub enum ColorSelector {
    /// The same color for all pixel.
    ConstantColor(Color),
    /// Uses a noise function to interpolate between 2 colors.
    Noise {
        color0: Color,
        color1: Color,
        noise: Box<SuperSimplex>,
        base_factor: f32,
        scale_x: f64,
        scale_y: f64,
    },
    WoodRings {
        center: Point,
        selector: WoodSelector,
    },
    WoodX {
        start_y: f32,
        selector: WoodSelector,
    },
    WoodY {
        start_x: f32,
        selector: WoodSelector,
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
                base_factor,
                scale_x,
                scale_y,
            } => {
                let x = point.x as f64 / scale_x;
                let y = point.y as f64 / scale_y;
                let factor = noise.get([x, y]);
                color0.lerp(color1, factor as f32 + *base_factor)
            }
            ColorSelector::WoodRings { center, selector } => {
                let distance = center.calculate_distance(point);
                selector.select(point, distance)
            }
            ColorSelector::WoodX { start_y, selector } => {
                selector.select(point, point.y as f32 - *start_y)
            }
            ColorSelector::WoodY { start_x, selector } => {
                selector.select(point, point.x as f32 - *start_x)
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
