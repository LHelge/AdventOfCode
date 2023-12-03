# AdventOfCode
Repository for keeping my [Advent of Code](https://adventofcode.com/) solutions in Rust

**Year:**
 - [2021](2021/README.md)
 - [2022](2022/README.md)
 - [2023](2023/README.md)


## Usage
The projects is defined as a cargo workspace with a subcrate for each year. Each day is defined as a separate binary. To run the code you need to have cargo and [Rust](https://www.rust-lang.org/) installed.

### Tests
There is a test defined for each of the daily examples, those can be run with `cargo test` either for a specific year or for the whole workspace

### Run
To calculate the answer for a given day run the binary with the following commmand (to run the 2023 day 3 solution):
```bash
cargo run --bin y23d03
```

Your problem input will be fetched automatically and since your input is unique, you need to set a valid session token as an environment variable
```bash
export SESSION=xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

The session token can be found by inspecting the cookies using your browsers developer tools.
