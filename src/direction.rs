use std::collections::HashSet;

use bitflags::_core::hash::Hash;
use bitflags::bitflags;

use crate::coordinate::Coordinate;

bitflags! {
    pub struct Directions: u8 {
        const North     = 0b00000001;
        const South     = 0b00000010;
        const East      = 0b00000100;
        const West      = 0b00001000;
        const NorthWest = 0b00010000;
        const NorthEast = 0b00100000;
        const SouthWest = 0b01000000;
        const SouthEast = 0b10000000;
        const Cardinal  = Self::North.bits | Self::South.bits | Self::East.bits | Self::West.bits;
        const Diagonal  = Self::NorthWest.bits | Self::NorthEast.bits | Self::SouthWest.bits | Self::SouthEast.bits;
        const All       = Self::Cardinal.bits | Self::Diagonal.bits;
        const None      = 0b00000000;
    }
}
impl From<Direction> for Directions {
    fn from(from: Direction) -> Self {
        match from {
            Direction::North => Self::North,
            Direction::South => Self::South,
            Direction::East => Self::East,
            Direction::West => Self::West,
            Direction::NorthWest => Self::NorthWest,
            Direction::NorthEast => Self::NorthEast,
            Direction::SouthWest => Self::SouthWest,
            Direction::SouthEast => Self::SouthEast,
        }
    }
}
impl Directions {
    fn run_for_all(self, function: impl Fn(Direction)) {
        if self.contains(Directions::North) { function(Direction::North); }
        if self.contains(Directions::South) { function(Direction::South); }
        if self.contains(Directions::East) { function(Direction::East); }
        if self.contains(Directions::West) { function(Direction::West); }
        if self.contains(Directions::NorthWest) { function(Direction::NorthWest); }
        if self.contains(Directions::NorthEast) { function(Direction::NorthEast); }
        if self.contains(Directions::SouthWest) { function(Direction::SouthWest); }
        if self.contains(Directions::SouthEast) { function(Direction::SouthEast); }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}
impl Direction {
    pub fn offset(&self) -> Coordinate {
        match self {
            Direction::North => Coordinate::new(0, -1),
            Direction::South => Coordinate::new(0, 1),
            Direction::East => Coordinate::new(1, 0),
            Direction::West => Coordinate::new(-1, 0),
            Direction::NorthWest => Coordinate::new(-1, -1),
            Direction::NorthEast => Coordinate::new(1, -1),
            Direction::SouthWest => Coordinate::new(-1, 1),
            Direction::SouthEast => Coordinate::new(1, 1),
        }
    }
}
impl From<Directions> for HashSet<Direction> {
    fn from(from: Directions) -> Self {
        let mut out = HashSet::new();
        from.run_for_all(|direction| {
            out.insert(direction);
        });
        out
    }
}
impl From<Directions> for Vec<Direction> {
    fn from(from: Directions) -> Self {
        let mut out = Self::new();
        from.run_for_all(|direction| out.push(direction));
        out
    }
}
