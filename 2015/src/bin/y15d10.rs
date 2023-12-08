fn look_and_say(input: &str) -> String {
    let mut result = String::new();

    let mut current_char = None;
    let mut count = 0usize;
    for c in input.chars() {
        if current_char == Some(c) {
            count += 1;
        } else {
            if let Some(current) = current_char {
                result.push_str(&count.to_string());
                result.push(current);
            }
            current_char = Some(c);
            count = 1;
        }
    }

    if let Some(current) = current_char {
        result.push_str(&count.to_string());
        result.push(current);
    }

    result
}

fn solve_task(input: &str) -> (usize, usize) {
    let mut task1 = input.to_string();
    for _ in 0..40 {
        task1 = look_and_say(&task1);
    }

    let mut task2 = input.to_string();
    for _ in 0..50 {
        task2 = look_and_say(&task2);
    }

    (task1.len(), task2.len())
}

fn main() {
    let (task1, task2) = solve_task("1113222113");

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2015d10 {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(look_and_say("1"), "11");
        assert_eq!(look_and_say("11"), "21");
        assert_eq!(look_and_say("21"), "1211");
        assert_eq!(look_and_say("1211"), "111221");
        assert_eq!(look_and_say("111221"), "312211");
    }
}
