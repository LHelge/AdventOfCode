use std::{
    fmt::Display, iter::Flatten, ops::{Add, Mul, Sub}, slice
};
use strum::EnumIter;

#[derive(Debug)]
pub enum Error {
    Empty,
    NotSquare,
    OutOfBounds,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Empty => write!(f, "The Vec2d is empty"),
            Error::NotSquare => write!(f, "The Vec2d is not square"),
            Error::OutOfBounds => write!(f, "The position is out of bounds"),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl Size {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub fn area(&self) -> usize {
        self.width * self.height
    }

    pub fn iter(&self) -> SizeIter {
        SizeIter {
            size: *self,
            current: Position::new(0, 0),
        }
    }
}

pub struct SizeIter {
    size: Size,
    current: Position,
}

impl Iterator for SizeIter {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.y < self.size.height {
            let pos = self.current;
            self.current.x += 1;

            if self.current.x >= self.size.width {
                self.current.x = 0;
                self.current.y += 1;
            }

            Some(pos)
        } else {
            None
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
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

impl Sub<Position> for Position {
    type Output = Distance;

    fn sub(self, other: Position) -> Distance {
        Distance {
            dx: self.x as isize - other.x as isize,
            dy: self.y as isize - other.y as isize,
        }
    }
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn is_inside(&self, size: Size) -> bool {
        self.x < size.width && self.y < size.height
    }
}

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
}

/// A 2D vector that can be indexed with a `Position` struct.
/// (0, 0) is the top-left corner with x increasing to the east and y increasing south.
#[derive(Debug, Default, Clone)]
pub struct Vec2d<T> {
    data: Vec<Vec<T>>,
    size: Size,
}

impl<T> Vec2d<T> {
    pub fn new(data: Vec<Vec<T>>) -> Result<Self, Error> {
        if let Some(row) = data.first() {
            let size = Size {
                width: row.len(),
                height: data.len(),
            };

            if data.iter().all(|row| row.is_empty()) {
                Err(Error::Empty)
            } else if data.iter().any(|row| row.len() != size.width) {
                Err(Error::NotSquare)
            } else {
                Ok(Self { data, size })
            }
        } else {
            Err(Error::Empty)
        }
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn width(&self) -> usize {
        self.size.width
    }

    pub fn height(&self) -> usize {
        self.size.height
    }

    pub fn get(&self, pos: Position) -> Option<&T> {
        self.data.get(pos.y)?.get(pos.x)
    }

    pub fn set(&mut self, pos: Position, item: T) -> Result<(), Error> {
        if !pos.is_inside(self.size) {
            return Err(Error::OutOfBounds);
        }

        self.data[pos.y][pos.x] = item;
        Ok(())
    }

    pub fn flatten(&self) -> Flatten<slice::Iter<'_, Vec<T>>> {
        self.data.iter().flatten()
    }

    pub fn find(&self, item: &T) -> Option<(Position, &T)> where T: Eq{
        for p in self.size().iter() {
            if let Some(i) = self.get(p) {
                if item == i {
                    return Some((p, i));
                }
            }
        }
        None
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        let size = Size::new(3, 4);
        assert_eq!(size.width, 3);
        assert_eq!(size.height, 4);
        assert_eq!(size.area(), 12);

        let mut iter = size.iter();
        assert_eq!(iter.next(), Some(Position::new(0, 0)));
        assert_eq!(iter.next(), Some(Position::new(1, 0)));
        assert_eq!(iter.next(), Some(Position::new(2, 0)));
        assert_eq!(iter.next(), Some(Position::new(0, 1)));
        assert_eq!(iter.nth(8), None);
    }

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

    #[test]
    fn test_vec2d() {
        let data = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
            vec![10, 12, 12],
        ];

        let vec2d = Vec2d::new(data).unwrap();

        assert_eq!(vec2d.width(), 3);
        assert_eq!(vec2d.height(), 4);

        assert_eq!(vec2d.get(Position::new(0, 0)), Some(&1));
        assert_eq!(vec2d.get(Position::new(1, 1)), Some(&5));
        assert_eq!(vec2d.get(Position::new(2, 2)), Some(&9));
        assert_eq!(vec2d.get(Position::new(3, 4)), None);
    }
}
