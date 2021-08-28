use core::fmt;
use core::fmt::{Debug, Display, Formatter};
use core::result::Result;
use core::result::Result::{Err, Ok};
use std::error::Error;
use std::ops::Index;

use matrix::format::conventional::Conventional;

use crate::coordinate::Coordinate;
use crate::game_board::Color;
use crate::ruleset::board_type::space::Space;
use std::collections::HashSet;

pub mod space;

/// A board definition
#[derive(Clone, Debug)]
pub enum BoardType {
    /// Rectangular board of size (rows, columns) with goals in columns defined by goal_locations.
    /// All goal locations must be < columns.
    /// Red on top, Blue on bottom
    /// `
    ///             Red Goal
    /// (0,0)-------------------------(0,y)
    /// |                                 |
    /// |                                 |
    /// |                                 |
    /// |                                 |
    /// (x,0)-------------------------(x,y)
    ///             Blue Goal
    /// `
    Rectangular {
        /// Must be >= 1 and <= `u8::max_value() - 2`.
        rows: u8,
        /// Must be >= 2.
        columns: u8,
        /// All must be < columns.
        goal_locations: HashSet<u8>,
    },
    /// Custom board definition.
    Custom(Conventional<Space>),
}
impl BoardType {
    fn verify(&self) -> BoardTypeVerifyResult<()> {
        match self {
            BoardType::Rectangular {
                rows,
                columns,
                goal_locations,
            } => {
                if *rows < 1 || *rows > u8::MAX - 2 {
                    Err(BoardTypeVerifyError::InvalidRows(*rows as usize))
                } else if *columns < 2 {
                    Err(BoardTypeVerifyError::InvalidColumns(*columns as usize))
                } else {
                    for &location in goal_locations {
                        if location >= *columns {
                            return Err(BoardTypeVerifyError::InvalidGoalLocation(
                                location as usize,
                            ));
                        }
                    }
                    Ok(())
                }
            }
            BoardType::Custom(board) => {
                if board.rows > u8::MAX as usize {
                    return Err(BoardTypeVerifyError::InvalidRows(board.rows));
                }
                if board.columns > u8::MAX as usize {
                    return Err(BoardTypeVerifyError::InvalidColumns(board.columns));
                }
                Ok(())
            }
        }
    }

    pub fn into_matrix(self) -> Result<Conventional<Space>, (Self, BoardTypeVerifyError)> {
        match self.verify() {
            Ok(_) => match self {
                BoardType::Rectangular {
                    rows,
                    columns,
                    goal_locations,
                } => {
                    let mut out: Conventional<Space> =
                        Conventional::new((rows as usize + 2, columns as usize));

                    for x in 0..columns {
                        let is_goal = goal_locations.contains(&x);
                        out[(0, x as usize)] = if is_goal {
                            Space::Goal(Color::Red)
                        } else {
                            Space::Invalid
                        };
                        out[(rows as usize + 1, x as usize)] = if is_goal {
                            Space::Goal(Color::Blue)
                        } else {
                            Space::Invalid
                        };
                    }

                    Ok(out)
                }
                BoardType::Custom(out) => Ok(out),
            },
            Err(error) => Err((self, error)),
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
impl Error for BoardTypeVerifyError {}

#[cfg(test)]
mod test {
    use crate::ruleset::board_type::{BoardType, BoardTypeVerifyError};
    use std::collections::HashSet;
    #[test]
    fn verify_test() {
        assert_eq!(
            BoardType::Rectangular {
                rows: 0,
                columns: 2,
                goal_locations: HashSet::new(),
            }
            .verify(),
            Err(BoardTypeVerifyError::InvalidRows(0))
        );
        assert_eq!(
            BoardType::Rectangular {
                rows: 1,
                columns: 0,
                goal_locations: HashSet::new(),
            }
            .verify(),
            Err(BoardTypeVerifyError::InvalidColumns(0))
        );
        assert_eq!(
            BoardType::Rectangular {
                rows: 1,
                columns: 2,
                goal_locations: HashSet::new(),
            }
            .verify(),
            Ok(())
        )
    }
}
