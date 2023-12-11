use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Command<'a> {
    Cd(Cd<'a>),
    Ls(Vec<FsObj<'a>>),
}

#[derive(Debug, Clone)]
enum Cd<'a> {
    Root,
    Up,
    Down(&'a str),
}

#[derive(Debug, Clone)]
enum FsObj<'a> {
    File(&'a str, usize),
    Dir(&'a str),
}

fn solve_task(input: &str) -> (usize, usize) {
    // Parse input commands
    let mut commands: Vec<Command> = Vec::new();
    let mut lines = input.lines().peekable();
    while let Some(line) = lines.next() {
        if line.starts_with("$ cd") {
            match &line[5..] {
                "/" => commands.push(Command::Cd(Cd::Root)),
                ".." => commands.push(Command::Cd(Cd::Up)),
                dir => commands.push(Command::Cd(Cd::Down(dir))),
            }
        } else if line.starts_with("$ ls") {
            let mut content: Vec<FsObj> = vec![];
            while lines.peek().is_some() && !lines.peek().unwrap().starts_with("$ ") {
                let obj = lines.next().unwrap();
                if obj.starts_with("dir ") {
                    content.push(FsObj::Dir(&obj[5..]));
                } else {
                    let parts: Vec<&str> = obj.split(' ').collect();
                    content.push(FsObj::File(parts[1], parts[0].parse().unwrap()));
                }
            }
            commands.push(Command::Ls(content));
        }
    }

    // Calculate file sizes
    let mut cwd: Vec<&str> = vec![];
    let mut sizes: HashMap<String, usize> = HashMap::new();
    for cmd in commands {
        match cmd {
            Command::Cd(cd) => match cd {
                Cd::Root => {
                    cwd.clear();
                    cwd.push("")
                }
                Cd::Up => {
                    cwd.pop();
                }
                Cd::Down(dir) => cwd.push(dir),
            },
            Command::Ls(files) => {
                let mut dir_size: usize = 0;
                for file in files {
                    if let FsObj::File(.., size) = file {
                        dir_size += size;
                    }
                }

                for i in 0..cwd.len() {
                    sizes
                        .entry(cwd[0..=i].join("/"))
                        .and_modify(|v| *v += dir_size)
                        .or_insert(dir_size);
                }
            }
        }
    }

    // Calculate task1 (all files smaller than 100000 bytes)
    let task1 = sizes.values().filter(|&&s| s < 100000).sum();

    // Calculate task2 (find smalles directory to delete to have at least 30 MB free out of 70 MB)
    let to_delete = sizes.get("").unwrap() + 30_000_000 - 70_000_000;
    let task2 = sizes.values().fold(
        usize::MAX,
        |acc, &s| {
            if s >= to_delete && s < acc {
                s
            } else {
                acc
            }
        },
    );

    (task1, task2)
}

fn main() {
    let input = aoc::get_input(
        2022,
        7,
        &std::env::var("SESSION").expect("SESSION environment variable not set"),
    )
    .unwrap();

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2022d07 {
    use super::*;

    #[test]
    fn examples() {
        let example_input = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

        let (example1, example2) = solve_task(example_input);

        assert_eq!(example1, 95437);
        assert_eq!(example2, 24933642);
    }
}
