use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::fmt::Display;
use std::hash::{Hash, Hasher};

use crate::direction::Directions;

/// Defines a piece
///
/// Hash, Eq, and PartialEq are only defined for `name`
#[derive(Clone, Debug)]
pub struct PieceDefinition {
    /// The name of the piece type
    pub name: String,
    /// How this piece can capture and who that captures
    pub capture_rules: HashMap<CaptureRule, CaptureTarget>,
    /// The rule for how jumps can happen for this piece
    pub jump_rule: JumpRule,
    /// The rule for when pieces are captured by this piece
    pub capture_timing_rule: CaptureTimingRule,
    /// The rule for whether this piece is forced to capture if possible
    pub capture_requirement: CaptureRequirement,
    /// The rule for how many jumps this piece can make
    pub jump_limit: JumpLimit,
    /// The rule for how this piece can move
    pub move_rule: MoveRule,
    /// The rule for how this piece moves within a goal
    pub goal_move_rule: GoalMovementRule,
}
impl PieceDefinition {
    pub fn verify(&self) -> PieceDefinitionResult<()> {
        if self.name.is_empty() {
            return Err(PieceDefinitionError::NameInvalid(self.name.clone()));
        }
        self.jump_limit.verify()?;
        self.move_rule.verify()?;
        Ok(())
    }
}
impl Hash for PieceDefinition {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
impl PartialEq for PieceDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}
impl Eq for PieceDefinition {}

pub type PieceDefinitionResult<T> = Result<T, PieceDefinitionError>;
#[derive(Clone, Debug)]
pub enum PieceDefinitionError {
    NameInvalid(String),
    JumpLimitError(JumpLimitError),
    MoveRuleError(MoveRuleError),
}
impl Display for PieceDefinitionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}
impl Error for PieceDefinitionError {
    fn cause(&self) -> Option<&dyn Error> {
        match self {
            PieceDefinitionError::NameInvalid(_) => None,
            PieceDefinitionError::JumpLimitError(error) => Some(error),
            PieceDefinitionError::MoveRuleError(error) => Some(error),
        }
    }
}
impl From<JumpLimitError> for PieceDefinitionError {
    fn from(from: JumpLimitError) -> Self {
        Self::JumpLimitError(from)
    }
}
impl From<MoveRuleError> for PieceDefinitionError {
    fn from(from: MoveRuleError) -> Self {
        Self::MoveRuleError(from)
    }
}

/// The rule for how jumps can happen
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum JumpRule {
    /// A piece may not go to any previous space within the same jump
    NoBacktracking,
    /// A piece cannot end up in the same starting location
    NoSameStart,
    /// All non-repetitive jumps are legal
    Open,
}
/// The rule for how captures can happen
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum CaptureRule {
    /// Can capture by jumping over
    JumpOver,
    /// Can capture by jumping on
    JumpOn,
    /// Can capture by moving onto
    Move,
}
/// The rule for when captures happen during
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum CaptureTimingRule {
    /// Pieces are removed after they are jumped over, an enemy piece cannot be jumped twice
    AfterJump,
    /// Pieces are removed at the end of the turn
    AfterTurn,
}
/// The rule for what can get captured
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum CaptureTarget {
    /// Captures only enemy pieces
    EnemyOnly,
    /// Captures only own pieces
    OwnOnly,
    /// Captures all color pieces
    All,
}
/// The rule for when this piece is forced to capture
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum CaptureRequirement {
    /// Must capture if possible, higher values mean this piece is forced before others
    Forced(isize),
    /// No forced capture
    None,
}
/// The rule for how a piece jumps
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum JumpLimit {
    /// Piece can jump an unlimited number of times
    Unlimited {
        directions: Directions,
    },
    /// Piece can jump a limited number of times
    Limited {
        limit: usize,
        directions: Directions,
    },
    /// Piece cannot jump
    Cannot,
}
impl JumpLimit {
    pub fn verify(&self) -> JumpLimitResult<()> {
        let (&directions, limit) = match self {
            Self::Unlimited { directions } => (directions, None),
            Self::Limited { limit, directions } => (directions, Some(limit)),
            Self::Cannot => return Ok(()),
        };
        if directions == Directions::NONE {
            return Err(JumpLimitError::NoDirectionsSet);
        } else if let Some(&limit) = limit {
            if limit == 0 {
                return Err(JumpLimitError::LimitedTo0);
            }
        }
        Ok(())
    }
}
pub type JumpLimitResult<T> = Result<T, JumpLimitError>;
#[derive(Copy, Clone, Debug)]
pub enum JumpLimitError {
    NoDirectionsSet,
    LimitedTo0,
}
impl Display for JumpLimitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}
impl Error for JumpLimitError {}

/// The rule for how this piece moves
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MoveRule {
    /// Piece can move in any one direction from the vec up to the limit amount
    SameDirection {
        limit: usize,
        directions: Directions,
    },
    /// Piece can move in any direction from the vec up to the limit amount
    AnyDirection {
        limit: usize,
        directions: Directions,
    },
    /// Pieces cannot move without jumping
    None,
}
impl MoveRule {
    pub fn verify(&self) -> MoveRuleResult<()> {
        let (&directions, &limit) = match self {
            Self::SameDirection { limit, directions } => (directions, limit),
            Self::AnyDirection { limit, directions } => (directions, limit),
            Self::None => return Ok(()),
        };
        if directions == Directions::NONE {
            return Err(MoveRuleError::NoDirectionsSet);
        }
        if limit == 0 {
            return Err(MoveRuleError::LimitedTo0);
        }
        Ok(())
    }
}
pub type MoveRuleResult<T> = Result<T, MoveRuleError>;
#[derive(Copy, Clone, Debug)]
pub enum MoveRuleError {
    NoDirectionsSet,
    LimitedTo0,
}
impl Display for MoveRuleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}
impl Error for MoveRuleError {}

/// The rule for what movement is allowed while in a goal
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum GoalMovementRule {
    /// Piece is locked in place once it gets to the goal
    Locked,
    /// Piece can only move to other goals
    OnlyToGoal,
    /// Piece is free to move from the goal
    Free,
}
