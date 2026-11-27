use super::{Position, Size};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Rectangle {
    top_left: Position,
    size: Size,
}

impl Rectangle {
    pub fn new(top_left: Position, size: Size) -> Self {
        Self { top_left, size }
    }

    pub fn left(&self) -> usize {
        self.top_left.x
    }

    pub fn top(&self) -> usize {
        self.top_left.y
    }

    pub fn bottom(&self) -> usize {
        self.top_left.y + self.size.height
    }

    pub fn right(&self) -> usize {
        self.top_left.x + self.size.width
    }

    pub fn contains(&self, pos: Position) -> bool {
        pos.x >= self.left() && pos.x < self.right() && pos.y >= self.top() && pos.y < self.bottom()
    }

    pub fn intersects(&self, other: &Rectangle) -> bool {
        !(self.left() >= other.right()
            || self.right() <= other.left()
            || self.top() >= other.bottom()
            || self.bottom() <= other.top())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rect(x: usize, y: usize, width: usize, height: usize) -> Rectangle {
        Rectangle::new(Position { x, y }, Size { width, height })
    }

    #[test]
    fn edges() {
        let r = rect(3, 2, 5, 4);
        assert_eq!(r.left(), 3);
        assert_eq!(r.top(), 2);
        assert_eq!(r.right(), 8);
        assert_eq!(r.bottom(), 6);
    }

    #[test]
    fn overlapping_rectangles_intersect() {
        assert!(rect(1, 3, 4, 4).intersects(&rect(3, 1, 4, 4)));
    }

    #[test]
    fn identical_and_nested_rectangles_intersect() {
        let r = rect(2, 2, 4, 4);
        assert!(r.intersects(&r));
        assert!(r.intersects(&rect(3, 3, 1, 1)));
        assert!(rect(3, 3, 1, 1).intersects(&r));
    }

    #[test]
    fn cross_shape_intersects() {
        // Neither rectangle has a corner inside the other
        assert!(rect(0, 2, 6, 2).intersects(&rect(2, 0, 2, 6)));
    }

    #[test]
    fn separated_rectangles_do_not_intersect() {
        let r = rect(5, 5, 2, 2);
        assert!(!r.intersects(&rect(0, 5, 2, 2))); // left
        assert!(!r.intersects(&rect(10, 5, 2, 2))); // right
        assert!(!r.intersects(&rect(5, 0, 2, 2))); // above
        assert!(!r.intersects(&rect(5, 10, 2, 2))); // below
    }
}
