use core::fmt;
use core::fmt::{Debug, Display, Formatter};
use core::option::Option::{None, Some};
use core::result::Result;
use core::result::Result::{Err, Ok};
use std::collections::{HashMap, HashSet};
use std::error::Error;

use enum_iterator::IntoEnumIterator;

use placement_area::PlacementArea;

use crate::coordinate::{flip_coordinate, rotate_coordinate, Coordinate};
use crate::game_board::Color;
use crate::ruleset::board_type::space::Space;
use crate::ruleset::piece_definition::PieceDefinition;
use crate::ruleset::starting_positions::alteration_type::{AlterationTypeError, AlternationType};
use crate::ruleset::starting_positions::piece_limit::{PieceLimit, PieceLimitError};
use crate::ruleset::starting_positions::placement_area::PlacementAreaError;
use crate::ruleset::{BoardType, Ruleset};

pub mod alteration_type;
pub mod piece_limit;
pub mod placement_area;

/// Defines the starting positions
#[derive(Clone, Debug)]
pub enum StartingPositions {
    /// Mirrored start positions, only defines a single side.
    /// Mirror will flip about horizontal center.
    /// Will error if overlapping.
    MirroredFlipped(HashMap<usize, Vec<Coordinate>>),
    /// Mirrored start positions, only defines a single side.
    /// Mirror will rotate.
    /// Will error if overlapping.
    MirroredRotated(HashMap<usize, Vec<Coordinate>>),
    /// Start positions for all colors.
    /// Will error if overlapping.
    /// All colors must be set.
    NotMirrored(HashMap<Color, HashMap<usize, Vec<Coordinate>>>),
    /// Players will alternate placing pieces.
    Placement {
        /// The color to go first.
        first_color: Color,
        /// The way turns are alternated.
        alternation_type: AlternationType,
        /// The valid placement area.
        placement_area: PlacementArea,
        /// The limitations on piece placement.
        piece_limits: HashSet<PieceLimit>,
    },
}
impl StartingPositions {
    fn verify_mirrored_flipped(
        piece_positions: &HashMap<usize, Vec<Coordinate>>,
        board: &BoardType,
        ruleset: &Ruleset,
    ) -> StartingPositionsResult<()> {
        // Tracks already used positions
        let mut found = HashSet::new();
        for (&piece_index, positions) in piece_positions {
            let piece = match ruleset.get_piece(piece_index) {
                None => return Err(StartingPositionsError::PieceIndexNotFound(piece_index)),
                Some(piece) => piece,
            };
            for &position in positions {
                // Check already used positions and add to list
                if !found.insert(position) {
                    return Err(StartingPositionsError::DuplicatePosition {
                        piece: piece.clone(),
                        position,
                    });
                }

                match board.get_space(position) {
                    Space::Normal => {}
                    space => {
                        return Err(StartingPositionsError::InvalidPositionForBoard {
                            space,
                            piece: piece.clone(),
                            position,
                        });
                    }
                }
                match board.get_space(flip_coordinate(board, position)) {
                    Space::Normal => {}
                    space => {
                        return Err(StartingPositionsError::InvalidPositionForBoard {
                            space,
                            piece: piece.clone(),
                            position,
                        });
                    }
                }
            }
        }
        Ok(())
    }
    fn verify_mirrored_rotated(
        piece_positions: &HashMap<usize, Vec<Coordinate>>,
        board: &BoardType,
        ruleset: &Ruleset,
    ) -> StartingPositionsResult<()> {
        // Tracks already used positions
        let mut found = HashSet::new();
        for (&piece_index, positions) in piece_positions {
            let piece = match ruleset.get_piece(piece_index) {
                None => return Err(StartingPositionsError::PieceIndexNotFound(piece_index)),
                Some(piece) => piece,
            };
            for &position in positions {
                // Check already used positions and add to list
                if !found.insert(position) {
                    return Err(StartingPositionsError::DuplicatePosition {
                        piece: piece.clone(),
                        position,
                    });
                }

                match board.get_space(position) {
                    Space::Normal => {}
                    space => {
                        return Err(StartingPositionsError::InvalidPositionForBoard {
                            space,
                            piece: piece.clone(),
                            position,
                        });
                    }
                }
                match board.get_space(rotate_coordinate(board, position)) {
                    Space::Normal => {}
                    space => {
                        return Err(StartingPositionsError::InvalidPositionForBoard {
                            space,
                            piece: piece.clone(),
                            position,
                        });
                    }
                }
            }
        }
        Ok(())
    }
    fn verify_not_mirrored(
        color_piece_positions: &HashMap<Color, HashMap<usize, Vec<Coordinate>>>,
        input: &BoardType,
        ruleset: &Ruleset,
    ) -> StartingPositionsResult<()> {
        // Tracks already used positions
        let mut found = HashSet::new();
        for color in Color::into_enum_iter() {
            let piece_positions = match color_piece_positions.get(&color) {
                Some(piece_positions) => piece_positions,
                None => return Err(StartingPositionsError::ColorNotFound(color)),
            };
            for (&piece_index, positions) in piece_positions {
                let piece = match ruleset.get_piece(piece_index) {
                    None => return Err(StartingPositionsError::PieceIndexNotFound(piece_index)),
                    Some(piece) => piece,
                };
                for &position in positions {
                    // Check already used positions and add to list
                    if !found.insert(position) {
                        return Err(StartingPositionsError::DuplicatePosition {
                            piece: piece.clone(),
                            position,
                        });
                    }

                    match input.get_space(position) {
                        Space::Normal => {}
                        space => {
                            return Err(StartingPositionsError::InvalidPositionForBoard {
                                space,
                                piece: piece.clone(),
                                position,
                            });
                        }
                    }
                }
            }
        }
        Ok(())
    }
    fn verify_placement(
        _: Color,
        alternation_type: AlternationType,
        placement_area: &PlacementArea,
        piece_limits: &HashSet<PieceLimit>,
        board: &BoardType,
        ruleset: &Ruleset,
    ) -> Result<(), StartingPositionsError> {
        alternation_type.verify(piece_limits)?;
        placement_area.verify(board)?;
        PieceLimit::verify(piece_limits, ruleset)?;
        Ok(())
    }

