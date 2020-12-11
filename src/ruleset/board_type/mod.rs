use core::fmt::{Debug, Display, Formatter};
use core::fmt;
use core::result::Result::{Err, Ok};
use core::result::Result;
use std::error::Error;
use std::ops::Index;
use std::vec::Vec;

use matrix::format::conventional::Conventional;

use crate::coordinate::Coordinate;
use crate::game_board::Color;
use crate::ruleset::board_type::space::Space;

pub mod space;

/// A board definition
#[derive(Clone, Debug)]
pub enum BoardType {
    /// Rectangular board of size (rows, columns) with goals in columns defined by goal_locations.
    /// All goal locations must be < columns.
    /// Red on top, Blue on bottom
    Rectangular {
        /// Must be >= 1 and <= `u8::max_value() - 2`.
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
                if *rows < 1 || *rows > u8::max_value() - 2 {
                    Err(BoardTypeVerifyError::InvalidRows(*rows as usize))
                } else if *columns < 2 {
                    Err(BoardTypeVerifyError::InvalidColumns(*columns as usize))
                } else {
                    for &location in goal_locations {
                        if location >= *columns {
                            return Err(BoardTypeVerifyError::InvalidGoalLocation(location as usize));
                        }
                    }
                    Ok(())
                }
            }
            BoardType::Custom(board) => {
                if board.rows > u8::max_value() as usize {
                    return Err(BoardTypeVerifyError::InvalidRows(board.rows));
                }
                if board.columns > u8::max_value() as usize {
                    return Err(BoardTypeVerifyError::InvalidColumns(board.columns));
                }
                Ok(())
            }
        }
    }

    pub fn get_space(&self, coordinate: Coordinate) -> Space {
        match self {
            BoardType::Rectangular {
                rows,
                columns,
                goal_locations,
            } => {
                if coordinate.row >= (rows + 2) as i16 || coordinate.column >= *columns as i16 {
                    Space::Invalid
                } else if coordinate.row == 0 || coordinate.row == (rows + 1) as i16 {
                    if goal_locations.contains(&(coordinate.column as u8)) {
                        Space::Goal(if coordinate.row == 0 {
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
                if coordinate.row >= board.rows as i16 || coordinate.column >= board.columns as i16 {
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
            BoardType::Custom(board) => board.rows as u8,
        }
    }
    pub fn columns(&self) -> u8 {
        match self {
            BoardType::Rectangular { rows: _, columns, goal_locations: _ } => *columns,
            BoardType::Custom(board) => board.columns as u8,
        }
    }
    pub fn has_goal(&self) -> bool {
        if let Self::Rectangular { rows: _, columns: _, goal_locations } = self {
            return !goal_locations.is_empty();
        }
        for row in 0..self.rows() {
            for column in 0..self.columns() {
                if let Space::Goal(_) = self.get_space(Coordinate::new(row as i16, column as i16)) {
                    return true;
                }
            }
        }
        false
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
