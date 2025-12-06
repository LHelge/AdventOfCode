const YEAR: u16 = 2024;
const DAY: u8 = 15;

use aoc::utils::*;
use aoc::*;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Box,
    BoxEast,
    BoxWest,
    Robot,
}

impl Tile {
    fn is_box(&self) -> bool {
        matches!(self, Self::Box | Self::BoxEast | Self::BoxWest)
    }

    fn is_wide_box(&self) -> bool {
        matches!(self, Self::BoxEast | Self::BoxWest)
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::Wall => write!(f, "#"),
            Tile::Box => write!(f, "O"),
            Tile::BoxEast => write!(f, "]"),
            Tile::BoxWest => write!(f, "["),
            Tile::Robot => write!(f, "@"),
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = AoCError;

    fn try_from(value: char) -> Result<Self> {
        match value {
            '#' => Ok(Self::Wall),
            '.' => Ok(Self::Empty),
            'O' => Ok(Self::Box),
            '@' => Ok(Self::Robot),
            c => Err(AoCError::UnexpectedCharacter(c)),
        }
    }
}

#[derive(Debug, Clone)]
struct Warehouse {
    map: Vec2d<Tile>,
    robot: Position,
    moves: Vec<Direction>,
}

impl Warehouse {
    fn expanded(&self) -> Self {
        let mut data = vec![];

        for line in self.map.data() {
            let mut new_line = vec![];

            for tile in line {
                match tile {
                    Tile::Empty => {
                        new_line.push(Tile::Empty);
                        new_line.push(Tile::Empty);
                    }
                    Tile::Wall => {
                        new_line.push(Tile::Wall);
                        new_line.push(Tile::Wall);
                    }
                    Tile::Box => {
                        new_line.push(Tile::BoxWest);
                        new_line.push(Tile::BoxEast);
                    }
                    Tile::Robot => {
                        new_line.push(Tile::Robot);
                        new_line.push(Tile::Empty);
                    }
                    _ => {}
                }
            }

            data.push(new_line);
        }

        let map = Vec2d::new(data).unwrap(); // This should never fail since original is rectangular
        let robot = map
            .iter()
            .find_map(|(p, t)| if *t == Tile::Robot { Some(p) } else { None })
            .unwrap(); // This should never fail unless the robot is missing in the original map

        Self {
            map,
            robot,
            moves: self.moves.clone(),
        }
    }

    fn move_robot(&mut self, direction: Direction) {
        let new_pos = self.robot + direction.into();

        match self.map.get(new_pos) {
            Some(Tile::Empty) => {
                self.map.swap(self.robot, new_pos).unwrap();
                self.robot = new_pos;
            }
            Some(tile) if tile.is_box() => {
                if self.can_be_pushed(new_pos, direction) {
                    self.push_box(new_pos, direction);
                    self.map.swap(self.robot, new_pos).unwrap();
                    self.robot = new_pos;
                }
            }

            _ => {}
        }
    }

