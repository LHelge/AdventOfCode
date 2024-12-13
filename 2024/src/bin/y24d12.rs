const YEAR: u16 = 2024;
const DAY: u8 = 12;
use std::collections::HashSet;

use aoc::*;
use vec2d::{Direction, Position, Vec2d};

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

struct Region {
    id: char,
    positions: HashSet<Position>,
}

impl Region {
    fn new(id: char) -> Self {
        Self {
            id,
            positions: HashSet::new(),
        }
    }

    fn add(&mut self, position: Position) {
        self.positions.insert(position);
    }

    fn contains(&self, position: &Position) -> bool {
        self.positions.contains(position)
    }

    fn size(&self) -> usize {
        self.positions.len()
    }

    fn perimeter(&self) -> usize {
        let mut perimeter = 0;
        for &position in self.positions.iter() {
            for direction in DIRECTIONS {
                let neighbour = position + direction.into();
                if !self.contains(&neighbour) {
                    perimeter += 1;
                }
            }
        }
        perimeter
    }

    fn sides(&self) -> usize {
        // Count the number of corners instead of sides, much easier
        let mut corners = 0;

        for &position in self.positions.iter() {
            let north = position + Direction::North.into();
            let east = position + Direction::East.into();
            let south = position + Direction::South.into();
            let west = position + Direction::West.into();

            // Outer corners
            if !self.contains(&north) && !self.contains(&west) {
                corners += 1;
            }
            if !self.contains(&north) && !self.contains(&east) {
                corners += 1;
            }
            if !self.contains(&south) && !self.contains(&east) {
                corners += 1;
            }
            if !self.contains(&south) && !self.contains(&west) {
                corners += 1;
            }

            let north_east = position + Direction::NorthEast.into();
            let north_west = position + Direction::NorthWest.into();
            let south_east = position + Direction::SouthEast.into();
            let south_west = position + Direction::SouthWest.into();

            // Inner corners
            if self.contains(&north) && self.contains(&east) && !self.contains(&north_east) {
                corners += 1;
            }
            if self.contains(&north) && self.contains(&west) && !self.contains(&north_west) {
                corners += 1;
            }
            if self.contains(&south) && self.contains(&east) && !self.contains(&south_east) {
                corners += 1;
            }
            if self.contains(&south) && self.contains(&west) && !self.contains(&south_west) {
                corners += 1;
            }
        }

        corners
    }
}

type ResultType = usize;
type DataType = Vec<Region>;

fn find_region(map: &Vec2d<char>, position: Position, region: &mut Region) {
    const DIRECTIONS: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    for direction in DIRECTIONS {
        let neighbour = position + direction.into();
        if let Some(&id) = map.get(neighbour) {
            if id == region.id && !region.contains(&neighbour) {
                region.add(neighbour);
                find_region(map, neighbour, region);
            }
        }
    }
}

fn parse(input: &str) -> Result<DataType> {
    let data = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<_>>();
    let data = Vec2d::new(data)?;

    let mut regions: Vec<Region> = vec![];

    for (position, &id) in data.iter() {
        if regions.iter().any(|region| region.contains(&position)) {
            // Already in a region
            continue;
        }

        let mut region = Region::new(id);
        region.add(position);
        find_region(&data, position, &mut region);
        regions.push(region);
    }

    Ok(regions)
}

fn task1(data: &DataType) -> Result<ResultType> {
    Ok(data
        .iter()
        .map(|region| region.size() * region.perimeter())
        .sum())
}

fn task2(data: &DataType) -> Result<ResultType> {
    Ok(data
        .iter()
        .map(|region| region.size() * region.sides())
        .sum())
}

fn main() -> Result<()> {
    let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
    solution.solve_for_answer(YEAR, DAY)?;

    println!("Advent of Code {YEAR} day {DAY}");
    println!("-------------------------");
    println!("{solution}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = r#"AAAA
BBCD
BBCC
EEEC"#;

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(140));
        assert_eq!(task2, Some(80));
    }

    #[test]
    fn example2() {
        let input = r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#;

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(772));
        assert_eq!(task2, Some(436));
    }

    #[test]
    fn example3() {
        let input = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(1930));
        assert_eq!(task2, Some(1206));
    }

    #[test]
    fn example4() {
        let input = r#"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"#;

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(692));
        assert_eq!(task2, Some(236));
    }

    #[test]
    fn example5() {
        let input = r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"#;

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(1184));
        assert_eq!(task2, Some(368));
    }
}
