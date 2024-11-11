use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
    str::FromStr,
};

use aoc::{AoCError, AoCInput};

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
enum Card {
    Joker = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            '*' => Self::Joker,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!("Invalid card: {}", c),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Hand {
    cards: [Card; 5],
}

impl FromStr for Hand {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 5 {
            return Err(AoCError::BadInput);
        }

        let mut cards = [Card::Two; 5];
        for (i, c) in s.chars().enumerate() {
            cards[i] = Card::from(c);
        }
        Ok(Self { cards })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPairs = 2,
    OnePair = 1,
    HighCard = 0,
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut counts: HashMap<Card, usize> = HashMap::new();
        for card in self.cards.iter() {
            *counts.entry(*card).or_insert(0) += 1;
        }

        let jokers = counts.remove(&Card::Joker).unwrap_or(0);
        if jokers == 5 {
            return HandType::FiveOfAKind;
        }

        for &num in counts.values() {
            match (num + jokers, counts.len(), jokers) {
                (5, _, _) => return HandType::FiveOfAKind,
                (4, _, _) => return HandType::FourOfAKind,
                (3, 2, 0) | (3, 2, 1) | (2, 2, 0) => return HandType::FullHouse,
                (3, 3, _) => return HandType::ThreeOfAKind,
                (2, 4, _) => return HandType::OnePair,
                (2, 3, 0) => return HandType::TwoPairs,
                _ => {}
            }
        }

        if jokers > 0 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_type = self.hand_type();
        let other_type = other.hand_type();

        //println!("\nComparing {:?} to {:?}", self, other);
        //println!("Self type: {:?}, Other type: {:?}", self_type, other_type);

        if self_type != other_type {
            return self_type.cmp(&other_type);
        }

        // Find first differing card
        if let Some((&self_card, &other_card)) = self
            .cards
            .iter()
            .zip(other.cards.iter())
            .find(|(s, o)| s != o)
        {
            //println!("Diff cards: {:?} - {:?}", self_card, other_card);
            self_card.cmp(&other_card)
        } else {
            std::cmp::Ordering::Equal
        }
    }
}

fn solve_task(input: &str) -> (u64, u64) {
    let hands: BTreeMap<Hand, u64> = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let hand = parts.next().unwrap().parse::<Hand>().unwrap();
            let value = parts.next().unwrap().parse::<u64>().unwrap();
            (hand, value)
        })
        .collect();

    let mut task1 = 0;
    for (i, (_, value)) in hands.iter().enumerate() {
        task1 += value * (i + 1) as u64;
    }

    let hands_with_jokers: BTreeMap<Hand, u64> = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let hand = parts
                .next()
                .unwrap()
                .replace('J', "*")
                .parse::<Hand>()
                .unwrap();
            let value = parts.next().unwrap().parse::<u64>().unwrap();
            (hand, value)
        })
        .collect();

    let mut task2 = 0;
    for (i, (_, value)) in hands_with_jokers.iter().enumerate() {
        task2 += value * (i + 1) as u64;
    }

    (task1, task2)
}

fn main() {
    let input = AoCInput::from_env()
        .get_input(2023, 7)
        .expect("Could not fetch input");

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2023d07 {
    use super::*;

    #[test]
    fn examples() {
        let example_input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

        let (example1, example2) = solve_task(example_input);

        assert_eq!(example1, 6440);
        assert_eq!(example2, 5905);
    }

    #[test]
    fn hands() {
        let five_of_a_kind: Hand = "AAAAA".parse().unwrap();
        let four_of_a_kind: Hand = "KKKKT".parse().unwrap();
        let full_house: Hand = "QJJQQ".parse().unwrap();
        let three_of_a_kind: Hand = "TTTQJ".parse().unwrap();
        let two_pairs: Hand = "JJ3QQ".parse().unwrap();
        let one_pair1: Hand = "JJTQK".parse().unwrap();
        let one_pair2: Hand = "JJ9QT".parse().unwrap();
        let high_card: Hand = "KQJT9".parse().unwrap();

        assert_eq!(five_of_a_kind.hand_type(), HandType::FiveOfAKind);
        assert_eq!(four_of_a_kind.hand_type(), HandType::FourOfAKind);
        assert_eq!(full_house.hand_type(), HandType::FullHouse);
        assert_eq!(three_of_a_kind.hand_type(), HandType::ThreeOfAKind);
        assert_eq!(two_pairs.hand_type(), HandType::TwoPairs);
        assert_eq!(one_pair1.hand_type(), HandType::OnePair);
        assert_eq!(one_pair2.hand_type(), HandType::OnePair);
        assert_eq!(high_card.hand_type(), HandType::HighCard);

        assert!(five_of_a_kind > four_of_a_kind);
        assert!(four_of_a_kind > full_house);
        assert!(full_house > three_of_a_kind);
        assert!(three_of_a_kind > two_pairs);
        assert!(two_pairs > one_pair1);
        assert!(one_pair1 > one_pair2);
        assert!(one_pair2 > high_card);
    }

    #[test]
    fn hands_with_jokers() {
        let five_of_a_kind1: Hand = "A*AAA".parse().unwrap();
        let five_of_a_kind2: Hand = "*****".parse().unwrap();
        let four_of_a_kind: Hand = "KT**T".parse().unwrap();
        let full_house: Hand = "KKTT*".parse().unwrap();
        let three_of_a_kind: Hand = "T*TQJ".parse().unwrap();
        let two_pairs: Hand = "JJ3QQ".parse().unwrap(); //Two pairs with jokers is not possible
        let one_pair1: Hand = "JJTQK".parse().unwrap();
        let one_pair2: Hand = "*J9QT".parse().unwrap();
        let high_card: Hand = "KQJT9".parse().unwrap();

        assert_eq!(five_of_a_kind1.hand_type(), HandType::FiveOfAKind);
        assert_eq!(five_of_a_kind2.hand_type(), HandType::FiveOfAKind);
        assert_eq!(four_of_a_kind.hand_type(), HandType::FourOfAKind);
        assert_eq!(full_house.hand_type(), HandType::FullHouse);
        assert_eq!(three_of_a_kind.hand_type(), HandType::ThreeOfAKind);
        assert_eq!(two_pairs.hand_type(), HandType::TwoPairs);
        assert_eq!(one_pair1.hand_type(), HandType::OnePair);
        assert_eq!(one_pair2.hand_type(), HandType::OnePair);
        assert_eq!(high_card.hand_type(), HandType::HighCard);

        assert!(five_of_a_kind1 > five_of_a_kind2);
        assert!(five_of_a_kind2 > four_of_a_kind);
        assert!(four_of_a_kind > full_house);
        assert!(full_house > three_of_a_kind);
        assert!(three_of_a_kind > two_pairs);
        assert!(two_pairs > one_pair1);
        assert!(one_pair1 > one_pair2);
        assert!(one_pair2 > high_card);
    }
}