    pub fn verify(&self, board: &BoardType, ruleset: &Ruleset) -> StartingPositionsResult<()> {
        match self {
            StartingPositions::MirroredFlipped(self_data) => {
                Self::verify_mirrored_flipped(self_data, board, ruleset)
            }
            StartingPositions::MirroredRotated(self_data) => {
                Self::verify_mirrored_rotated(self_data, board, ruleset)
            }
            StartingPositions::NotMirrored(positions) => {
                Self::verify_not_mirrored(positions, board, ruleset)
            }
            StartingPositions::Placement {
                first_color,
                alternation_type,
                placement_area,
                piece_limits,
            } => Self::verify_placement(
                *first_color,
                *alternation_type,
                placement_area,
                piece_limits,
                board,
                ruleset,
            ),
        }
    }
}

pub type StartingPositionsResult<T> = Result<T, StartingPositionsError>;
#[derive(Clone, Debug)]
pub enum StartingPositionsError {
    /// Color was not set
    ColorNotFound(Color),
    /// Piece index was not found
    PieceIndexNotFound(usize),
    /// Position duplicate found
    DuplicatePosition {
        piece: PieceDefinition,
        position: Coordinate,
    },
    /// Position invalid
    InvalidPositionForBoard {
        space: Space,
        piece: PieceDefinition,
        position: Coordinate,
    },
    AlterationTypeError(AlterationTypeError),
    PlacementAreaError(PlacementAreaError),
    PieceLimitError(PieceLimitError),
}
impl Display for StartingPositionsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}
impl Error for StartingPositionsError {
    fn cause(&self) -> Option<&dyn Error> {
        match self {
            StartingPositionsError::ColorNotFound(_) => None,
            StartingPositionsError::PieceIndexNotFound(_) => None,
            StartingPositionsError::DuplicatePosition { .. } => None,
            StartingPositionsError::InvalidPositionForBoard { .. } => None,
            StartingPositionsError::AlterationTypeError(error) => Some(error),
            StartingPositionsError::PlacementAreaError(error) => Some(error),
            StartingPositionsError::PieceLimitError(error) => Some(error),
        }
    }
}
impl From<AlterationTypeError> for StartingPositionsError {
    fn from(from: AlterationTypeError) -> Self {
        Self::AlterationTypeError(from)
    }
}
impl From<PlacementAreaError> for StartingPositionsError {
    fn from(from: PlacementAreaError) -> Self {
        Self::PlacementAreaError(from)
    }
}
impl From<PieceLimitError> for StartingPositionsError {
    fn from(from: PieceLimitError) -> Self {
        Self::PieceLimitError(from)
    }
}
