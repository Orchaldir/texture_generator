use crate::math::color::Color;
use std::cell::RefCell;
use std::ops::AddAssign;

#[derive(Clone, Debug, PartialEq)]
pub enum ColorSelector {
    /// Everything has the same color.
    ConstantColor(Color),
    /// A sequence of colors that repeats.
    Sequence(Vec<Color>, RefCell<usize>),
}

impl ColorSelector {
    pub fn new_sequence(colors: Vec<Color>) -> ColorSelector {
        ColorSelector::Sequence(colors, RefCell::new(0))
    }

    pub fn select(&self) -> Color {
        match self {
            ColorSelector::ConstantColor(color) => *color,
            ColorSelector::Sequence(colors, index) => {
                let current_index = *index.borrow() % colors.len();
                index.borrow_mut().add_assign(1);
                colors[current_index]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::color::{BLUE, GREEN, RED};

    #[test]
    fn test_constant() {
        let selector = ColorSelector::ConstantColor(RED);

        assert_eq!(selector.select(), RED);
        assert_eq!(selector.select(), RED);
        assert_eq!(selector.select(), RED);
    }

    #[test]
    fn test_uniform() {
        let selector = ColorSelector::new_sequence(vec![RED, GREEN, BLUE]);

        assert_eq!(selector.select(), RED);
        assert_eq!(selector.select(), GREEN);
        assert_eq!(selector.select(), BLUE);
        assert_eq!(selector.select(), RED);
        assert_eq!(selector.select(), GREEN);
        assert_eq!(selector.select(), BLUE);
    }
}
