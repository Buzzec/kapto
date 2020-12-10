use core::fmt;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::mem::discriminant;
use std::ops::Index;

use matrix::Element;
use matrix::format::Conventional;

use crate::coordinate::Coordinate;
use crate::game_board::Color;
use crate::ruleset::piece_definition::{PieceDefinition, PieceDefinitionError};
use crate::ruleset::starting_positions::StartingPositions;

pub mod piece_definition;
pub mod standard;
pub mod starting_positions;

/// The ruleset for a game of Kapto
#[derive(Clone, Debug)]
pub struct Ruleset<'a> {
    /// All possible pieces
    pub pieces: Vec<PieceDefinition>,
    /// The type of board to use
    pub board_type: BoardType,
    /// Starting position type to use
    pub starting_positions: StartingPositions,
    /// How to win the game
    /// At least one must be set
    pub victory_conditions: HashSet<VictoryCondition<'a>>,
}
impl<'a> Ruleset<'a> {
    fn verify(&self) -> Result<(), Self::Error> {
        let mut pieces_set = HashSet::with_capacity(self.pieces.len());
        for piece in self.pieces {
            piece.verify()?;
            if !pieces_set.insert(&piece) {
                return Err(RulesetError::PieceDuplicated(piece.clone()));
            }
        }
        self.board_type.verify()?;
        Ok(())
    }

    pub fn get_piece(&self, index: usize) -> Option<&PieceDefinition> {
        self.pieces.get(index)
    }
}
pub type RulesetResult<T> = Result<T, RulesetError>;
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum RulesetError {
    PieceDuplicated(PieceDefinition),
    PieceDefinitionError(PieceDefinitionError),
    BoardTypeVerifyError(BoardTypeVerifyError),
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

/// A board definition
#[derive(Clone, Debug)]
pub enum BoardType {
    /// Rectangular board of size (rows, columns) with goals in columns defined by goal_locations.
    /// All goal locations must be < columns.
    /// Red on top, Blue on bottom
    Rectangular {
        /// Must be >= 1 and <= `usize::max_value() - 1`.
        rows: u8,
        /// Must be >= 2.
        columns: u8,
        /// All must be < columns.
        goal_locations: Vec<u8>,
    },
    /// Custom board definition.
    Custom(Conventional<Space>),
}
impl BoardType {
    pub fn verify(&self) -> BoardTypeVerifyResult<()> {
        match self {
            BoardType::Rectangular {
                rows,
                columns,
                goal_locations,
            } => {
                if *rows < 1 || *rows > u8::max_value() - 1 {
                    Err(Self::Error::InvalidRows(*rows))
                } else if *columns < 2 {
                    Err(Self::Error::InvalidColumns(*columns))
                } else {
                    for &location in goal_locations {
                        if location >= *columns {
                            return Err(Self::Error::InvalidGoalLocation(location));
                        }
                    }
                    Ok(())
                }
            }
            BoardType::Custom(_) => Ok(()),
        }
    }

    pub fn get_space(&self, coordinate: Coordinate) -> Space {
        match self {
            BoardType::Rectangular {
                rows,
                columns,
                goal_locations,
            } => {
                if coordinate.row >= rows + 2 as i16 || coordinate.column >= *columns as i16 {
                    Space::Invalid
                } else if coordinate.0 == 0 || coordinate.0 == rows + 1 {
                    if goal_locations.contains(&coordinate.1) {
                        Space::Goal(if coordinate.0 == 0 {
                            Color::Red
                        } else {
                            Color::Blue
                        })
                    } else {
                        Space::Invalid
                    }
                } else {
                    Space::Normal
                }
            }
            BoardType::Custom(board) => {
                if coordinate.0 >= board.rows || coordinate.1 >= board.columns {
                    Space::Invalid
                } else {
                    *board.index(coordinate)
                }
            }
        }
    }
    pub fn rows(&self) -> u8 {
        match self {
            BoardType::Rectangular { rows, columns: _, goal_locations: _ } => rows + 2,
            BoardType::Custom(board) => board.rows,
        }
    }
    pub fn columns(&self) -> u8 {
        match self {
            BoardType::Rectangular { rows: _, columns, goal_locations: _ } => *columns,
            BoardType::Custom(board) => board.columns,
        }
    }
}
pub type BoardTypeVerifyResult<T> = Result<T, BoardTypeVerifyError>;
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum BoardTypeVerifyError {
    InvalidRows(usize),
    InvalidColumns(usize),
    InvalidGoalLocation(usize),
}
impl Display for BoardTypeVerifyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}
impl Error for BoardTypeVerifyError {
    fn description(&self) -> &str {
        match self {
            BoardTypeVerifyError::InvalidRows(_) => "Invalid row size",
            BoardTypeVerifyError::InvalidColumns(_) => "Invalid column size",
            BoardTypeVerifyError::InvalidGoalLocation(_) => "Invalid goal location",
        }
    }
}

pub fn flip_coordinate(board: &BoardType, coordinate: Coordinate) -> Coordinate {
    Coordinate::new(board.rows() as i16 - coordinate.row - 1, coordinate.column)
}
pub fn rotate_coordinate(board: &BoardType, coordinate: Coordinate) -> Coordinate {
    Coordinate::new(board.rows() as i16 - coordinate.row - 1, board.columns() as i16 - coordinate.column - 1)
}

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

/// How the game is won.
///
/// Hash, Eq, and PartialEq are based on the discriminate.
#[derive(Clone, Debug)]
pub enum VictoryCondition<'a> {
    /// Victory can be achieved by having a certain number of goals owned by pieces.
    /// If this is not set goals are considered invalid spaces.
    GoalCount {
        /// Goals that need to be occupied to achieve this condition.
        amount: usize,
        /// Pieces that can occupy goals.
        valid_pieces: Vec<&'a PieceDefinition>,
    },
    /// Victory can be achieved by capturing all of your opponents pieces.
    AllCaptured,
    /// Victory can be achieved by having a non-captured point difference.
    PointDifference(usize),
}
impl<'a> Hash for VictoryCondition<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        discriminant(self).hash(state);
    }
}
impl<'a> PartialEq for VictoryCondition<'a> {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self).eq(&discriminant(other))
    }
}
impl<'a> Eq for VictoryCondition<'a> {}
