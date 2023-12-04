use aoc::AoCError;
use std::{
    collections::{BTreeMap, BTreeSet},
    str::FromStr,
};

#[derive(Debug, Clone)]
struct Card {
    id: usize,
    numbers: BTreeSet<u8>,
    winning: BTreeSet<u8>,
}

impl FromStr for Card {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (card, numbers) = s.split_once(": ").ok_or(AoCError::BadInput)?;
        let id = card.split_whitespace().nth(1).ok_or(AoCError::BadInput)?;

        let (winning, numbers) = numbers.split_once(" | ").ok_or(AoCError::BadInput)?;

        let id = id.parse()?;
        let numbers = numbers
            .split_whitespace()
            .map(|n| n.parse().expect("Bad input"))
            .collect();
        let winning = winning
            .split_whitespace()
            .map(|n| n.parse().expect("Bad input"))
            .collect();

        Ok(Card {
            id,
            numbers,
            winning,
        })
    }
}

fn number_of_cards_won(
    card: &Card,
    base_cards: &BTreeMap<usize, Card>,
    cache: &mut BTreeMap<usize, usize>,
) -> usize {
    if let Some(c) = cache.get(&card.id) {
        return *c;
    }

    let mut won_cards = 0;
    let correct = card.winning.intersection(&card.numbers).count();

    for i in 1..=correct {
        won_cards += 1;
        if let Some(card) = base_cards.get(&(card.id + i)) {
            won_cards += number_of_cards_won(card, base_cards, cache);
        }
    }

    cache.insert(card.id, won_cards);

    won_cards
}

fn solve_task(input: &str) -> (u64, u64) {
    let base_cards = input
        .lines()
        .map(|l| {
            let card = l.parse::<Card>().expect("Bad input: {}");
            (card.id, card)
        })
        .collect::<BTreeMap<usize, Card>>();

    let mut task1 = 0;
    for card in base_cards.values() {
        let correct = card.winning.intersection(&card.numbers).count();
        if correct > 0 {
            task1 += 2u64.pow(correct as u32 - 1);
        }
    }

    let mut task2 = base_cards.len();
    let mut cache: BTreeMap<usize, usize> = BTreeMap::new();
    for card in base_cards.values() {
        task2 += number_of_cards_won(card, &base_cards, &mut cache);
    }

    (task1, task2 as u64)
}

fn main() {
    let input = aoc::get_input(
        2023,
        4,
        &std::env::var("SESSION").expect("SESSION environment variable not set"),
    )
    .unwrap();

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2023d04 {
    use super::*;

    #[test]
    fn examples() {
        let example_input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

        let (example1, example2) = solve_task(example_input);

        assert_eq!(example1, 13);
        assert_eq!(example2, 30);
    }
}
