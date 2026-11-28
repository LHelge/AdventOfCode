use aoc::{problem::*, utils::*, *};
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{line_ending, u64, usize},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};

#[derive(Debug)]
struct Claim {
    id: u64,
    rect: Rectangle,
}

impl PartialEq for Claim {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

fn position(input: &str) -> IResult<&str, Position> {
    let (input, (x, y)) = separated_pair(usize, tag(","), usize).parse(input)?;
    Ok((input, Position::new(x, y)))
}

fn size(input: &str) -> IResult<&str, Size> {
    let (input, (width, height)) = separated_pair(usize, tag("x"), usize).parse(input)?;
    Ok((input, Size::new(width, height)))
}

fn rect(input: &str) -> IResult<&str, Rectangle> {
    let (input, (top_left, size)) = separated_pair(position, tag(": "), size).parse(input)?;

    Ok((input, Rectangle::new(top_left, size)))
}

fn id(input: &str) -> IResult<&str, u64> {
    let (input, id) = preceded(tag("#"), u64).parse(input)?;

    Ok((input, id))
}

fn claim(input: &str) -> IResult<&str, Claim> {
    let (input, (id, rect)) = separated_pair(id, tag(" @ "), rect).parse(input)?;

    Ok((input, Claim { id, rect }))
}

struct Problem {
    claims: Vec<Claim>,
}

impl Problem {
    fn has_overlap(&self, pos: Position) -> bool {
        self.claims
            .iter()
            .filter(|claim| claim.rect.contains(pos))
            .count()
            > 1
    }
}

impl AoCProblem for Problem {
    type Solution1 = u64;
    type Solution2 = u64;

    const YEAR: u16 = 2018;
    const DAY: u8 = 3;

    fn parse(input: &str) -> Result<Self> {
        let (_, claims) = separated_list1(line_ending, claim)
            .parse(input)
            .map_err(|_| AoCError::BadInput)?;
        Ok(Self { claims })
    }

    fn part1(&self) -> Result<u64> {
        let mut count = 0;

        for pos in Size::new(1000, 1000).iter() {
            if self.has_overlap(pos) {
                count += 1;
            }
        }

        Ok(count)
    }

    fn part2(&self) -> Result<u64> {
        for c1 in self.claims.iter() {
            if self
                .claims
                .iter()
                .filter(|&c2| c1 != c2)
                .all(|c2| !c1.rect.intersects(&c2.rect))
            {
                return Ok(c1.id);
            }
        }
        Err(AoCError::BadInput)
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
        let input = r#"#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2"#;

        let problem = Problem::parse(input)?;
        assert_eq!(problem.part1()?, 4);
        assert_eq!(problem.part2()?, 3);

        Ok(())
    }
}
