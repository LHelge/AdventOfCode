use super::Position;

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
}
