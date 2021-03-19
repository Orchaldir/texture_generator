pub mod generation;
pub mod math;

pub fn convert(value: u32, factor: f32) -> u32 {
    (value as f32 * factor) as u32
}
