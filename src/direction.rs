use std::collections::HashSet;
use std::hash::Hash;

use bitflags::bitflags;

use crate::coordinate::Coordinate;

bitflags! {
    pub struct Directions: u8 {
        const NORTH         = 0b00000001;
        const SOUTH         = 0b00000010;
        const EAST          = 0b00000100;
        const WEST          = 0b00001000;
        const NORTH_WEST    = 0b00010000;
        const NORTH_EAST    = 0b00100000;
        const SOUTH_WEST    = 0b01000000;
        const SOUTH_EAST    = 0b10000000;
        const CARDINAL      = Self::NORTH.bits | Self::SOUTH.bits | Self::EAST.bits | Self::WEST.bits;
        const DIAGONAL      = Self::NORTH_WEST.bits | Self::NORTH_EAST.bits | Self::SOUTH_WEST.bits | Self::SOUTH_EAST.bits;
        const ALL           = Self::CARDINAL.bits | Self::DIAGONAL.bits;
        const NONE          = 0b00000000;
    }
}
impl From<Direction> for Directions {
    fn from(from: Direction) -> Self {
        match from {
            Direction::North => Self::NORTH,
            Direction::South => Self::SOUTH,
            Direction::East => Self::EAST,
            Direction::West => Self::WEST,
            Direction::NorthWest => Self::NORTH_WEST,
            Direction::NorthEast => Self::NORTH_EAST,
            Direction::SouthWest => Self::SOUTH_WEST,
            Direction::SouthEast => Self::SOUTH_EAST,
        }
    }
}
impl Directions {
    fn run_for_all(self, mut function: impl FnMut(Direction)) {
        if self.contains(Directions::NORTH) { function(Direction::North); }
        if self.contains(Directions::SOUTH) { function(Direction::South); }
        if self.contains(Directions::EAST) { function(Direction::East); }
        if self.contains(Directions::WEST) { function(Direction::West); }
        if self.contains(Directions::NORTH_WEST) { function(Direction::NorthWest); }
        if self.contains(Directions::NORTH_EAST) { function(Direction::NorthEast); }
        if self.contains(Directions::SOUTH_WEST) { function(Direction::SouthWest); }
        if self.contains(Directions::SOUTH_EAST) { function(Direction::SouthEast); }
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
