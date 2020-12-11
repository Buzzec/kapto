use std::ops::{Index, IndexMut};

use enum_iterator::IntoEnumIterator;
use matrix::{Element, Position, Size};
use matrix::prelude::Conventional;

use crate::action::{Action, ActionError, ActionType};
use crate::action::ActionError::PieceOnMove;
use crate::coordinate::Coordinate;
use crate::direction::Direction;

#[derive(Clone, Debug)]
pub struct GameBoard {
    pub board: Conventional<BoardSpace>,
}
impl GameBoard {
    pub fn new<S: Size>(board_size: S, goal_pos: &[usize]) -> Self {
        assert!(!goal_pos.is_empty(), "Must have at least 1 goal position");
        let rows = board_size.rows() + 2;
        let columns = board_size.columns();
        assert!(rows >= 1, "Rows must be >= 1");
        assert!(columns >= 2, "Columns must be >= 2");
        let mut board = Conventional::new((rows, columns));
        for index in 0..columns {
            if !goal_pos.contains(&index) {
                *board.index_mut((0, index)) = BoardSpace::Invalid;
                *board.index_mut((rows - 1, index)) = BoardSpace::Invalid;
            }
        }
        Self { board }
    }

    pub fn is_valid_position(&self, position: impl Position) -> bool {
        self.board.columns > position.column()
            && self.board.rows > position.row()
            && self.board.index(position) != &BoardSpace::Invalid
    }
    fn check_valid_position(&self, position: impl Position) -> GameBoardResult<()> {
        if self.is_valid_position(position) {
            Ok(())
        } else {
            Err(GameBoardError::InvalidPosition)
        }
    }

    pub fn pieces_of_size(&self, size: PieceSize) -> Vec<(impl Position, Piece)> {
        let mut out = Vec::new();
        for (index, space) in self.board.values.iter().enumerate() {
            match space {
                BoardSpace::Normal(piece) | BoardSpace::Goal { goal_for: _, piece } => {
                    if let Some(piece) = piece {
                        if piece.size() == size {
                            out.push((index_to_position(&self.board, index), *piece));
                        }
                    }
                }
                _ => {}
            }
        }
        out
    }
    pub fn pieces_of_color(&self, color: Color) -> Vec<(impl Position, Piece)> {
        let mut out = Vec::new();
        for (index, space) in self.board.values.iter().enumerate() {
            match space {
                BoardSpace::Normal(piece) | BoardSpace::Goal { goal_for: _, piece } => {
                    if let Some(piece) = piece {
                        if piece.color() == color {
                            out.push((index_to_position(&self.board, index), *piece));
                        }
                    }
                }
                _ => {}
            }
        }
        out
    }

    pub fn piece(&self, position: impl Position + Copy) -> GameBoardResult<Option<Piece>> {
        self.check_valid_position(position)?;
        match self.board.index(position) {
            BoardSpace::Normal(piece) | BoardSpace::Goal { goal_for: _, piece } => Ok(*piece),
            _ => unreachable!("Should have been checked with check_valid_position"),
        }
    }
    pub fn piece_mut(
        &mut self,
        position: impl Position + Copy,
    ) -> GameBoardResult<&mut Option<Piece>> {
        self.check_valid_position(position)?;
        match self.board.index_mut(position) {
            BoardSpace::Normal(piece) | BoardSpace::Goal { goal_for: _, piece } => Ok(piece),
            _ => unreachable!("Should have been checked with check_valid_position"),
        }
    }

