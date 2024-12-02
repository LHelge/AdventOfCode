use aoc::AoCInput;
use std::collections::HashMap;

fn count_bag_content<'a>(
    bag: &'a str,
    rules: &HashMap<&'a str, HashMap<&'a str, usize>>,
    memoization: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(&count) = memoization.get(bag) {
        return count;
    };

    let count: usize = rules
        .get(bag)
        .unwrap()
        .iter()
        .map(|(&bag, &num_bags)| {
            let mut count = 1; // Count the containing bag as well
            count += count_bag_content(bag, rules, memoization);
            count * num_bags
        })
        .sum();

    memoization.insert(bag, count);
    count
}

fn does_bag_contain(
    needle: &str,
    haystack: &str,
    rules: &HashMap<&str, HashMap<&str, usize>>,
) -> bool {
    for &content in rules.get(haystack).unwrap().keys() {
        if content == needle || does_bag_contain(needle, content, rules) {
            return true;
        }
    }

    false
}

fn parse_rules(input: &str) -> HashMap<&str, HashMap<&str, usize>> {
    let mut rules = HashMap::new();

    for rule in input.lines() {
        let (bag, content) = rule.split_once(" bags contain ").unwrap();

        let content = if content.starts_with("no") {
            HashMap::new()
        } else {
            content
                .split(", ")
                .map(|c| {
                    let (amount, bag) = c.split_once(' ').unwrap();
                    let (bag, _) = bag.rsplit_once(' ').unwrap();
                    (bag, amount.parse().unwrap())
                })
                .collect()
        };

        rules.insert(bag, content);
    }

    rules
}

fn solve_task(input: &str) -> (usize, usize) {
    let rules = parse_rules(input);

    let task1 = rules
        .iter()
        .filter(|(&bag, _)| does_bag_contain("shiny gold", bag, &rules))
        .count();

    let mut memoization = HashMap::new();
    let task2 = count_bag_content("shiny gold", &rules, &mut memoization);
    (task1, task2)
}

fn main() {
    let input = AoCInput::from_env()
        .expect("SESSION env variable not found")
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
    fn example1() {
        let input = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#;

        let (example1, example2) = solve_task(input);
        assert_eq!(example1, 4);
        assert_eq!(example2, 32);
    }

    #[test]
    fn example2() {
        let input = r#"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."#;

        let (example1, example2) = solve_task(input);
        assert_eq!(example1, 0);
        assert_eq!(example2, 126);
    }
}
