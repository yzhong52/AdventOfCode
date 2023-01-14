use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fs,
};

pub fn day16() -> (String, String) {
    let content = fs::read_to_string("input/day16").unwrap();
    run(content)
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct VisitedState {
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

struct ValveProp {
    rate: i32,
    leading_valves: Vec<ValveName>,
}

fn run(content: String) -> (String, String) {
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

    let initial_state = VisitedState {
        current_valve: "AA".to_string(),
        opened_valves: BTreeSet::new(),
    };
    let mut visited_states: HashMap<VisitedState, i32> = HashMap::new();
    visited_states.insert(initial_state.clone(), 0);

    let mut visiting_states = vec![(
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
    for _steps in 1..=30 {
        let mut next_visiting_states = vec![];

        for (current_visiting_state, pressure) in &visiting_states {
            // open the current valve
            if !current_visiting_state
                .opened_valves
                .contains(&current_visiting_state.current_valve)
            {
                let current_valve_property = parsed
                    .get(&current_visiting_state.current_valve)
                    .unwrap();

                // Only consider opening the valve if the flow rate is greater than 0
                if current_valve_property.rate > 0 {
                    let mut new_state = current_visiting_state.clone();
                    new_state
                        .opened_valves
                        .insert(current_visiting_state.current_valve.clone());
                    let new_pressure = Pressure {
                        releasing: pressure.releasing
                            + parsed
                            .get(&current_visiting_state.current_valve)
                            .unwrap()
                            .rate,
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

            let new_pressure = Pressure {
                releasing: pressure.releasing,
                total: pressure.total + pressure.releasing,
            };

            // Move on to the next valve
            for next_valve in &parsed
                .get(&current_visiting_state.current_valve)
                .unwrap()
                .leading_valves
            {
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
                let max_unopened_flow = parsed
                    .iter()
                    .filter(|(valve_name, _)| !state.opened_valves.contains(*valve_name))
                    .map(|(_, valve_property)| valve_property.rate)
                    .max()
                    .unwrap_or(0);

                let maximum_release_pressure = (max_unopened_flow * ((30 - _steps) / 2) + pressure.releasing) * steps_remained + pressure.total;

                maximum_release_pressure >= overall_minimum_release_pressure
            })
            .collect_vec();


        println!(
            "steps: {}, potential states count: {}, states pruned: {}",
            _steps,
            next_visiting_states_len,
            next_visiting_states_len as i32 - visiting_states.len() as i32,
        );
    }

    let part1: i32 = visiting_states
        .iter()
        .map(|(_, pressure)| pressure.total)
        .max()
        .unwrap();

    let part2 = "";

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
        assert_eq!(part2, "xxx");
    }

    #[test]
    fn day16_test() {
        let (part1, part2) = day16();
        assert_eq!(part1, "2181");
        assert_eq!(part2, "xx");
    }
}
