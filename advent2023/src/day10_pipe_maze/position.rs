use std::fmt::Display;
use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Position(pub isize, pub isize);

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Add<Displacement> for Position {
    type Output = Position;

    fn add(self, rhs: Displacement) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Position {
    type Output = Displacement;

    fn sub(self, rhs: Self) -> Self::Output {
        Displacement(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Displacement(pub isize, pub isize);

impl Add<Position> for Displacement {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_positions() {
        let position1 = Position(3, 10);
        let position2 = Position(5, 5);
        assert_eq!(position1 + position2, Position(8, 15));
    }

    #[test]
    fn test_add_positions_negative_result() {
        let position1 = Position(3, 10);
        let position2 = Position(-12, -15);
        assert_eq!(position1 + position2, Position(-9, -5));
    }

    #[test]
    fn test_add_position_with_negative_displacement() {
        let position = Position(3, 10);
        let position_delta = Displacement(-6, -10);
        assert_eq!(position + position_delta, Position(-3, 0));
    }

    #[test]
    fn test_add_position_with_displacement() {
        let position = Position(3, 10);
        let position_delta = Displacement(6, 10);
        assert_eq!(position + position_delta, Position(9, 20));
    }

    #[test]
    fn test_add_displacement_with_position() {
        let position = Position(3, 10);
        let position_delta = Displacement(6, 10);
        assert_eq!(position_delta + position, Position(9, 20));
    }

    #[test]
    fn test_sub_positions() {
        let position1 = Position(3, 10);
        let position2 = Position(6, 10);
        assert_eq!(position1 - position2, Displacement(-3, 0));
    }
}
