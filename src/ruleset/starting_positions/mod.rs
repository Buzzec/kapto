use core::fmt::{Debug, Display, Formatter};
use core::fmt;
use core::option::Option::{None, Some};
use core::result::Result::{Err, Ok};
use core::result::Result;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::mem::discriminant;

use placement_area::PlacementArea;

use crate::coordinate::Coordinate;
use crate::game_board::Color;
use crate::ruleset::{BoardType, flip_coordinate, rotate_coordinate, Ruleset, Space};
use crate::ruleset::piece_definition::PieceDefinition;
use crate::ruleset::starting_positions::alteration_type::AlternationType;

pub mod alteration_type;
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
        piece_limits: HashSet<PieceLimits>,
    },
}
impl StartingPositions {
    fn verify_mirrored_flipped(piece_positions: &HashMap<usize, Vec<Coordinate>>, board: &BoardType, ruleset: &Ruleset) -> StartingPositionsResult<()> {
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
                match board.get_space(flip_coordinate(board, position.1)) {
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
    fn verify_mirrored_rotated(piece_positions: &HashMap<usize, Vec<Coordinate>>, board: &BoardType, ruleset: &Ruleset) -> StartingPositionsResult<()> {
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
                match board.get_space(rotate_coordinate(board, position.1)) {
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
    fn verify_not_mirrored(color_piece_positions: &HashMap<Color, HashMap<usize, Vec<Coordinate>>>, input: &BoardType, ruleset: &Ruleset) -> StartingPositionsResult<()> {
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
    fn verify_placement(first_color: Color, alternation_type: AlternationType, placement_area: &PlacementArea, piece_limits: &HashSet<PieceLimits>, board: &BoardType, ruleset: &Ruleset) -> Result<(), StartingPositionsError> {}

    fn verify(&self, board: &BoardType, ruleset: &Ruleset) -> StartingPositionsResult<()> {
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
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
}
impl Display for StartingPositionsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}
impl Error for StartingPositionsError {}

/// Limits for piece placement.
///
/// Hash, Eq, PartialEq are defined for the discriminant.
#[derive(Clone, Debug)]
pub enum PieceLimits {
    /// Limit to the total count of pieces.
    TotalLimit { limit: usize },
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
impl<'a> Verifiable for PieceLimits<'a> {
    type Input = HashSet<&'a PieceDefinition>;
    type Error = PieceLimitsVerifyError<'a>;

    fn verify(&self, input: Self::Input) -> Result<(), Self::Error> {
        match self {
            Self::PointLimit { point_values, point_limit: _ } => {
                for definition in input {
                    if !point_values.contains_key(definition) {
                        return Err(PieceLimitsVerifyError::PieceHasNoPointValue(definition));
                    }
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
impl<'a> Hash for PieceLimits<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        discriminant(self).hash(state);
    }
}
impl<'a> PartialEq for PieceLimits<'a> {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self).eq(&discriminant(other))
    }
}
impl<'a> Eq for PieceLimits<'a> {}
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum PieceLimitsVerifyError<'a> {
    PieceHasNoPointValue(&'a PieceDefinition),
}
impl<'a> Display for PieceLimitsVerifyError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}
impl<'a> Error for PieceLimitsVerifyError<'a> {}