    pub fn apply_action(
        &self,
        action: &Action,
        capture_callback: impl Fn(Coordinate, Piece),
    ) -> Result<GameBoard, ActionError> {
        self.is_valid_action(action)?;
        let mut board = self.clone();
        let piece_start = board.piece_mut(action.start_pos).unwrap();
        let piece = piece_start.unwrap();
        *piece_start = None;

        match &action.action_type {
            ActionType::Move(direction) => {
                *board.piece_mut(direction.offset() + action.start_pos).unwrap() = Some(piece);
            }
            ActionType::Jump(directions) => {
                let mut position = action.start_pos;
                for direction in directions {
                    let middle_pos = direction.offset() + position;
                    let middle_piece = board.piece_mut(middle_pos).unwrap();
                    if middle_piece.unwrap().color() != piece.color() {
                        capture_callback(middle_pos, middle_piece.unwrap());
                        *middle_piece = None;
                    }

                    position = direction.offset() * 2 + position;
                }
                *board.piece_mut(position).unwrap() = Some(piece);
            }
        }

        Ok(board)
    }
    pub fn is_valid_action(&self, action: &Action) -> Result<(), ActionError> {
        let piece = match self.piece(action.start_pos) {
            Ok(piece) => piece,
            Err(error) => {
                return match error {
                    GameBoardError::InvalidPosition => Err(ActionError::InvalidStartPosition),
                };
            }
        };
        if piece.is_none() {
            return Err(ActionError::NoPieceAtStart);
        }
        let piece = piece.unwrap();

        match &action.action_type {
            ActionType::Move(direction) => self.is_valid_move(action.start_pos, *direction)?,
            ActionType::Jump(directions) => {
                self.is_valid_jump(piece, action.start_pos, directions)?
            }
        }

        Ok(())
    }
    pub fn is_valid_move(
        &self,
        start_pos: Coordinate,
        direction: Direction,
    ) -> Result<(), ActionError> {
        let new_pos = direction.offset() + start_pos;
        match self.piece(new_pos) {
            Ok(piece) => {
                if let Some(piece) = piece {
                    Err(PieceOnMove(piece))
                } else {
                    Ok(())
                }
            }
            Err(error) => match error {
                GameBoardError::InvalidPosition => Err(ActionError::MoveOffBoard),
            },
        }
    }
    pub fn is_valid_jump(
        &self,
        piece: Piece,
        start_pos: Coordinate,
        directions: &[Direction],
    ) -> Result<(), ActionError> {
        if directions.is_empty() {
            return Err(ActionError::EmptyJump);
        }
        if piece.size().is_small() && directions.len() > 1 {
            return Err(ActionError::MultipleJumpsForSmall);
        }

        let mut prev_positions = Vec::with_capacity(directions.len());
        prev_positions.push(start_pos);
        for direction in directions {
            let middle_pos = direction.offset() + *prev_positions.last().unwrap();
            let new_pos = direction.offset() + middle_pos;
            if let Some(piece) = match self.piece(new_pos) {
                Ok(piece) => piece,
                Err(error) => {
                    return match error {
                        GameBoardError::InvalidPosition => Err(ActionError::JumpOffBoard),
                    };
                }
            } {
                return Err(ActionError::PieceOnJump(piece));
            }
            if prev_positions.contains(&new_pos) {
                return Err(ActionError::JumpedBackToPrevPosition);
            }
            prev_positions.push(new_pos);

            if self.piece(middle_pos).unwrap().is_none() {
                return Err(ActionError::NoPieceJumped);
            }
        }
        Ok(())
    }
}

pub fn index_to_position<T: Element>(matrix: &Conventional<T>, index: usize) -> impl Position {
    (index % matrix.rows, index / matrix.rows)
}

pub type GameBoardResult<T> = Result<T, GameBoardError>;
#[derive(Copy, Clone, Debug)]
pub enum GameBoardError {
    InvalidPosition,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum BoardSpace {
    Invalid,
    Normal(Option<Piece>),
    Goal {
        goal_for: Color,
        piece: Option<Piece>,
    },
}
impl Element for BoardSpace {
    fn zero() -> Self {
        Self::Normal(None)
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Piece {
    SmallRed,
    LargeRed,
    SmallBlue,
    LargeBlue,
}
impl Piece {
    pub fn color(&self) -> Color {
        match self {
            Piece::SmallRed => Color::Red,
            Piece::LargeRed => Color::Red,
            Piece::SmallBlue => Color::Blue,
            Piece::LargeBlue => Color::Blue,
        }
    }

    pub fn size(&self) -> PieceSize {
        match self {
            Piece::SmallRed => PieceSize::Small,
            Piece::LargeRed => PieceSize::Large,
            Piece::SmallBlue => PieceSize::Small,
            Piece::LargeBlue => PieceSize::Large,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, IntoEnumIterator)]
pub enum Color {
    Red,
    Blue,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum PieceSize {
    Small,
    Large,
}
impl PieceSize {
    pub fn is_small(&self) -> bool {
        matches!(self, PieceSize::Small)
    }

    pub fn is_large(&self) -> bool {
        matches!(self, PieceSize::Large)
    }
}

#[cfg(test)]
mod test {
    use std::ops::Index;

    use matrix::format::Conventional;
    use matrix::matrix;

    use crate::game_board::index_to_position;

    #[test]
    fn index_position_test() {
        let matrix = Conventional::from_vec(
            (4, 4),
            matrix![
                 1,  2,  3,  4;
                 5,  6,  7,  8;
                 9, 10, 11, 12;
                13, 14, 15, 16;
            ],
        );

        for (index, val) in matrix.values.iter().enumerate() {
            assert_eq!(val, matrix.index(index_to_position(&matrix, index)));
        }
    }
}
