use crate::AoCError;
use std::{
    fmt::Display,
    iter::Flatten,
    ops::{Add, Mul, Sub},
    slice,
    str::FromStr,
};
use strum::EnumIter;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Vec2dError {
    #[error("The Vec2d is empty")]
    Empty,

    #[error("The Vec2d is not square")]
    NotSquare,

    #[error("The position is out of bounds")]
    OutOfBounds,
}

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

    pub fn contains(&self, pos: Position) -> bool {
        pos.is_inside(self)
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

/// A 2D vector that can be indexed with a `Position` struct.
/// (0, 0) is the top-left corner with x increasing to the east and y increasing south.
#[derive(Debug, Default, Clone)]
pub struct Vec2d<T> {
    data: Vec<Vec<T>>,
    size: Size,
}

impl<T> Vec2d<T> {
    pub fn new(data: Vec<Vec<T>>) -> Result<Self, Vec2dError> {
        if let Some(row) = data.first() {
            let size = Size {
                width: row.len(),
                height: data.len(),
            };

            if data.iter().all(|row| row.is_empty()) {
                Err(Vec2dError::Empty)
            } else if data.iter().any(|row| row.len() != size.width) {
                Err(Vec2dError::NotSquare)
            } else {
                Ok(Self { data, size })
            }
        } else {
            Err(Vec2dError::Empty)
        }
    }

    pub fn size(&self) -> &Size {
        &self.size
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

    pub fn set(&mut self, pos: Position, item: T) -> Result<(), Vec2dError> {
        if !self.size.contains(pos) {
            return Err(Vec2dError::OutOfBounds);
        }

        self.data[pos.y][pos.x] = item;
        Ok(())
    }

    pub fn flatten(&self) -> Flatten<slice::Iter<'_, Vec<T>>> {
        self.data.iter().flatten()
    }

    pub fn find(&self, item: &T) -> Option<(Position, &T)>
    where
        T: Eq,
    {
        for p in self.size().iter() {
            if let Some(i) = self.get(p)
                && i == item
            {
                return Some((p, i));
            }
        }
        None
    }

    pub fn iter(&self) -> Vec2dIter<'_, T> {
        Vec2dIter {
            vec2d: self,
            current: Position::new(0, 0),
        }
    }

    pub fn iter_row(&self, row: usize) -> Vec2dRowIter<'_, T> {
        Vec2dRowIter {
            vec2d: self,
            current: Position::new(0, row),
        }
    }

    pub fn iter_row_mut(&mut self, row: usize) -> Vec2dRowIterMut<'_, T> {
        Vec2dRowIterMut {
            iter: self.data[row].iter_mut(),
            row,
            col: 0,
        }
    }

    pub fn iter_col(&self, col: usize) -> Vec2dColIter<'_, T> {
        Vec2dColIter {
            vec2d: self,
            current: Position::new(col, 0),
        }
    }

    pub fn data(&self) -> &Vec<Vec<T>> {
        &self.data
    }

    pub fn transposed(&self) -> Self
    where
        T: Clone,
    {
        let mut transposed = Vec::with_capacity(self.width());
        for x in 0..self.width() {
            let mut row = Vec::with_capacity(self.height());
            for y in 0..self.height() {
                row.push(self.data[y][x].clone());
            }
            transposed.push(row);
        }

        // Should never fail since the original Vec2d is square
        Self::new(transposed).unwrap()
    }

    pub fn modify(&mut self, pos: Position, f: fn(&mut T)) -> Result<(), Vec2dError> {
        if !self.size.contains(pos) {
            return Err(Vec2dError::OutOfBounds);
        }

        f(&mut self.data[pos.y][pos.x]);
        Ok(())
    }

    pub fn swap(&mut self, src: Position, dst: Position) -> Result<(), Vec2dError>
    where
        T: Copy,
    {
        if !self.size.contains(src) || !self.size.contains(dst) {
            return Err(Vec2dError::OutOfBounds);
        }

        let tmp = self.data[src.y][src.x];
        self.data[src.y][src.x] = self.data[dst.y][dst.x];
        self.data[dst.y][dst.x] = tmp;
        Ok(())
    }
}

impl<T> FromStr for Vec2d<T>
where
    T: TryFrom<char, Error = AoCError>,
{
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .lines()
            .map(|line| line.chars().map(T::try_from).collect())
            .collect::<Result<Vec<Vec<T>>, Self::Err>>()?;

        Ok(Self::new(data)?)
    }
}

pub struct Vec2dIter<'a, T> {
    vec2d: &'a Vec2d<T>,
    current: Position,
}

impl<'a, T> Iterator for Vec2dIter<'a, T> {
    type Item = (Position, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.y < self.vec2d.height() {
            let pos = self.current;
            self.current.x += 1;

            if self.current.x >= self.vec2d.width() {
                self.current.x = 0;
                self.current.y += 1;
            }

            Some((pos, self.vec2d.get(pos).unwrap()))
        } else {
            None
        }
    }
}

pub struct Vec2dRowIter<'a, T> {
    vec2d: &'a Vec2d<T>,
    current: Position,
}

impl<'a, T> Iterator for Vec2dRowIter<'a, T> {
    type Item = (Position, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.x < self.vec2d.width() {
            let pos = self.current;
            self.current.x += 1;

            Some((pos, self.vec2d.get(pos)?))
        } else {
            None
        }
    }
}

pub struct Vec2dRowIterMut<'a, T> {
    iter: slice::IterMut<'a, T>,
    row: usize,
    col: usize,
}

