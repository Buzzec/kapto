use matrix::Element;

use crate::game_board::Color;

/// A space for the board.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Space {
    /// Not a valid space
    Invalid,
    /// A normal space
    Normal,
    /// A goal space for a color
    Goal(Color),
}
impl Element for Space {
    fn zero() -> Self {
        Self::Normal
    }
}
