#[derive(Debug)]
enum Action {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Outcome {
    Win,
    Loose,
    Draw,
}

fn to_action(action: &str) -> Option<Action> {
    match action {
        "A" | "X" => Some(Action::Rock),
        "B" | "Y" => Some(Action::Paper),
        "C" | "Z" => Some(Action::Scissors),
        _ => None,
    }
}

fn to_outcome(outcome: &str) -> Option<Outcome> {
    match outcome {
        "X" => Some(Outcome::Loose),
        "Y" => Some(Outcome::Draw),
        "Z" => Some(Outcome::Win),
        _ => None,
    }
}

fn judge(opponent: &Action, you: &Action) -> Outcome {
    match opponent {
        Action::Rock => match you {
            Action::Rock => Outcome::Draw,
            Action::Paper => Outcome::Win,
            Action::Scissors => Outcome::Loose,
        },
        Action::Paper => match you {
            Action::Rock => Outcome::Loose,
            Action::Paper => Outcome::Draw,
            Action::Scissors => Outcome::Win,
        },
        Action::Scissors => match you {
            Action::Rock => Outcome::Win,
            Action::Paper => Outcome::Loose,
            Action::Scissors => Outcome::Draw,
        },
    }
}

fn needed_action(opponent: &Action, expected_outcome: &Outcome) -> Action {
    match opponent {
        Action::Rock => match expected_outcome {
            Outcome::Win => Action::Paper,
            Outcome::Loose => Action::Scissors,
            Outcome::Draw => Action::Rock,
        },
        Action::Paper => match expected_outcome {
            Outcome::Win => Action::Scissors,
            Outcome::Loose => Action::Rock,
            Outcome::Draw => Action::Paper,
        },
        Action::Scissors => match expected_outcome {
            Outcome::Win => Action::Rock,
            Outcome::Loose => Action::Paper,
            Outcome::Draw => Action::Scissors,
        },
    }
}

fn score(outcome: &Outcome, you: &Action) -> u64 {
    let mut score = 0;

    match outcome {
        Outcome::Win => score += 6,
        Outcome::Draw => score += 3,
        Outcome::Loose => score += 0,
    }

    match you {
        Action::Rock => score += 1,
        Action::Paper => score += 2,
        Action::Scissors => score += 3,
    }

    score
}

pub fn solve_task(input: &str) -> (u64, u64) {
    let mut score1 = 0;
    let mut score2 = 0;
    for (index, line) in input.lines().enumerate() {
        let game: Vec<&str> = line.split_whitespace().collect();

        if game.len() == 2 {
            let opponent = to_action(game[0]).unwrap();

            // Part 1
            let you = to_action(game[1]).unwrap();
            let outcome = judge(&opponent, &you);
            score1 += score(&outcome, &you);

            // Part 2
            let outcome = to_outcome(game[1]).unwrap();
            let you = needed_action(&opponent, &outcome);
            score2 += score(&outcome, &you);
        } else {
            panic!("Bad input {} on line {}", line, index);
        }
    }

    (score1, score2)
}

fn main() {
    let input = aoc::get_input(
        2022,
        2,
        &std::env::var("SESSION").expect("SESSION environment variable not set"),
    )
    .unwrap();

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2022d02 {
    use super::*;

    #[test]
    fn examples() {
        let input = r#"A Y
        B X
        C Z"#;

        let (example1, example2) = solve_task(input);
        assert_eq!(example1, 15);
        assert_eq!(example2, 12)
    }
}
