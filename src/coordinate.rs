use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};
use std::ops::MulAssign;

use matrix::Position;

use crate::ruleset::board_type::BoardType;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Coordinate {
    pub row: i16,
    pub column: i16,
}
impl Coordinate {
    pub fn new(row: i16, column: i16) -> Self {
        Self { row, column }
    }
}
impl Position for Coordinate {
    fn row(&self) -> usize {
        self.row as usize
    }

    fn column(&self) -> usize {
        self.column as usize
    }

    fn coordinates(&self) -> (usize, usize) {
        (self.row as usize, self.column as usize)
    }
}
impl Add for Coordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.row + rhs.row, self.column)
    }
}
impl AddAssign for Coordinate {
    fn add_assign(&mut self, rhs: Self) {
        self.row += rhs.row;
        self.column += rhs.column;
    }
}
impl Sub for Coordinate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.row - rhs.row, self.column - rhs.column)
    }
}
impl SubAssign for Coordinate {
    fn sub_assign(&mut self, rhs: Self) {
        self.row -= rhs.row;
        self.column -= rhs.column;
    }
}
impl Mul<i16> for Coordinate {
    type Output = Self;

    fn mul(self, rhs: i16) -> Self::Output {
        Self::new(self.row * rhs, self.column * rhs)
    }
}
impl MulAssign<i16> for Coordinate {
    fn mul_assign(&mut self, rhs: i16) {
        self.row *= rhs;
        self.column *= rhs;
    }
}

pub fn flip_coordinate(board: &BoardType, coordinate: Coordinate) -> Coordinate {
    Coordinate::new(board.rows() as i16 - coordinate.row - 1, coordinate.column)
}
pub fn rotate_coordinate(board: &BoardType, coordinate: Coordinate) -> Coordinate {
    Coordinate::new(board.rows() as i16 - coordinate.row - 1, board.columns() as i16 - coordinate.column - 1)
}
