use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::{
    collections::{BTreeSet, HashMap},
    fs,
};

pub fn day16() -> (String, String) {
    let content = fs::read_to_string("input/day16").unwrap();
    run(content)
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct VisitedState {
    // The current valve position
    current_valve: String,

    // FIXME: This should be 'HashSet'. But itself cannot be hashed.
    // - https://github.com/rust-lang/rust/pull/48366
    opened_valves: BTreeSet<String>,
}

#[derive(Debug, Clone)]
struct Pressure {
    releasing: i32,
    total: i32,
}

type ValveName = String;

#[derive(Debug, Clone)]
struct ValveProp {
    rate: i32,
    leading_valves: Vec<ValveName>,
}

fn solve(
    initial_state: VisitedState,
    all_valves: &HashMap<ValveName, ValveProp>,
    openable_valves: HashSet<ValveName>,
    total_steps: i32,
) -> i32 {
    let mut visited_states: HashMap<VisitedState, i32> = HashMap::new();
    visited_states.insert(initial_state.clone(), 0);

    let mut visiting_states: Vec<(VisitedState, Pressure)> = vec![(
        initial_state,
        Pressure {
            releasing: 0,
            total: 0,
        },
    )];

    // To keep track of the states, we only need to know
    //
    // 1) where we are currently located, `current_valve`
    // 2) what the valves that have been opened, `opened_valves`
    //
    // Every step, we can take two actions,
    //
    // 1) open the current valve, which will add one more entry to `opened_valves`
    // 2) move to another vale, which will change the value of `opened_valve`.
    //
    // Now, when we have multiple states to expand our search, which one should we start with
    // so that we can find the optimal solution fastest?
    for _steps in 1..=total_steps {
        let mut next_visiting_states = vec![];

        for (current_visiting_state, pressure) in &visiting_states {
            let current_valve = &current_visiting_state.current_valve;

            // Try to open the current valve
            // Only consider opening the valve if it is not yet opened
            if !current_visiting_state.opened_valves.contains(current_valve) {
                let current_valve_property = all_valves.get(current_valve).unwrap();

                // Only consider opening the valve if the flow rate is greater than 0
                if openable_valves.contains(current_valve) {
                    let mut new_state = current_visiting_state.clone();
                    new_state.opened_valves.insert(current_valve.clone());
                    let new_pressure = Pressure {
                        releasing: pressure.releasing + current_valve_property.rate,
                        total: pressure.total + pressure.releasing,
                    };
                    if visited_states
                        .get(&new_state)
                        .map(|total_pressure| total_pressure < &new_pressure.total)
                        .unwrap_or(true)
                    {
                        visited_states.insert(new_state.clone(), new_pressure.total);
                        next_visiting_states.push((new_state, new_pressure));
                    }
                }
            }

            // Try to move on to the next valve
            let new_pressure = Pressure {
                releasing: pressure.releasing,
                total: pressure.total + pressure.releasing,
            };

            for next_valve in &all_valves.get(current_valve).unwrap().leading_valves {
                let mut new_state = current_visiting_state.clone();
                new_state.current_valve = next_valve.clone();

                if visited_states
                    .get(&new_state)
                    .map(|total_pressure| total_pressure < &new_pressure.total)
                    .unwrap_or(true)
                {
                    visited_states.insert(new_state.clone(), new_pressure.total);
                    next_visiting_states.push((new_state, new_pressure.clone()));
                }
            }

            // Do nothing
            visited_states.insert(current_visiting_state.clone(), new_pressure.total);
            next_visiting_states.push((current_visiting_state.clone(), new_pressure));
        }

        // Can we do some more pruning?
        let next_visiting_states_len = next_visiting_states.len();

        let steps_remained = 30 - _steps;

        let overall_minimum_release_pressure = next_visiting_states
            .iter()
            .map(|(_state, pressure)| pressure.releasing * steps_remained + pressure.total)
            .max()
            .unwrap();

        visiting_states = next_visiting_states
            .into_iter()
            .filter(|(state, pressure)| {
                // From all the remaining valves, find the one with the maximum flow that is not yet
                // opened.
                let max_unopened_flow = all_valves
                    .iter()
                    .filter(|(valve_name, _)| !state.opened_valves.contains(*valve_name))
                    .map(|(_, valve_property)| valve_property.rate)
                    .max()
                    .unwrap_or(0);

                let maximum_release_pressure = (max_unopened_flow * ((total_steps - _steps) / 2)
                    + pressure.releasing)
                    * steps_remained
                    + pressure.total;

                maximum_release_pressure >= overall_minimum_release_pressure
            })
            .collect_vec();

        // println!(
        //     "steps: {}, potential states count: {}, states pruned: {}",
        //     _steps,
        //     next_visiting_states_len,
        //     next_visiting_states_len as i32 - visiting_states.len() as i32,
        // );
    }

    visiting_states
        .iter()
        .map(|(_, pressure)| pressure.total)
        .max()
        .unwrap()
}

fn parse(content: String) -> HashMap<ValveName, ValveProp> {
    lazy_static! {
        // Examples
        // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        // Valve HH has flow rate=22; tunnel leads to valve GG
        static ref RE: Regex =
            Regex::new(r"^Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? (.*)$")
                .unwrap();
    }
    let parsed: HashMap<ValveName, ValveProp> = content
        .trim()
        .split("\n")
        .map(|line| -> (String, ValveProp) {
            let capture = RE
                .captures(line.trim())
                .expect(&format!("Unable to unwrap string '{}'.", line.trim()));

            let source: ValveName = capture.get(1).unwrap().as_str().to_owned();
            let rate: i32 = capture.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let leading_valves: Vec<ValveName> = capture
                .get(3)
                .unwrap()
                .as_str()
                .to_owned()
                .split(", ")
                .map(|value| value.to_string())
                .collect();

            let value_prop = ValveProp {
                rate,
                leading_valves,
            };
            (source, value_prop)
        })
        .collect();
    parsed
}

fn partitions(values: Vec<String>) -> Vec<(Vec<String>, Vec<String>)> {
    // We can assume the first value is always assigned to the left (due to symetry)
    let mut result: Vec<Vec<String>> = vec![vec![values[0].clone()]];

    for value in &values[1..] {
        let mut new_result = result.clone();
        for r in result {
            let mut cloned = r.clone();
            cloned.push(value.clone());
            new_result.push(cloned);
        }
        result = new_result;
    }

    result
        .into_iter()
        .map(|left| {
            let values_cloned = values.clone();
            let right = values_cloned
                .into_iter()
                .filter(|x| !left.contains(x))
                .collect_vec();
            (left, right)
        })
        .collect_vec()
}

fn run(content: String) -> (String, String) {
    let all_valves = parse(content);
    let initial_state = VisitedState {
        current_valve: "AA".to_string(),
        opened_valves: BTreeSet::new(),
    };

    let all_valves_cloned = all_valves.clone();
    let valves_with_positive_flow_rate: Vec<String> = all_valves_cloned
        .into_iter()
        .filter(|(_value_name, valve_property)| valve_property.rate > 0)
        .map(|(value_name, _valve_property)| value_name)
        .collect_vec();

    let openable_valves: HashSet<String> =
        HashSet::from_iter(valves_with_positive_flow_rate.clone());

    let part1 = solve(initial_state.clone(), &all_valves, openable_valves, 30);

    // Inspired by discussion https://www.reddit.com/r/adventofcode/comments/zn6k1l/2022_day_16_solutions/
    // We don't have to simulate both agent at once, we can partition the sets into two.
    let partitions = partitions(valves_with_positive_flow_rate);
    let partitions_size = partitions.len();
    let part2 = partitions
        .iter().enumerate()
        .map(|(size, (left, right))| {
            println!("{}/{}", size, partitions_size);
            let left = solve(
                initial_state.clone(),
                &all_valves,
                HashSet::from_iter(left.clone()),
                26,
            );
            let right = solve(
                initial_state.clone(),
                &all_valves,
                HashSet::from_iter(right.clone()),
                26,
            );

            left + right
        })
        .max()
        .unwrap();

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day16_example_test() {
        let input = "
            Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
            Valve BB has flow rate=13; tunnels lead to valves CC, AA
            Valve CC has flow rate=2; tunnels lead to valves DD, BB
            Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
            Valve EE has flow rate=3; tunnels lead to valves FF, DD
            Valve FF has flow rate=0; tunnels lead to valves EE, GG
            Valve GG has flow rate=0; tunnels lead to valves FF, HH
            Valve HH has flow rate=22; tunnel leads to valve GG
            Valve II has flow rate=0; tunnels lead to valves AA, JJ
            Valve JJ has flow rate=21; tunnel leads to valve II
            "
        .to_string();

        let (part1, part2) = run(input);
        assert_eq!(part1, "1651");
        assert_eq!(part2, "1707");
    }

    #[test]
    fn day16_test() {
        let (part1, part2) = day16();
        assert_eq!(part1, "2181");
        assert_eq!(part2, "xx");
    }
}