    fn can_be_pushed(&self, pos: Position, direction: Direction) -> bool {
        let new_space = pos + direction.into();

        match (direction, self.map.get(pos)) {
            (_, Some(Tile::Box)) => {
                // Pushing a thin box in any direction
                match self.map.get(new_space) {
                    Some(Tile::Empty) => true,
                    Some(Tile::Wall) => false,
                    Some(tile) if tile.is_box() => self.can_be_pushed(new_space, direction),
                    None => unreachable!("You are pushing into the void"),
                    _ => unreachable!("What the hell are you pushing?"),
                }
            }
            (Direction::West, Some(Tile::BoxEast)) => {
                // Pushing a wide box to the west

                if !matches!(self.map.get(new_space), Some(Tile::BoxWest)) {
                    unreachable!("The box have been split some way");
                }
                let new_space = new_space + direction.into();

                match self.map.get(new_space) {
                    Some(Tile::Empty) => true,
                    Some(Tile::Wall) => false,
                    Some(tile) if tile.is_box() => self.can_be_pushed(new_space, direction),
                    None => unreachable!("You are pushing into the void"),
                    _ => unreachable!("What the hell are you pushing?"),
                }
            }
            (Direction::East, Some(Tile::BoxWest)) => {
                // Pushing a wide box to the east

                if !matches!(self.map.get(new_space), Some(Tile::BoxEast)) {
                    unreachable!("The box have been split some way");
                }
                let new_space = new_space + direction.into();

                match self.map.get(new_space) {
                    Some(Tile::Empty) => true,
                    Some(Tile::Wall) => false,
                    Some(tile) if tile.is_box() => self.can_be_pushed(new_space, direction),
                    None => unreachable!("You are pushing into the void"),
                    _ => unreachable!("What the hell are you pushing?"),
                }
            }
            (dir, Some(Tile::BoxEast)) if dir.is_north_south() => {
                // Pushing the eastern part of a wide box to the north or south

                if !matches!(
                    self.map.get(pos + Direction::West.into()),
                    Some(Tile::BoxWest)
                ) {
                    unreachable!("The box have been split some way");
                }

                let new_space_east = new_space;
                let new_space_west = new_space + Direction::West.into();

                match (self.map.get(new_space_west), self.map.get(new_space_east)) {
                    (Some(Tile::Empty), Some(Tile::Empty)) => true,
                    (Some(Tile::Wall), _) | (_, Some(Tile::Wall)) => false,
                    (Some(tile), Some(Tile::Empty)) if tile.is_box() => {
                        self.can_be_pushed(new_space_west, direction)
                    }
                    (Some(Tile::Empty), Some(tile)) if tile.is_box() => {
                        self.can_be_pushed(new_space_east, direction)
                    }
                    (Some(tile1), Some(tile2)) if tile1.is_box() && tile2.is_box() => {
                        self.can_be_pushed(new_space_west, direction)
                            && self.can_be_pushed(new_space_east, direction)
                    }
                    (None, _) | (_, None) => unreachable!("You are pushing into the void"),
                    _ => unreachable!("What the hell are you pushing?"),
                }
            }
            (dir, Some(Tile::BoxWest)) if dir.is_north_south() => {
                // Pushing the western part of a wide box to the north or south

                if !matches!(
                    self.map.get(pos + Direction::East.into()),
                    Some(Tile::BoxEast)
                ) {
                    unreachable!("The box have been split some way");
                }

                let new_space_east = new_space + Direction::East.into();
                let new_space_west = new_space;

                match (self.map.get(new_space_west), self.map.get(new_space_east)) {
                    (Some(Tile::Empty), Some(Tile::Empty)) => true,
                    (Some(Tile::Wall), _) | (_, Some(Tile::Wall)) => false,
                    (Some(tile), Some(Tile::Empty)) if tile.is_box() => {
                        self.can_be_pushed(new_space_west, direction)
                    }
                    (Some(Tile::Empty), Some(tile)) if tile.is_box() => {
                        self.can_be_pushed(new_space_east, direction)
                    }
                    (Some(tile1), Some(tile2)) if tile1.is_box() && tile2.is_box() => {
                        self.can_be_pushed(new_space_west, direction)
                            && self.can_be_pushed(new_space_east, direction)
                    }
                    (None, _) | (_, None) => unreachable!("You are pushing into the void"),
                    _ => unreachable!("What the hell are you pushing?"),
                }
            }
            _ => unreachable!("You are pushing something that is not a box"),
        }
    }

