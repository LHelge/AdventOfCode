/// Calculate the gamma value for the given integers.
fn calc_gamma_epsilon(integers: &Vec<u64>, bits: usize) -> (u64, u64) {
    let mut columns: Vec<usize> = vec![0; bits];

    // Count the number of 1s in each column.
    for (i, col) in columns.iter_mut().enumerate() {
        for int in integers {
            if int & 2u64.pow(i as u32) != 0 {
                *col += 1;
            }
        }
    }

    // Calculate the gamma value.
    let mut gamma = 0;
    for (i, &col) in columns.iter().enumerate() {
        if col > (integers.len() - 1) / 2 {
            gamma += 2u64.pow(i as u32);
        }
    }

    // Epsilon is the bitwise inverse of gamma.
    let epsilon = gamma ^ (2u64.pow(bits as u32) - 1);

    (gamma, epsilon)
}

fn solve_task(input: &str) -> (u64, u64) {
    // Count the number of columns and convert the input to integers.
    let bits = input.lines().next().unwrap().trim().len();
    let integers = input
        .lines()
        .map(|line| u64::from_str_radix(line.trim(), 2).unwrap())
        .collect::<Vec<u64>>();

    // Calculate the gamma and epsilon values
    let (gamma, epsilon) = calc_gamma_epsilon(&integers, bits);

    let mut oxygen = integers.clone();
    let mut mask = 2u64.pow(bits as u32 - 1);
    while oxygen.len() > 1 {
        let (gamma, _) = calc_gamma_epsilon(&oxygen, bits);

        oxygen.retain(|&o| !(o ^ gamma) & mask != 0);

        mask >>= 1;
    }

    let mut co2 = integers.clone();
    let mut mask = 2u64.pow(bits as u32 - 1);
    while co2.len() > 1 {
        let (_, epsilon) = calc_gamma_epsilon(&co2, bits);

        co2.retain(|&o| !(o ^ epsilon) & mask != 0);

        mask >>= 1;
    }

    (epsilon * gamma, oxygen[0] * co2[0])
}

fn main() {
    let input = aoc::get_input(
        2021,
        3,
        &std::env::var("SESSION").expect("SESSION environment variable not set"),
    )
    .unwrap();

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2021d03 {
    use super::*;

    #[test]
    fn examples3() {
        let input = r#"00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010"#;

        let (example1, example2) = solve_task(input);
        assert_eq!(example1, 198);
        assert_eq!(example2, 230);
    }
}
