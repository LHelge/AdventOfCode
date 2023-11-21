pub fn find_last_remaining(
    integers: &Vec<u64>,
    bits: usize,
    mask_fn: fn(&Vec<u64>, usize) -> u64,
) -> u64 {
    let mut integers = integers.clone();

    // Go through until only one is left
    let mut i = 0;
    let mut bit_mask = 2u64.pow(bits as u32 - 1);
    let mut mask = mask_fn(&integers, bits);
    while integers.len() > 1 {
        println!("{:#07b}", integers[i]);
        println!("{:#07b}", mask);
        println!("{:#07b}", bit_mask);
        println!("{:#07b}", integers[i] & mask & bit_mask);
        if integers[i] & mask & bit_mask == 0 {
            integers.remove(i);
            println!("Removed!\n");
        } else {
            i += 1;
            println!("Kept!\n");
        }
        if i >= integers.len() {
            i = 0;
            mask = mask_fn(&integers, bits);
            bit_mask >>= 1;
        }
    }

    println!("Only left: {:?}", integers);

    // Pop and return last one
    integers.pop().unwrap()
}

fn calc_epsilon(integers: &Vec<u64>, bits: usize) -> u64 {
    let mut columns: Vec<usize> = vec![0; bits];

    for i in 0..bits {
        for int in integers {
            if int & 2u64.pow(i as u32) != 0 {
                columns[i] += 1;
            }
        }
    }

    let mut epsilon = 0;
    for (i, &col) in columns.iter().enumerate() {
        if col >= integers.len() / 2 {
            // Could be off by one
            epsilon += 2u64.pow(i as u32);
        }
    }

    epsilon
}

fn calc_gamma(integers: &Vec<u64>, bits: usize) -> u64 {
    let epsilon = calc_epsilon(&integers, bits);
    epsilon ^ (2u64.pow(bits as u32) - 1)
}

pub fn solve_task1(input: &str) -> (u64, u64) {
    // Convert binary strings to integers
    let integers: Vec<u64> = input
        .lines()
        .map(|line| u64::from_str_radix(line.trim(), 2).unwrap())
        .collect();

    // Count number of bits on first line
    let bits = input.lines().next().unwrap().trim().len();

    (calc_epsilon(&integers, bits), calc_gamma(&integers, bits))
}

pub fn solve_task2(input: &str) -> (u64, u64) {
    // Convert binary strings to integers
    let integers: Vec<u64> = input
        .lines()
        .map(|line| u64::from_str_radix(line.trim(), 2).unwrap())
        .collect();

    // Count number of bits on first line
    let bits = input.lines().next().unwrap().trim().len();

    println!("Calculating oxygen");
    let oxygen = find_last_remaining(&integers, bits, calc_epsilon);

    println!("Calculating co2");
    let co2 = find_last_remaining(&integers, bits, calc_gamma);

    (oxygen, co2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_2021_03() {
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

        // Task 1
        let (epsilon, gamma) = solve_task1(input);
        assert_eq!(epsilon * gamma, 198);

        // Task 2
        let (oxygen, co2) = solve_task2(input);
        assert_eq!(oxygen * co2, 230);
    }

    #[test]
    fn tasks_2021_03() {
        let input = aoc_input::get_input(
            2021,
            3,
            &std::env::var("SESSION").expect("SESSION environment variable not set"),
        )
        .unwrap();

        // Task 1
        let (gamma, epsilon) = solve_task1(&input);
        assert_eq!(gamma * epsilon, 2743844);

        // Task 2
        //let oxygen = match_first(&input, gamma);
        //let co2 = match_first(&input, epsilon);
        //assert_eq!(oxygen * co2, 0);
    }
}
