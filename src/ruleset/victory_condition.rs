use core::cmp::{Eq, PartialEq};
use core::hash::{Hash, Hasher};
use core::mem::discriminant;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fmt;
use std::vec::Vec;

use crate::ruleset::Ruleset;

/// How the game is won.
///
/// Hash, Eq, and PartialEq are based on the discriminate.
#[derive(Clone, Debug)]
pub enum VictoryCondition {
    /// Victory can be achieved by having a certain number of goals owned by pieces.
    /// Condition becomes impossible if player has less than amount pieces left.
    GoalCount {
        /// Goals that need to be occupied to achieve this condition.
        amount: usize,
        /// Piece indexes that count for occupying goals.
        valid_pieces: Vec<usize>,
    },
    /// Victory can be achieved by capturing all of your opponents pieces.
    AllCaptured,
    /// Victory can be achieved by having a non-captured point difference.
    PointDifference(usize),
}
impl VictoryCondition {
    pub fn verify(&self, ruleset: &Ruleset) -> VictoryConditionResult<()> {
        match self {
            VictoryCondition::GoalCount { amount, valid_pieces } => {
                if *amount == 0 {
                    return Err(VictoryConditionError::AmountIs0);
                }
                if valid_pieces.is_empty() {
                    return Err(VictoryConditionError::NoValidPieces);
                }
                for &piece_index in valid_pieces {
                    match ruleset.get_piece(piece_index) {
                        None => return Err(VictoryConditionError::PieceNotFound(piece_index)),
                        Some(_) => {}
                    };
                }
                if !ruleset.board_type.has_goal() {
                    return Err(VictoryConditionError::BoardHasNoGoal);
                }
            }
            VictoryCondition::AllCaptured => {}
            VictoryCondition::PointDifference(difference) => if *difference == 0 {
                return Err(VictoryConditionError::PointDifferenceIs0);
            }
        }
        Ok(())
    }
}
impl Hash for VictoryCondition {
    fn hash<H: Hasher>(&self, state: &mut H) {
        discriminant(self).hash(state);
    }
}
impl PartialEq for VictoryCondition {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self).eq(&discriminant(other))
    }
}
impl Eq for VictoryCondition {}

pub type VictoryConditionResult<T> = Result<T, VictoryConditionError>;
#[derive(Copy, Clone, Debug)]
pub enum VictoryConditionError {
    AmountIs0,
    NoValidPieces,
    BoardHasNoGoal,
    PieceNotFound(usize),
    PointDifferenceIs0,
}
impl Display for VictoryConditionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}
impl Error for VictoryConditionError {}
