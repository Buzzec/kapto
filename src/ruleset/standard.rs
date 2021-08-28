use std::collections::HashMap;

use crate::direction::Directions;
use crate::ruleset::piece_definition::{
    CaptureRequirement, CaptureRule, CaptureTarget, CaptureTimingRule, GoalMovementRule, JumpLimit,
    JumpRule, MoveRule, PieceDefinition,
};
use crate::ruleset::starting_positions::StartingPositions;
use crate::ruleset::{BoardType, Ruleset, RulesetResult};

pub fn standard_rules() -> RulesetResult<Ruleset> {
    let out = Ruleset {
        pieces: get_pieces(),
        board_type: get_board(),
        starting_positions: get_starting_positions(),
        victory_conditions: Default::default(),
    };
    out.verify()?;
    Ok(out)
}

fn get_pieces() -> Vec<PieceDefinition> {
    let capture_rules: HashMap<_, _> = vec![(CaptureRule::JumpOver, CaptureTarget::EnemyOnly)]
        .into_iter()
        .collect();
    let big = PieceDefinition {
        name: "Big".to_string(),
        capture_rules: capture_rules.clone(),
        jump_rule: JumpRule::NoSameStart,
        capture_timing_rule: CaptureTimingRule::AfterTurn,
        capture_requirement: CaptureRequirement::Forced(10),
        jump_limit: JumpLimit::Unlimited {
            directions: Directions::ALL,
        },
        move_rule: MoveRule::AnyDirection {
            limit: 1,
            directions: Directions::ALL,
        },
        goal_move_rule: GoalMovementRule::Free,
    };

    let small = PieceDefinition {
        name: "Little".to_string(),
        capture_rules,
        jump_rule: JumpRule::NoSameStart,
        capture_timing_rule: CaptureTimingRule::AfterTurn,
        capture_requirement: CaptureRequirement::Forced(10),
        jump_limit: JumpLimit::Limited {
            limit: 1,
            directions: Directions::ALL,
        },
        move_rule: MoveRule::AnyDirection {
            limit: 1,
            directions: Directions::ALL,
        },
        goal_move_rule: GoalMovementRule::Free,
    };

    vec![big, small]
}
fn get_board() -> BoardType {
    BoardType::Rectangular {
        rows: 10,
        columns: 10,
        goal_locations: [4, 5].iter().cloned().collect(),
    }
}
fn get_starting_positions() -> StartingPositions {
    // StartingPositions::MirroredFlipped()
    unimplemented!()
}
