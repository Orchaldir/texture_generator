use crate::math::size::Size;

pub mod generation;
pub mod math;

pub fn convert(value: u32, factor: f32) -> u32 {
    (value as f32 * factor) as u32
}

pub fn convert_size(value: &Size, factor: f32) -> Size {
    Size::new(
        convert(value.width(), factor),
        convert(value.height(), factor),
    )
}
