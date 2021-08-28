use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use enum_iterator::IntoEnumIterator;

use crate::coordinate::{flip_coordinate, rotate_coordinate, Coordinate};
use crate::game_board::Color;
use crate::ruleset::board_type::space::Space;
use crate::ruleset::board_type::BoardType;

/// Placement area definition.
#[derive(Clone, Debug)]
pub enum PlacementArea {
    /// Players can place on half the board.
    Half,
    /// Players can place on a mirrored set of places.
    /// Mirroring will flip.
    /// Will error if overlapping.
    MirroredFlipped(HashSet<Coordinate>),
    /// Players can place on a mirrored set of places.
    /// Mirroring will rotate.
    /// Will error if overlapping.
    MirroredRotated(HashSet<Coordinate>),
    /// Players can place on a given set of places based on color.
    /// Must be set for all colors.
    NonMirrored(HashMap<Color, HashSet<Coordinate>>),
}
impl PlacementArea {
    pub fn verify(&self, board: &BoardType) -> PlacementAreaResult<()> {
        match self {
            Self::Half => {}
            Self::MirroredFlipped(positions) | Self::MirroredRotated(positions) => {
                let func = if let Self::MirroredFlipped(_) = self {
                    flip_coordinate
                } else {
                    rotate_coordinate
                };
                let mut found = positions.clone();
                for &position in positions {
                    if position.row < 0
                        || position.row >= board.rows() as i16
                        || position.column < 0
                        || position.column >= board.columns() as i16
                    {
                        return Err(PlacementAreaError::PositionCannotPlace(
                            Space::Invalid,
                            position,
                        ));
                    }
                    let opposite = func(board, position);
                    if !found.insert(position) || found.insert(opposite) {
                        return Err(PlacementAreaError::PositionCollision(position));
                    }
                }
            }
            Self::NonMirrored(color_map) => {
                let mut found = HashSet::new();
                for color in Color::into_enum_iter() {
                    let coordinate_set = match color_map.get(&color) {
                        None => return Err(PlacementAreaError::ColorNotFound(color)),
                        Some(coordinate_set) => coordinate_set,
                    };
                    for &coordinate in coordinate_set {
                        if !found.insert(coordinate) {
                            return Err(PlacementAreaError::PositionCollision(coordinate));
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
pub type PlacementAreaResult<T> = Result<T, PlacementAreaError>;
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum PlacementAreaError {
    PositionCannotPlace(Space, Coordinate),
    PositionCollision(Coordinate),
    ColorNotFound(Color),
}
impl Display for PlacementAreaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}
impl Error for PlacementAreaError {}
