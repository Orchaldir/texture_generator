use crate::definition::convert;
use crate::generation::component::rendering::color::wood::WoodRing;
use crate::math::color::Color;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WoodRingDefinition {
    color: String,
    color_variation: f32,
    ring_size: u32,
    ring_size_variation: u32,
}

impl WoodRingDefinition {
    pub fn convert(&self, factor: f32) -> Result<WoodRing> {
        let color = Color::convert(&self.color)?;
        WoodRing::new(
            color,
            self.color_variation,
            convert(self.ring_size, factor),
            convert(self.ring_size_variation, factor),
        )
    }
}