impl<'a, T> Iterator for Vec2dRowIterMut<'a, T> {
    type Item = (Position, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iter.next()?;
        let pos = Position::new(self.col, self.row);
        self.col += 1;
        Some((pos, item))
    }
}

pub struct Vec2dColIter<'a, T> {
    vec2d: &'a Vec2d<T>,
    current: Position,
}

impl<'a, T> Iterator for Vec2dColIter<'a, T> {
    type Item = (Position, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.y < self.vec2d.height() {
            let pos = self.current;
            self.current.y += 1;

            Some((pos, self.vec2d.get(pos)?))
        } else {
            None
        }
    }
}

impl<T> Display for Vec2d<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in &self.data {
            for item in row {
                write!(f, "{}", item)?;
            }
            writeln!(f)?;
        }
        Ok(())
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

    #[test]
    fn test_vec2d_iter_row() {
        let data = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
            vec![10, 12, 12],
        ];

        let vec2d = Vec2d::new(data).unwrap();
        let mut iter = vec2d.iter_row(1);

        assert_eq!(iter.next(), Some((Position::new(0, 1), &4)));
        assert_eq!(iter.next(), Some((Position::new(1, 1), &5)));
        assert_eq!(iter.next(), Some((Position::new(2, 1), &6)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_vec2d_iter_row_mut() {
        let data = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
            vec![10, 11, 12],
        ];

        let mut vec2d = Vec2d::new(data).unwrap();

        for (_, item) in vec2d.iter_row_mut(1) {
            *item *= 2;
        }

        assert_eq!(vec2d.get(Position::new(0, 0)), Some(&1));
        assert_eq!(vec2d.get(Position::new(1, 0)), Some(&2));
        assert_eq!(vec2d.get(Position::new(2, 0)), Some(&3));
        assert_eq!(vec2d.get(Position::new(0, 1)), Some(&8));
        assert_eq!(vec2d.get(Position::new(1, 1)), Some(&10));
        assert_eq!(vec2d.get(Position::new(2, 1)), Some(&12));
        assert_eq!(vec2d.get(Position::new(0, 2)), Some(&7));
        assert_eq!(vec2d.get(Position::new(1, 2)), Some(&8));
        assert_eq!(vec2d.get(Position::new(2, 2)), Some(&9));
        assert_eq!(vec2d.get(Position::new(0, 3)), Some(&10));
        assert_eq!(vec2d.get(Position::new(1, 3)), Some(&11));
        assert_eq!(vec2d.get(Position::new(2, 3)), Some(&12));
    }

    #[test]
    fn test_vec2d_iter_col() {
        let data = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
            vec![10, 12, 12],
        ];

        let vec2d = Vec2d::new(data).unwrap();
        let mut iter = vec2d.iter_col(1);

        assert_eq!(iter.next(), Some((Position::new(1, 0), &2)));
        assert_eq!(iter.next(), Some((Position::new(1, 1), &5)));
        assert_eq!(iter.next(), Some((Position::new(1, 2), &8)));
        assert_eq!(iter.next(), Some((Position::new(1, 3), &12)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_transposed() {
        let data = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
            vec![10, 12, 12],
        ];

        let vec2d = Vec2d::new(data).unwrap().transposed();

        assert_eq!(vec2d.width(), 4);
        assert_eq!(vec2d.height(), 3);

        assert_eq!(vec2d.get(Position::new(0, 0)), Some(&1));
        assert_eq!(vec2d.get(Position::new(1, 0)), Some(&4));
        assert_eq!(vec2d.get(Position::new(2, 0)), Some(&7));
        assert_eq!(vec2d.get(Position::new(3, 0)), Some(&10));
        assert_eq!(vec2d.get(Position::new(0, 1)), Some(&2));
        assert_eq!(vec2d.get(Position::new(1, 1)), Some(&5));
        assert_eq!(vec2d.get(Position::new(2, 1)), Some(&8));
        assert_eq!(vec2d.get(Position::new(3, 1)), Some(&12));
        assert_eq!(vec2d.get(Position::new(0, 2)), Some(&3));
        assert_eq!(vec2d.get(Position::new(1, 2)), Some(&6));
        assert_eq!(vec2d.get(Position::new(2, 2)), Some(&9));
        assert_eq!(vec2d.get(Position::new(3, 2)), Some(&12));
    }

    #[test]
    fn test_modify() {
        let data = vec![vec![1, 2], vec![3, 4]];

        let mut vec2d = Vec2d::new(data).unwrap();

        vec2d.modify(Position::new(0, 0), |x| *x = 5).unwrap();

        assert_eq!(vec2d.get(Position::new(0, 0)), Some(&5));
        assert_eq!(vec2d.get(Position::new(0, 1)), Some(&3));
        assert_eq!(vec2d.get(Position::new(1, 0)), Some(&2));
        assert_eq!(vec2d.get(Position::new(1, 1)), Some(&4));
    }

    #[test]
    fn test_swap() {
        let data = vec![vec![1, 2], vec![3, 4]];

        let mut vec2d = Vec2d::new(data).unwrap();

        vec2d
            .swap(Position::new(0, 0), Position::new(1, 1))
            .unwrap();

        assert_eq!(vec2d.get(Position::new(0, 0)), Some(&4));
        assert_eq!(vec2d.get(Position::new(0, 1)), Some(&3));
        assert_eq!(vec2d.get(Position::new(1, 0)), Some(&2));
        assert_eq!(vec2d.get(Position::new(1, 1)), Some(&1));
    }
}
