use std::collections::{BTreeSet, HashMap};

mod valves;
use valves::{parse_valves, Valve};

#[derive(Debug, Hash, PartialEq, Eq)]
struct State<'a> {
    position: &'a str,
    time_left: u32,
    opened: BTreeSet<&'a str>,
}

// Recursive search algorithm with a HashMap for caching state
fn max_pressure_released<'a>(
    state: State<'a>,
    valves: &HashMap<&str, Valve<'a>>,
    cache: &mut HashMap<State<'a>, u64>,
) -> u64 {
    // No time left
    if state.time_left == 0 {
        return 0;
    }

    // Check cache if this state has already been
    if let Some(max_pressure) = cache.get(&state) {
        return *max_pressure;
    }

    let mut max_left_to_release = 0;

    // Don't run around if all working valves are already opened
    if state.opened.len() <= valves.iter().filter(|(_, v)| v.flow_rate > 0).count() {
        let valve = valves.get(state.position).unwrap();

        // Investigate all possible moves
        let max_move = valve
            .tunnels
            .iter()
            .map(|v| {
                max_pressure_released(
                    State {
                        position: v,
                        time_left: state.time_left - 1,
                        opened: state.opened.clone(),
                    },
                    valves,
                    cache,
                )
            })
            .max()
            .unwrap();

        // Investigate open
        let mut max_open = 0;
        if valve.flow_rate > 0 && !state.opened.contains(state.position) {
            let mut opened = state.opened.clone();
            opened.insert(state.position);
            max_open = max_pressure_released(
                State {
                    position: state.position,
                    time_left: state.time_left - 1,
                    opened: opened,
                },
                valves,
                cache,
            );
        }

        max_left_to_release = max_move.max(max_open);
    }

    // Calculate pressure released this time step
    let pressure_released_now = state
        .opened
        .iter()
        .map(|v| valves.get(v).unwrap().flow_rate)
        .sum::<u64>();

    // Store result in cache if needed again
    cache.insert(state, pressure_released_now + max_left_to_release);

    pressure_released_now + max_left_to_release
}

// Solve part1
fn task1(valves: &Vec<Valve>) -> u64 {
    let valves: HashMap<&str, Valve> = valves.iter().map(|v| (v.name, v.clone())).collect();

    //dbg!(&valves);

    let mut cache: HashMap<State, u64> = HashMap::new();

    max_pressure_released(
        State {
            position: "AA",
            time_left: 30,
            opened: BTreeSet::new(),
        },
        &valves,
        &mut cache,
    )
}

fn task2(_input: &Vec<Valve>) -> u64 {
    0
}

pub fn solve_task(input: &str) -> (u64, u64) {
    let valves = parse_valves(input).unwrap().1;

    let task1 = task1(&valves);
    let task2 = task2(&valves);

    (task1, task2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_2022_16() {
        let input = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        Valve BB has flow rate=13; tunnels lead to valves CC, AA
        Valve CC has flow rate=2; tunnels lead to valves DD, BB
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
        Valve EE has flow rate=3; tunnels lead to valves FF, DD
        Valve FF has flow rate=0; tunnels lead to valves EE, GG
        Valve GG has flow rate=0; tunnels lead to valves FF, HH
        Valve HH has flow rate=22; tunnel leads to valve GG
        Valve II has flow rate=0; tunnels lead to valves AA, JJ
        Valve JJ has flow rate=21; tunnel leads to valve II"#;

        let (example1, _example2) = solve_task(input);

        assert_eq!(example1, 1651);
        //assert_eq!(example2, 0);
    }

    #[test]
    fn tasks_2022_16() {
        let input = aoc_input::get_input(
            2022,
            16,
            &std::env::var("SESSION").expect("SESSION environment variable not set"),
        )
        .unwrap();

        let (task1, _task2) = solve_task(&input);

        assert_eq!(task1, 1673);
        //assert_eq!(task2, 0);
    }
}
