use self::Side::*;
use core::fmt;
use serde::{Deserialize, Serialize};
use std::slice::Iter;

/// Used for the 4 sides of a [`Tile`] or the orientation of an object.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Side {
    Top,
    Left,
    Bottom,
    Right,
}

impl Side {
    /// Iterates of all sides counter-clockwise.
    pub fn iterator() -> Iter<'static, Side> {
        static SIDES: [Side; 4] = [Top, Left, Bottom, Right];
        SIDES.iter()
    }

    pub fn is_straight(&self, other: Side) -> bool {
        match self {
            Top => other == Bottom,
            Left => other == Right,
            Bottom => other == Top,
            Right => other == Left,
        }
    }
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_straight() {
        assert!(Top.is_straight(Bottom));
        assert!(Left.is_straight(Right));
        assert!(Bottom.is_straight(Top));
        assert!(Right.is_straight(Left))
    }

    #[test]
    fn test_is_not_straight() {
        assert!(!Top.is_straight(Top));
        assert!(!Top.is_straight(Left));
        assert!(!Top.is_straight(Right));

        assert!(!Left.is_straight(Top));
        assert!(!Left.is_straight(Left));
        assert!(!Left.is_straight(Bottom));

        assert!(!Bottom.is_straight(Left));
        assert!(!Bottom.is_straight(Bottom));
        assert!(!Bottom.is_straight(Right));

        assert!(!Right.is_straight(Top));
        assert!(!Right.is_straight(Bottom));
        assert!(!Right.is_straight(Right));
    }
}
