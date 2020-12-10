/// The alteration for placement
#[derive(Copy, Clone, Debug)]
pub enum AlternationType {
    /// Players alternate placing per_turn_count pieces.
    TurnsCount {
        /// Must be > 0
        per_turn_count: usize
    },
    /// Players alternate placing per_turn_points points.
    /// Requires piece_limits to contain a point limit.
    TurnsPoints {
        per_turn_points: usize,
        hard_limit: bool,
    },
    /// The player with the lowest total points places, first color places on ties.
    Points,
    /// Players place their whole side on their turn.
    WholePlacement,
    /// Players place their pieces hidden to each other.
    Hidden,
}
impl AlternationType {
    pub fn verify(&self, piece_limits: &HashSet<PieceLimits>) -> AlterationTypeResult<()> {
        match self {
            AlternationType::TurnsCount { per_turn_count } => if per_turn_count == 0 {
                return Err(AlterationTypeError::CountIs0);
            },
            AlternationType::TurnsPoints { per_turn_points, hard_limit: _ } => {
                if per_turn_points == 0 {
                    return Err(AlterationTypeError::PointsIs0);
                }
                if piece_limits.contains(PieceLimits::)
            }
            _ => {}
        }
        Ok(())
    }
}
pub type AlterationTypeResult<T> = Result<T, AlterationTypeError>;
pub enum AlterationTypeError {
    CountIs0,
    PointsIs0,
}
