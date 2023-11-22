use std::collections::{BTreeSet, HashMap};

mod valves;
use valves::{parse_valves, Valve};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct State<'a> {
    my_position: &'a str,
    elefant_position: &'a str,
    time_left: u32,
    opened: BTreeSet<&'a str>,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Action<'a> {
    Open(&'a str),
    Move(&'a str),
}

// Recursive search algorithm with a HashMap for caching state
fn max_pressure_released_with_elefant<'a>(
    mut state: State<'a>,
    valves: &HashMap<&str, Valve<'a>>,
    cache: &mut HashMap<State<'a>, u64>,
    my_action: Option<Action<'a>>,
    elefant_action: Option<Action<'a>>,
) -> u64 {
    match my_action {
        Some(Action::Open(valve)) => {
            state.opened.insert(valve);
        }
        Some(Action::Move(valve)) => {
            state.my_position = valve;
        }
        None => {}
    }
    match elefant_action {
        Some(Action::Open(valve)) => {
            state.opened.insert(valve);
        }
        Some(Action::Move(valve)) => {
            state.elefant_position = valve;
        }
        None => {}
    }

    if my_action.is_some() || elefant_action.is_some() {
        state.time_left -= 1;
    }

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
        let my_valve = valves.get(state.my_position).unwrap();
        let elefant_valve = valves.get(state.elefant_position).unwrap();

        // Investigate all possible moves
        let mut my_actions: Vec<Action> = vec![];
        if !state.opened.contains(my_valve.name) && my_valve.flow_rate > 0 {
            my_actions.push(Action::Open(my_valve.name));
        }
        my_actions.extend(my_valve.tunnels.iter().map(|v| Action::Move(v)));

        let mut elefant_actions: Vec<Action> = vec![];
        if !state.opened.contains(elefant_valve.name) && elefant_valve.flow_rate > 0 {
            elefant_actions.push(Action::Open(elefant_valve.name));
        }
        elefant_actions.extend(elefant_valve.tunnels.iter().map(|v| Action::Move(v)));

        // Create all possible permutations of actions
        let mut actions = vec![];
        for my in my_actions {
            for elefant in &elefant_actions {
                actions.push((my, *elefant));
            }
        }

        max_left_to_release = actions
            .iter()
            .map(|(my, elefant)| {
                max_pressure_released_with_elefant(
                    state.clone(),
                    valves,
                    cache,
                    Some(*my),
                    Some(*elefant),
                )
            })
            .max()
            .unwrap();
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

// Recursive search algorithm with a HashMap for caching state
fn max_pressure_released<'a>(
    mut state: State<'a>,
    valves: &HashMap<&str, Valve<'a>>,
    cache: &mut HashMap<State<'a>, u64>,
    action: Option<Action<'a>>,
) -> u64 {
    match action {
        Some(Action::Open(valve)) => {
            state.opened.insert(valve);
        }
        Some(Action::Move(valve)) => {
            state.my_position = valve;
        }
        None => {}
    }

    if action.is_some() {
        state.time_left -= 1;
    }

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
        let valve = valves.get(state.my_position).unwrap();

        // Investigate all possible moves
        let max_move = valve
            .tunnels
            .iter()
            .map(|v| max_pressure_released(state.clone(), valves, cache, Some(Action::Move(v))))
            .max()
            .unwrap();

        // Investigate open
        let mut max_open = 0;
        if valve.flow_rate > 0 && !state.opened.contains(state.my_position) {
            max_open = max_pressure_released(
                state.clone(),
                valves,
                cache,
                Some(Action::Open(state.my_position)),
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
fn task1(valves: &HashMap<&str, Valve>) -> u64 {
    let mut cache: HashMap<State, u64> = HashMap::new();

    max_pressure_released(
        State {
            my_position: "AA",
            elefant_position: "AA",
            time_left: 30,
            opened: BTreeSet::new(),
        },
        &valves,
        &mut cache,
        None,
    )
}

fn task2(valves: &HashMap<&str, Valve>) -> u64 {
    let mut cache: HashMap<State, u64> = HashMap::new();

    max_pressure_released_with_elefant(
        State {
            my_position: "AA",
            elefant_position: "AA",
            time_left: 30 - 4,
            opened: BTreeSet::new(),
        },
        &valves,
        &mut cache,
        None,
        None,
    )
}

pub fn solve_task(input: &str) -> (u64, u64) {
    let valves = parse_valves(input).unwrap().1;
    let valves: HashMap<&str, Valve> = valves.iter().map(|v| (v.name, v.clone())).collect();

    let task1 = task1(&valves);
    let task2 = task2(&valves);

    (task1, task2)
}

fn main() {
    let input = aoc_input::get_input(
        2022,
        16,
        &std::env::var("SESSION").expect("SESSION environment variable not set"),
    )
    .unwrap();

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2022d16 {
    use super::*;

    #[test]
    fn examples() {
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

        let (example1, example2) = solve_task(input);

        assert_eq!(example1, 1651);
        assert_eq!(example2, 1707);
    }
}
