use super::position::Displacement;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum PipeType {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    StartingPosition,
}

// TODO should rename that error, this is about pipe type
#[derive(Debug, Eq, PartialEq)]
pub struct InvalidPositionError(pub char);

impl Display for InvalidPositionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid pipe type during conversion. Symbol found: {}",
            self.0
        )
    }
}

impl Error for InvalidPositionError {}

impl TryFrom<char> for PipeType {
    type Error = InvalidPositionError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(PipeType::NorthSouth),
            '-' => Ok(PipeType::EastWest),
            'L' => Ok(PipeType::NorthEast),
            'J' => Ok(PipeType::NorthWest),
            '7' => Ok(PipeType::SouthWest),
            'F' => Ok(PipeType::SouthEast),
            '.' => Ok(PipeType::Ground),
            'S' => Ok(PipeType::StartingPosition),
            invalid => Err(InvalidPositionError(invalid)),
        }
    }
}

impl PipeType {
    pub fn get_valid_connections(&self) -> Vec<Displacement> {
        match self {
            PipeType::NorthSouth => vec![Displacement(0, -1), Displacement(0, 1)],
            PipeType::EastWest => vec![Displacement(-1, 0), Displacement(1, 0)],
            PipeType::NorthEast => vec![Displacement(0, 1), Displacement(1, 0)],
            PipeType::NorthWest => vec![Displacement(0, 1), Displacement(-1, 0)],
            PipeType::SouthWest => vec![Displacement(0, -1), Displacement(-1, 0)],
            PipeType::SouthEast => vec![Displacement(0, -1), Displacement(1, 0)],
            PipeType::Ground => vec![],
            PipeType::StartingPosition => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_from() {
        let pos = PipeType::try_from('F');
        assert_eq!(pos, Ok(PipeType::SouthEast));
    }

    #[test]
    fn position_from_invalid() {
        let pos = PipeType::try_from('X');
        assert_eq!(pos, Err(InvalidPositionError('X')));
    }
}
