use crate::coordinate::Coordinate;
use crate::direction::Direction;
use crate::game_board::Piece;

#[derive(Debug)]
pub struct Action {
    pub start_pos: Coordinate,
    pub action_type: ActionType,
}

#[derive(Debug)]
pub enum ActionType {
    Move(Direction),
    Jump(Vec<Direction>),
}

#[derive(Copy, Clone, Debug)]
pub enum ActionError {
    InvalidStartPosition,
    NoPieceAtStart,
    PieceOnMove(Piece),
    MoveOffBoard,
    EmptyJump,
    PieceOnJump(Piece),
    NoPieceJumped,
    JumpOffBoard,
    JumpedBackToPrevPosition,
    MultipleJumpsForSmall,
}
