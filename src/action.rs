use crate::direction::Direction;
use crate::game_board::Piece;

pub struct Action {
    pub start_pos: (usize, usize),
    pub action_type: ActionType,
}

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
