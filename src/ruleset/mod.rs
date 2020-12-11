use core::fmt;
use std::collections::HashSet;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use crate::ruleset::board_type::{BoardType, BoardTypeVerifyError};
use crate::ruleset::piece_definition::{PieceDefinition, PieceDefinitionError};
use crate::ruleset::starting_positions::{StartingPositions, StartingPositionsError};
use crate::ruleset::victory_condition::{VictoryCondition, VictoryConditionError};

pub mod starting_positions;

pub mod board_type;
pub mod piece_definition;
pub mod standard;
pub mod victory_condition;

/// The ruleset for a game of Kapto
#[derive(Clone, Debug)]
pub struct Ruleset {
    /// All possible pieces
    pub pieces: Vec<PieceDefinition>,
    /// The type of board to use
    pub board_type: BoardType,
    /// Starting position type to use
    pub starting_positions: StartingPositions,
    /// How to win the game
    /// At least one must be set
    pub victory_conditions: HashSet<VictoryCondition>,
}
impl Ruleset {
    fn verify(&self) -> RulesetResult<()> {
        let mut pieces_set = HashSet::with_capacity(self.pieces.len());
        for piece in self.pieces.iter() {
            piece.verify()?;
            if !pieces_set.insert(piece) {
                return Err(RulesetError::PieceDuplicated(piece.clone()));
            }
        }
        self.board_type.verify()?;
        self.starting_positions.verify(&self.board_type, self)?;
        for victory_condition in self.victory_conditions.iter() {
            victory_condition.verify(self)?;
        }
        Ok(())
    }

    pub fn get_piece(&self, index: usize) -> Option<&PieceDefinition> {
        self.pieces.get(index)
    }
}
pub type RulesetResult<T> = Result<T, RulesetError>;
#[derive(Clone, Debug)]
pub enum RulesetError {
    PieceDuplicated(PieceDefinition),
    PieceDefinitionError(PieceDefinitionError),
    BoardTypeVerifyError(BoardTypeVerifyError),
    StartingPositionsError(StartingPositionsError),
    VictoryConditionError(VictoryConditionError),
}
impl Display for RulesetError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}
impl Error for RulesetError {
    fn cause(&self) -> Option<&dyn Error> {
        match self {
            Self::PieceDuplicated(_) => None,
            Self::PieceDefinitionError(error) => Some(error),
            Self::BoardTypeVerifyError(error) => Some(error),
            Self::StartingPositionsError(error) => Some(error),
            Self::VictoryConditionError(error) => Some(error),
        }
    }
}
impl From<PieceDefinitionError> for RulesetError {
    fn from(from: PieceDefinitionError) -> Self {
        Self::PieceDefinitionError(from)
    }
}
impl From<BoardTypeVerifyError> for RulesetError {
    fn from(from: BoardTypeVerifyError) -> Self {
        Self::BoardTypeVerifyError(from)
    }
}
impl From<StartingPositionsError> for RulesetError {
    fn from(from: StartingPositionsError) -> Self {
        Self::StartingPositionsError(from)
    }
}
impl From<VictoryConditionError> for RulesetError {
    fn from(from: VictoryConditionError) -> Self {
        Self::VictoryConditionError(from)
    }
}
