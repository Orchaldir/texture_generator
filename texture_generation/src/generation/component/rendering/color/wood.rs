use crate::math::color::Color;
use anyhow::{bail, Result};

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
