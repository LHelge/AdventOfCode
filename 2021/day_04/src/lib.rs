#[derive(Debug)]
struct BingoBoard {
    board: [[(u8, bool); 5]; 5],
    won: bool,
}

struct RowIterator<'a> {
    board: &'a BingoBoard,
    row: usize,
    col: usize,
}

impl<'a> Iterator for RowIterator<'a> {
    type Item = &'a (u8, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if self.col < 5 {
            let item = &self.board.board[self.row][self.col];
            self.col += 1;
            Some(item)
        } else {
            None
        }
    }
}

struct ColIterator<'a> {
    board: &'a BingoBoard,
    row: usize,
    col: usize,
}

impl<'a> Iterator for ColIterator<'a> {
    type Item = &'a (u8, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < 5 {
            let item = &self.board.board[self.row][self.col];
            self.row += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl BingoBoard {
    fn new(numbers: Vec<u8>) -> Self {
        let mut board = [[(0, false); 5]; 5];

        for (i, &n) in numbers.iter().enumerate() {
            board[i / 5][i % 5] = (n, false);
        }

        Self { board, won: false }
    }

    fn row(&self, row: usize) -> RowIterator {
        RowIterator {
            board: self,
            row,
            col: 0,
        }
    }

    fn col(&self, col: usize) -> ColIterator {
        ColIterator {
            board: self,
            row: 0,
            col,
        }
    }

    fn draw(&mut self, n: u8) -> bool {
        if self.won {
            return true;
        }

        for row in 0..5 {
            for col in 0..5 {
                if self.board[row][col].0 == n {
                    self.board[row][col].1 = true;

                    if self.row(row).all(|n| n.1) || self.col(col).all(|n| n.1) {
                        self.won = true;
                        return true;
                    }
                }
            }
        }

        false
    }

    fn sum_unmarked(&self) -> u64 {
        self.board
            .iter()
            .flatten()
            .filter(|n| !n.1)
            .fold(0, |acc, n| acc + n.0 as u64)
    }
}

pub fn solve_task(input: &str) -> (u64, u64) {
    let mut lines = input.lines();

    let draws: Vec<u8> = lines
        .next()
        .unwrap()
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    // Skip empty line
    let mut boards = vec![];
    while let Some(_) = lines.next() {
        let mut board: Vec<u8> = vec![];

        for _ in 0..5 {
            board.extend(
                lines
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|n| n.parse::<u8>().unwrap()),
            );
        }

        boards.push(BingoBoard::new(board));
    }

    // Draw numbers
    let mut first: Option<u64> = None;
    for n in draws {
        for i in 0..boards.len() {
            if boards[i].draw(n) {
                if first.is_none() {
                    first = Some(boards[i].sum_unmarked() * n as u64);
                }
                if boards.iter().all(|b| b.won) {
                    return (first.unwrap(), boards[i].sum_unmarked() * n as u64);
                }
            }
        }
    }

    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_2021_04() {
        let input = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19
        
         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6
        
        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7"#;

        let (first, last) = solve_task(input);
        assert_eq!(first, 4512);
        assert_eq!(last, 1924);
    }

    #[test]
    fn tasks_2021_04() {
        let input = aoc_input::get_input(
            2021,
            4,
            &std::env::var("SESSION").expect("SESSION environment variable not set"),
        )
        .unwrap();

        let (first, last) = solve_task(&input);

        // Task 1
        assert_eq!(first, 29440);

        // Task 2
        assert_eq!(last, 13884);
    }
}
