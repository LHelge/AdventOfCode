use super::{Position, Size};
use crate::AoCError;
use std::{fmt::Display, iter::Flatten, slice, str::FromStr};
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
            for r in self.data.iter() {
                row.push(r[x].clone());
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
