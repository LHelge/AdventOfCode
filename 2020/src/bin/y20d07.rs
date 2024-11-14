use aoc::AoCInput;

fn solve_task(_input: &str) -> (usize, usize) {
    let task1 = 0;
    let task2 = 0;
    (task1, task2)
}

fn main() {
    let input = AoCInput::from_env()
        .get_input(2020, 7)
        .expect("Could not fetch input");

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2020d07 {
    use super::*;

    #[test]
    fn examples() {
        let input = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#;

        let (example1, _example2) = solve_task(input);
        assert_eq!(example1, 4);
    }
}
