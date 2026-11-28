use aoc::{problem::*, utils::*, *};

#[derive(Debug)]
enum Line {
    Horizontal {
        x1: usize,
        x2: usize,
        y: usize,
    },
    Vertical {
        x: usize,
        y1: usize,
        y2: usize,
    },
    #[allow(unused)]
    Other {
        topleft: Position,
        bottomright: Position,
    },
}

impl Line {
    fn new(p1: Position, p2: Position) -> Self {
        if p1.x == p2.x {
            let x = p1.x;
            let y1 = p1.y.min(p2.y);
            let y2 = p1.y.max(p2.y);
            Line::Vertical { x, y1, y2 }
        } else if p1.y == p2.y {
            let x1 = p1.x.min(p2.x);
            let x2 = p1.x.max(p2.x);
            let y = p1.y;
            Line::Horizontal { x1, x2, y }
        } else {
            let topleft = Position::new(p1.x.min(p2.x), p1.y.min(p2.y));
            let bottomright = Position::new(p1.x.max(p2.x), p1.y.max(p2.y));
            Line::Other {
                topleft,
                bottomright,
            }
        }
    }
}

#[derive(Debug)]
struct Rectangle(Position, Position);

impl Rectangle {
    fn new(p1: Position, p2: Position) -> Self {
        let topleft = Position::new(p1.x.min(p2.x), p1.y.min(p2.y));
        let bottomright = Position::new(p1.x.max(p2.x), p1.y.max(p2.y));
        Self(topleft, bottomright)
    }

    fn size(&self) -> usize {
        (self.0.x.abs_diff(self.1.x) + 1) * (self.0.y.abs_diff(self.1.y) + 1)
    }

    fn intersects(&self, line: &Line) -> bool {
        match line {
            Line::Horizontal { x1, x2, y } => {
                *y > self.0.y && *y < self.1.y && *x1 < self.1.x && *x2 > self.0.x
            }
            Line::Vertical { x, y1, y2 } => {
                *x > self.0.x && *x < self.1.x && *y1 < self.1.y && *y2 > self.0.y
            }
            Line::Other { .. } => unimplemented!("Not required here!"),
        }
    }
}

#[derive(Debug, Default)]
struct Polygon(Vec<Line>);

impl Polygon {
    fn contains(&self, rect: &Rectangle) -> bool {
        !self.0.iter().any(|line| rect.intersects(line))
    }
}

struct Problem {
    rectangles: Vec<Rectangle>,
    polygon: Polygon,
}

impl AoCProblem for Problem {
    type Solution1 = usize;
    type Solution2 = usize;

    const YEAR: u16 = 2025;
    const DAY: u8 = 9;

    fn parse(input: &str) -> Result<Self> {
        let tiles = input.parse_lines()?;

        let mut rectangles: Vec<Rectangle> = tiles
            .pairs()
            .map(|(p1, p2)| Rectangle::new(*p1, *p2))
            .collect();
        rectangles.sort_by_key(|r| r.size());
        rectangles.reverse();

        let mut lines: Vec<Line> = tiles
            .as_slice()
            .windows(2)
            .map(|w| Line::new(w[0], w[1]))
            .collect();

        // Close the polygon
        let start = tiles.first().ok_or(AoCError::BadInput)?;
        let end = tiles.last().ok_or(AoCError::BadInput)?;
        lines.push(Line::new(*start, *end));

        Ok(Self {
            rectangles,
            polygon: Polygon(lines),
        })
    }

    fn part1(&self) -> Result<usize> {
        Ok(self.rectangles.first().ok_or(AoCError::BadInput)?.size())
    }

    fn part2(&self) -> Result<usize> {
        Ok(self
            .rectangles
            .iter()
            .find(|rect| self.polygon.contains(rect))
            .ok_or(AoCError::BadInput)?
            .size())
    }
}

fn main() -> Result<()> {
    Problem::run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() -> Result<()> {
        let input = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"#;

        let problem = Problem::parse(input)?;
        assert_eq!(problem.part1()?, 50);
        assert_eq!(problem.part2()?, 24);

        Ok(())
    }
}
