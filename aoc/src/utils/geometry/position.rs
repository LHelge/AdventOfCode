use super::{Direction, Distance, Size};
use crate::AoCError;
use std::{
    fmt::Display,
    ops::{Add, Sub},
    str::FromStr,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl FromStr for Position {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(AoCError::BadInput)?;
        Ok(Position::new(x.parse()?, y.parse()?))
    }
}

impl Add<Distance> for Position {
    type Output = Self;

    fn add(self, distance: Distance) -> Self {
        Self {
            x: self.x.wrapping_add_signed(distance.dx),
            y: self.y.wrapping_add_signed(distance.dy),
        }
    }
}

impl Add<Direction> for Position {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        self + Distance::from(rhs)
    }
}

impl Sub<Distance> for Position {
    type Output = Self;

    fn sub(self, distance: Distance) -> Self {
        Self {
            x: self.x.wrapping_add_signed(-distance.dx),
            y: self.y.wrapping_add_signed(-distance.dy),
        }
    }
}

impl Sub<Position> for Position {
    type Output = Distance;

    fn sub(self, other: Position) -> Distance {
        Distance {
            dx: self.x as isize - other.x as isize,
            dy: self.y as isize - other.y as isize,
        }
    }
}

impl Sub<Direction> for Position {
    type Output = Position;

    fn sub(self, rhs: Direction) -> Self::Output {
        self - Distance::from(rhs)
    }
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn is_inside(&self, size: &Size) -> bool {
        self.x < size.width && self.y < size.height
    }

    pub fn invert(self) -> Self {
        Self {
            x: self.y,
            y: self.x,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position() {
        let pos1 = Position::new(2, 3);
        let pos2 = Position::new(4, 7);

        let distance = pos2 - pos1;

        assert_eq!(distance.dx, 2);
        assert_eq!(distance.dy, 4);

        let pos3 = pos1 + distance;

        assert_eq!(pos3, pos2);
    }
}
