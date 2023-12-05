fn solve_task(input: &str) -> (usize, usize) {
    let mut task1: Option<usize> = None;
    let mut task2: Option<usize> = None;
    for i in 0.. {
        let hash = format!("{:x}", md5::compute(format!("{}{}", input, i)));

        if task1.is_none() && hash.starts_with("00000") {
            task1 = Some(i);
        }

        if task2.is_none() && hash.starts_with("000000") {
            task2 = Some(i);
        }

        if task1.is_some() && task2.is_some() {
            break;
        }
    }

    (task1.unwrap(), task2.unwrap())
}

fn main() {
    let (task1, task2) = solve_task("yzbqklnj");

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2015d04 {
    use super::*;

    #[test]
    #[ignore] // Takes too long
    fn examples() {
        let (example1, _) = solve_task("abcdef");
        assert_eq!(example1, 609043);

        let (example1, _) = solve_task("pqrstuv");
        assert_eq!(example1, 1048970);
    }
}
