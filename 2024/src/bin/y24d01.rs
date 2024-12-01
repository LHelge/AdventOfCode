use aoc::AoCInput;

fn parse_line(line: &str) -> (u64, u64) {
    let (a, b) = line.split_once(' ').unwrap();
    (a.trim().parse().unwrap(), b.trim().parse().unwrap())
}

fn parse(input: &str) -> (Vec<u64>, Vec<u64>) {
    let vecs = input.lines().map(parse_line).unzip();

    (vecs.0, vecs.1)
}

fn task1(mut list1: Vec<u64>, mut list2: Vec<u64>) -> u64 {
    list1.sort();
    list2.sort();

    list1
        .iter()
        .zip(list2.iter())
        .map(|(&l1, &l2)| l1.abs_diff(l2))
        .sum()
}

fn task2(list1: &[u64], list2: &[u64]) -> u64 {
    list1
        .iter()
        .map(|&l1| list2.iter().filter(|&&l2| l1 == l2).count() as u64 * l1)
        .sum()
}

fn solve_task(input: &str) -> (u64, u64) {
    let (list1, list2) = parse(input);

    // Do task 2 first since there is no need to own the lists
    let task2 = task2(&list1, &list2);
    let task1 = task1(list1, list2);

    (task1, task2)
}

fn main() {
    let input = AoCInput::from_env()
        .get_input(2024, 1)
        .expect("Could not fetch input");

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2024d01 {
    use super::*;

    #[test]
    fn examples() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

        // Task 1
        let (example1, example2) = solve_task(input);
        assert_eq!(example1, 11);
        assert_eq!(example2, 31);
    }
}
