use core::cmp::{Eq, PartialEq};
use core::fmt::{Debug, Display, Formatter};
use core::fmt;
use core::hash::{Hash, Hasher};
use core::mem::discriminant;
use core::result::Result;
use core::result::Result::{Err, Ok};
use std::collections::{HashMap, HashSet};
use std::error::Error;

use crate::ruleset::piece_definition::PieceDefinition;
use crate::ruleset::Ruleset;
use crate::ruleset::starting_positions::piece_limit::PieceLimitError::PieceHasNoPointValue;

/// Limits for piece placement.
///
/// Hash, Eq, PartialEq are defined for the discriminant.
#[derive(Clone, Debug)]
pub enum PieceLimit {
    /// Limit to the total count of pieces.
    TotalLimit {
        limit: usize,
    },
    /// Limit to each type of piece.
    TypeCountLimit {
        /// Limits not set for a piece will be infinite.
        /// Maps from pieces index to limit
        limits: HashMap<usize, usize>,
    },
    /// Limit by point count and total points available.
    PointLimit {
        /// Must be set for all pieces.
        /// Maps from pieces index to points value
        point_values: HashMap<usize, usize>,
        /// The total limit for each side.
        point_limit: usize,
    },
}
impl PieceLimit {
    pub fn verify(self_set: &HashSet<Self>, ruleset: &Ruleset) -> PieceLimitResult<()> {
        for piece_limit in self_set {
            match piece_limit {
                PieceLimit::TotalLimit { limit } => if *limit == 0 {
                    return Err(PieceLimitError::LimitIs0);
                },
                PieceLimit::TypeCountLimit { limits } => for (&piece_index, &limit) in limits {
                    let piece = match ruleset.get_piece(piece_index) {
                        None => return Err(PieceLimitError::PieceIndexNotFound(piece_index)),
                        Some(piece) => piece,
                    };
                    if limit == 0 {
                        return Err(PieceLimitError::LimitIs0ForPiece(piece.clone()));
                    }
                },
                PieceLimit::PointLimit { point_values, point_limit: _ } => {
                    for (piece_index, definition) in ruleset.pieces.iter().enumerate() {
                        match point_values.get(&piece_index) {
                            None => return Err(PieceLimitError::PieceHasNoPointValue(definition.clone())),
                            Some(points) => if *points == 0 {
                                return Err(PieceHasNoPointValue(definition.clone()));
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
impl Hash for PieceLimit {
    fn hash<H: Hasher>(&self, state: &mut H) {
        discriminant(self).hash(state);
    }
}
impl PartialEq for PieceLimit {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self).eq(&discriminant(other))
    }
}
impl Eq for PieceLimit {}

pub type PieceLimitResult<T> = Result<T, PieceLimitError>;
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum PieceLimitError {
    LimitIs0,
    PieceIndexNotFound(usize),
    LimitIs0ForPiece(PieceDefinition),
    PointsIs0ForPiece(PieceDefinition),
    PieceHasNoPointValue(PieceDefinition),
}
impl Display for PieceLimitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}
impl Error for PieceLimitError {}
