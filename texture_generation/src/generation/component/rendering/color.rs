use crate::math::color::Color;
use crate::math::point::Point;

#[derive(Clone, Debug, PartialEq)]
pub enum ColorSelector {
    /// The same color for all pixel.
    ConstantColor(Color),
}

impl ColorSelector {
    pub fn select(&self, _point: &Point) -> Color {
        match self {
            ColorSelector::ConstantColor(color) => *color,
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
