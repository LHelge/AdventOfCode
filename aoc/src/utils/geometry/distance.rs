use super::Direction;
use std::ops::Mul;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Distance {
    pub dx: isize,
    pub dy: isize,
}

impl Mul<isize> for Distance {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self {
        Self {
            dx: self.dx * rhs,
            dy: self.dy * rhs,
        }
    }
}

impl Mul<usize> for Distance {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self {
        self * rhs as isize
    }
}

impl From<Direction> for Distance {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::North => Self { dx: 0, dy: -1 },
            Direction::NorthEast => Self { dx: 1, dy: -1 },
            Direction::East => Self { dx: 1, dy: 0 },
            Direction::SouthEast => Self { dx: 1, dy: 1 },
            Direction::South => Self { dx: 0, dy: 1 },
            Direction::SouthWest => Self { dx: -1, dy: 1 },
            Direction::West => Self { dx: -1, dy: 0 },
            Direction::NorthWest => Self { dx: -1, dy: -1 },
        }
    }
}

impl Distance {
    pub fn new(dx: isize, dy: isize) -> Self {
        Self { dx, dy }
    }

    pub fn manhattan(&self) -> usize {
        (self.dx.abs() + self.dy.abs()) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let distance1 = Distance::new(3, 4);
        assert_eq!(distance1.manhattan(), 7);

        let distance2 = distance1 * 2usize;
        assert_eq!(distance2.dx, 6);
        assert_eq!(distance2.dy, 8);

        let distance3 = distance1 * -1isize;
        assert_eq!(distance3.dx, -3);
        assert_eq!(distance3.dy, -4);
    }
}
