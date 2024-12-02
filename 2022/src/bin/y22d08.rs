use aoc::AoCInput;

fn find_visible(tree_grid: &[Vec<i32>]) -> Vec<Vec<bool>> {
    let width = tree_grid[0].len();
    let height = tree_grid.len();
    let mut visible = vec![vec![false; width]; height];

    for y in 0..height {
        // Left to right
        let mut highest = -1;
        for x in 0..width {
            if tree_grid[y][x] > highest {
                highest = tree_grid[y][x];
                visible[y][x] = true;
            }
        }

        // Right to left
        let mut highest = -1;
        for x in (0..width).rev() {
            if tree_grid[y][x] > highest {
                highest = tree_grid[y][x];
                visible[y][x] = true;
            }
        }
    }

    for x in 0..width {
        // Top to bottom
        let mut highest = -1;
        for y in 0..height {
            if tree_grid[y][x] > highest {
                highest = tree_grid[y][x];
                visible[y][x] = true;
            }
        }

        // Bottom to top
        let mut highest = -1;
        for y in (0..height).rev() {
            if tree_grid[y][x] > highest {
                highest = tree_grid[y][x];
                visible[y][x] = true;
            }
        }
    }

    visible
}

fn scenic_score(tree_grid: &[Vec<i32>]) -> Vec<Vec<u64>> {
    let width = tree_grid[0].len();
    let height = tree_grid.len();
    let mut scores = vec![vec![0; width]; height];

    // Don't  care about the edge trees since they will have score = 0
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let mut right = 0;
            for x2 in (x + 1)..width {
                right += 1;
                if tree_grid[y][x2] >= tree_grid[y][x] {
                    break;
                }
            }

            let mut left = 0;
            for x2 in (0..x).rev() {
                left += 1;
                if tree_grid[y][x2] >= tree_grid[y][x] {
                    break;
                }
            }

            let mut down = 0;
            for y2 in (y + 1)..height {
                down += 1;
                if tree_grid[y2][x] >= tree_grid[y][x] {
                    break;
                }
            }

            let mut up = 0;
            for y2 in (0..y).rev() {
                up += 1;
                if tree_grid[y2][x] >= tree_grid[y][x] {
                    break;
                }
            }

            scores[y][x] = left * right * up * down;
        }
    }

    scores
}

fn solve_task(input: &str) -> (usize, u64) {
    let tree_grid: Vec<Vec<i32>> = input
        .lines()
        .map(|line| line.bytes().map(|c| (c - b'0') as i32).collect())
        .collect();

    let visible_trees = find_visible(tree_grid.as_slice());
    let task1 = visible_trees.iter().flatten().filter(|&&b| b).count();

    let scenic_scores = scenic_score(&tree_grid);
    let task2 = *scenic_scores.iter().flatten().max().unwrap();

    (task1, task2)
}

fn main() {
    let input = AoCInput::from_env()
        .expect("SESSION env variable not found")
        .get_input(2022, 8)
        .expect("Could not fetch input");

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2022d08 {
    use super::*;

    #[test]
    fn examples() {
        let example_input = r#"30373
25512
65332
33549
35390"#;

        let (example1, example2) = solve_task(example_input);

        assert_eq!(example1, 21);
        assert_eq!(example2, 8);
    }
}