    fn push_box(&mut self, pos: Position, direction: Direction) {
        match (direction, self.map.get(pos)) {
            (_, Some(Tile::Box)) => {
                let new_space = pos + direction.into();
                match self.map.get(new_space) {
                    Some(Tile::Empty) => {
                        self.map.swap(pos, new_space).unwrap();
                    }
                    Some(tile) if tile.is_box() => {
                        self.push_box(new_space, direction);
                        self.map.swap(pos, new_space).unwrap();
                    }
                    _ => unreachable!("You are pushing into something"),
                }
            }
            (dir, Some(tile)) if dir.is_east_west() && tile.is_wide_box() => {
                let new_space = pos + dir.into() + dir.into();
                match self.map.get(new_space) {
                    Some(Tile::Empty) => {
                        self.map.swap(pos + dir.into(), new_space).unwrap();
                        self.map.swap(pos, pos + dir.into()).unwrap();
                    }
                    Some(tile) if tile.is_box() => {
                        self.push_box(new_space, direction);
                        self.map.swap(pos + dir.into(), new_space).unwrap();
                        self.map.swap(pos, pos + dir.into()).unwrap();
                    }
                    _ => unreachable!("You are pushing into something"),
                }
            }
            (dir, Some(Tile::BoxWest)) if dir.is_north_south() => {
                // Pushing the western part of a wide box to the north or south
                let pos_west = pos;
                let pos_east = pos + Direction::East.into();
                let new_space_west = pos_west + dir.into();
                let new_space_east = pos_east + dir.into();

                match (self.map.get(new_space_west), self.map.get(new_space_east)) {
                    (Some(Tile::Empty), Some(Tile::Empty)) => {
                        self.map.swap(pos_west, new_space_west).unwrap();
                        self.map.swap(pos_east, new_space_east).unwrap();
                    }
                    (Some(tile), Some(Tile::Empty)) if tile.is_box() => {
                        self.push_box(new_space_west, direction);
                        self.map.swap(pos_west, new_space_west).unwrap();
                        self.map.swap(pos_east, new_space_east).unwrap();
                    }
                    (Some(Tile::Empty), Some(tile)) if tile.is_box() => {
                        self.push_box(new_space_east, direction);
                        self.map.swap(pos_west, new_space_west).unwrap();
                        self.map.swap(pos_east, new_space_east).unwrap();
                    }
                    (Some(Tile::BoxWest), Some(Tile::BoxEast)) => {
                        self.push_box(new_space_east, direction);
                        self.map.swap(pos_west, new_space_west).unwrap();
                        self.map.swap(pos_east, new_space_east).unwrap();
                    }
                    (Some(tile1), Some(tile2)) if tile1.is_box() && tile2.is_box() => {
                        self.push_box(new_space_west, direction);
                        self.push_box(new_space_east, direction);
                        self.map.swap(pos_west, new_space_west).unwrap();
                        self.map.swap(pos_east, new_space_east).unwrap();
                    }
                    (None, _) | (_, None) => unreachable!("You are pushing into the void"),
                    _ => unreachable!("What the hell are you pushing?"),
                }
            }
            (dir, Some(Tile::BoxEast)) if dir.is_north_south() => {
                // Pushing the eastern part of a wide box to the north or south
                let pos_west = pos + Direction::West.into();
                let pos_east = pos;
                let new_space_west = pos_west + dir.into();
                let new_space_east = pos_east + dir.into();

                match (self.map.get(new_space_west), self.map.get(new_space_east)) {
                    (Some(Tile::Empty), Some(Tile::Empty)) => {
                        self.map.swap(pos_west, new_space_west).unwrap();
                        self.map.swap(pos_east, new_space_east).unwrap();
                    }
                    (Some(tile), Some(Tile::Empty)) if tile.is_box() => {
                        self.push_box(new_space_west, direction);
                        self.map.swap(pos_west, new_space_west).unwrap();
                        self.map.swap(pos_east, new_space_east).unwrap();
                    }
                    (Some(Tile::Empty), Some(tile)) if tile.is_box() => {
                        self.push_box(new_space_east, direction);
                        self.map.swap(pos_west, new_space_west).unwrap();
                        self.map.swap(pos_east, new_space_east).unwrap();
                    }
                    (Some(Tile::BoxWest), Some(Tile::BoxEast)) => {
                        self.push_box(new_space_east, direction);
                        self.map.swap(pos_west, new_space_west).unwrap();
                        self.map.swap(pos_east, new_space_east).unwrap();
                    }
                    (Some(tile1), Some(tile2)) if tile1.is_box() && tile2.is_box() => {
                        self.push_box(new_space_west, direction);
                        self.push_box(new_space_east, direction);
                        self.map.swap(pos_west, new_space_west).unwrap();
                        self.map.swap(pos_east, new_space_east).unwrap();
                    }
                    (None, _) | (_, None) => unreachable!("You are pushing into the void"),
                    _ => unreachable!("What the hell are you pushing?"),
                }
            }
            (dir, tile) => {
                unreachable!("You are pushing a {tile:?} in the {dir:?} direction");
            }
        }
    }

    fn box_coordinates(&self) -> usize {
        self.map
            .iter()
            .filter_map(|tile| match tile {
                (p, Tile::Box) => Some(p.y * 100 + p.x),
                (p, Tile::BoxWest) => Some(p.y * 100 + p.x),
                _ => None,
            })
            .sum()
    }
}

type ResultType = usize;
type DataType = Warehouse;

fn parse(input: &str) -> Result<DataType> {
    let (map, moves) = input.split_once("\n\n").ok_or(AoCError::BadInput)?;

    let map: Vec2d<Tile> = map.parse()?;

    let moves = moves
        .lines()
        .flat_map(|line| line.chars().map(Direction::try_from))
        .collect::<Result<Vec<Direction>>>()?;

    let robot = map
        .iter()
        .find_map(|(p, t)| if *t == Tile::Robot { Some(p) } else { None })
        .ok_or(AoCError::BadInput)?;

    Ok(Warehouse { map, robot, moves })
}

fn task1(data: &DataType) -> Result<ResultType> {
    let mut warehouse = data.clone();

    for &direction in data.moves.iter() {
        warehouse.move_robot(direction);
    }

    Ok(warehouse.box_coordinates())
}

fn task2(data: &DataType) -> Result<ResultType> {
    let mut warehouse = data.expanded();

    for &direction in data.moves.iter() {
        warehouse.move_robot(direction);
    }

    Ok(warehouse.box_coordinates())
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
    fn examples1() {
        let input = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(2028));
        assert_eq!(task2, Some(1751));
    }

    #[test]
    fn examples2() {
        let input = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(10092));
        assert_eq!(task2, Some(9021));
    }

    #[test]
    fn examples3() {
        let input = r#"#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^"#;

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(908));
        assert_eq!(task2, Some(618));
    }
}
