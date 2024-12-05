const YEAR: u16 = 2024;
const DAY: u8 = 5;
use std::result::Result as StdResult;
use std::{num::ParseIntError, str::FromStr};

use aoc::*;

#[derive(Debug)]
struct Rule {
    before: usize,
    after: usize,
}

impl FromStr for Rule {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self> {
        let (before, after) = s.split_once("|").ok_or(AoCError::BadInput)?;
        let before = before.parse()?;
        let after = after.parse()?;
        Ok(Rule { before, after })
    }
}

#[derive(Debug, Clone)]
struct PageList(Vec<usize>);

impl FromStr for PageList {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self> {
        let pages = s
            .split(',')
            .map(|page| page.parse())
            .collect::<StdResult<Vec<usize>, ParseIntError>>()?;
        Ok(PageList(pages))
    }
}

impl PageList {
    fn fulfill_rule(&self, rule: &Rule) -> bool {
        let first = self.0.iter().position(|&page| page == rule.before);
        let second = self.0.iter().position(|&page| page == rule.after);

        match (first, second) {
            (Some(first), Some(second)) => first < second,
            _ => true,
        }
    }

    fn middle(&self) -> usize {
        self.0[self.0.len() / 2]
    }

    fn correct(&mut self, rule: &Rule) {
        if self.fulfill_rule(rule) {
            return;
        }

        let first = self.0.iter().position(|&page| page == rule.before).unwrap();
        let second = self.0.iter().position(|&page| page == rule.after).unwrap();

        if first > second {
            let temp = self.0.remove(second);
            self.0.insert(first, temp);
        }
    }
}

#[derive(Debug)]
struct PrintQueue {
    rules: Vec<Rule>,
    pages: Vec<PageList>,
}

type ResultType = usize;
type DataType = PrintQueue;

fn parse(input: &str) -> Result<DataType> {
    let (rules, pages) = input.split_once("\n\n").ok_or(AoCError::BadInput)?;

    let rules = rules
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Rule>>>()?;

    let pages = pages
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<PageList>>>()?;

    Ok(PrintQueue { rules, pages })
}

fn task1(data: &DataType) -> Result<ResultType> {
    Ok(data
        .pages
        .iter()
        .filter(|&pages| data.rules.iter().all(|rule| pages.fulfill_rule(rule)))
        .map(PageList::middle)
        .sum())
}

fn task2(data: &DataType) -> Result<ResultType> {
    let mut pages = data.pages.clone();

    Ok(pages
        .iter_mut()
        .filter(|pages| data.rules.iter().any(|rule| !pages.fulfill_rule(rule)))
        .map(|pages| {
            while let Some(violated_rule) = data.rules.iter().find(|rule| !pages.fulfill_rule(rule))
            {
                pages.correct(violated_rule);
            }

            pages.middle()
        })
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
    fn examples() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(143));
        assert_eq!(task2, Some(123));
    }
}
