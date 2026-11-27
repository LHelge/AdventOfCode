use crate::AoCError;
use strum::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Hash)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub fn turn_cv(&self) -> Direction {
        match self {
            Self::North => Direction::NorthEast,
            Self::NorthEast => Direction::East,
            Self::East => Direction::SouthEast,
            Self::SouthEast => Direction::South,
            Self::South => Direction::SouthWest,
            Self::SouthWest => Direction::West,
            Self::West => Direction::NorthWest,
            Self::NorthWest => Direction::North,
        }
    }

    pub fn turn_ccv(&self) -> Direction {
        match self {
            Self::North => Direction::NorthWest,
            Self::NorthEast => Direction::North,
            Self::East => Direction::NorthEast,
            Self::SouthEast => Direction::East,
            Self::South => Direction::SouthEast,
            Self::SouthWest => Direction::South,
            Self::West => Direction::SouthWest,
            Self::NorthWest => Direction::West,
        }
    }

    pub fn is_east_west(&self) -> bool {
        matches!(self, Self::East | Self::West)
    }

    pub fn is_north_south(&self) -> bool {
        matches!(self, Self::North | Self::South)
    }
}

impl TryFrom<char> for Direction {
    type Error = AoCError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Self::North),
            '>' => Ok(Self::East),
            'v' => Ok(Self::South),
            '<' => Ok(Self::West),
            c => Err(AoCError::UnexpectedCharacter(c)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::Distance;

    #[test]
    fn test_direction() {
        let distance = Distance::from(Direction::North);
        assert_eq!(distance.dx, 0);
        assert_eq!(distance.dy, -1);

        let distance = Distance::from(Direction::NorthEast);
        assert_eq!(distance.dx, 1);
        assert_eq!(distance.dy, -1);

        let distance = Distance::from(Direction::East);
        assert_eq!(distance.dx, 1);
        assert_eq!(distance.dy, 0);

        let distance = Distance::from(Direction::SouthEast);
        assert_eq!(distance.dx, 1);
        assert_eq!(distance.dy, 1);

        let distance = Distance::from(Direction::South);
        assert_eq!(distance.dx, 0);
        assert_eq!(distance.dy, 1);

        let distance = Distance::from(Direction::SouthWest);
        assert_eq!(distance.dx, -1);
        assert_eq!(distance.dy, 1);

        let distance = Distance::from(Direction::West);
        assert_eq!(distance.dx, -1);
        assert_eq!(distance.dy, 0);

        let distance = Distance::from(Direction::NorthWest);
        assert_eq!(distance.dx, -1);
        assert_eq!(distance.dy, -1);
    }
}
