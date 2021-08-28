#![warn(missing_debug_implementations)]

pub mod action;
pub mod coordinate;
pub mod direction;
pub mod game_board;
pub mod ruleset;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
