/// Interpolates between 2 u8 linearly.
///
/// ```
///# use texture_generation::math::interpolate::lerp;
/// assert_eq!(lerp(100, 200, 0.0), 100);
/// assert_eq!(lerp(100, 200, 0.5), 150);
/// assert_eq!(lerp(100, 200, 1.0), 200);
/// ```
pub fn lerp(start: u8, end: u8, factor: f32) -> u8 {
    if factor > 1.0 {
        return end;
    }

    if end >= start {
        let diff = (end - start) as f32;
        return start + (diff * factor) as u8;
    }

    let diff = (start - end) as f32;

    start - (diff * factor) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lerp_from_high_to_low() {
        assert_eq!(lerp(200, 100, 0.0), 200);
        assert_eq!(lerp(200, 100, 0.5), 150);
        assert_eq!(lerp(200, 100, 1.0), 100);
    }

    #[test]
    fn test_lerp_with_negative_factor() {
        assert_eq!(lerp(100, 200, -0.5), 100);
        assert_eq!(lerp(200, 100, -0.5), 200);
    }

    #[test]
    fn test_lerp_with_too_high_factor() {
        assert_eq!(lerp(100, 200, 2.0), 200);
        assert_eq!(lerp(200, 100, 2.5), 100);
    }
}
