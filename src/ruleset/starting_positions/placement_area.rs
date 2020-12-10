use std::collections::{HashMap, HashSet};
use std::error::Error;

use bitflags::_core::fmt::{Debug, Display, Formatter};
use bitflags::_core::fmt;

use crate::game_board::Color;
use crate::ruleset::{BoardType, flip_coordinate};

/// Placement area definition.
#[derive(Clone, Debug)]
pub enum PlacementArea {
    /// Players can place on half the board.
    Half,
    /// Players can place on a mirrored set of places.
    /// Mirroring will flip.
    /// Will error if overlapping.
    MirroredFlipped(Vec<(usize, usize)>),
    /// Players can place on a mirrored set of places.
    /// Mirroring will rotate.
    /// Will error if overlapping.
    MirroredRotated(Vec<(usize, usize)>),
    /// Players can place on a given set of places based on color.
    /// Must be set for all colors.
    NonMirrored(HashMap<Color, Vec<(usize, usize)>>),
}
impl PlacementArea {
    fn verify(&self, board: &BoardType) -> PlacementAreaResult<()> {
        match self {
            Self::MirroredFlipped(positions) => {
                let mut found = HashSet::with_capacity(positions.len() * 2);
                for position in positions {
                    if !found.insert(*position) || found.insert(flip_coordinate(board, *position)) {
                        return Err(PlacementAreaError::PositionCollision(position));
                    }
                }
                Ok(())
            }
        }
    }
}
pub type PlacementAreaResult<T> = Result<T, PlacementAreaError>;
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum PlacementAreaError {
    PositionCollision((usize, usize)),
}
impl Display for PlacementAreaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}
impl Error for PlacementAreaError {}
