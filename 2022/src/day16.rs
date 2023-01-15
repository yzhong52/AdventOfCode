use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::time::Instant;
use std::{collections::HashMap, fs};
type Valve = usize;

#[derive(Debug, Clone)]
struct ValveProp {
    rate: i32,
    leading_valves: Vec<Valve>,
}

fn parse(content: String) -> HashMap<Valve, ValveProp> {
    lazy_static! {
        // Examples
        // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        // Valve HH has flow rate=22; tunnel leads to valve GG
        static ref RE: Regex =
            Regex::new(r"^Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? (.*)$")
                .unwrap();
    }

    let mut result: HashMap<Valve, ValveProp> = HashMap::new();

    let mut name_to_index: HashMap<&str, Valve> = [("AA", 0)].into_iter().collect();

    for line in content.trim().split("\n") {
        let capture = RE
            .captures(line.trim())
            .expect(&format!("Unable to unwrap string '{}'.", line.trim()));

        let source_str: &str = capture.get(1).unwrap().as_str();
        if !name_to_index.contains_key(source_str) {
            name_to_index.insert(source_str, name_to_index.len());
        }

        let source: Valve = *name_to_index.get(source_str).unwrap();
        let rate: i32 = capture.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let leading_valves: Vec<Valve> = capture
            .get(3)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|leading_valve_str| {
                if !name_to_index.contains_key(leading_valve_str) {
                    name_to_index.insert(leading_valve_str, name_to_index.len());
                }
                *name_to_index.get(leading_valve_str).unwrap()
            })
            .collect();

        let valve_prop = ValveProp {
            rate,
            leading_valves,
        };

        result.insert(source, valve_prop);
    }

    result
}

fn solve(
    all_valves: &HashMap<Valve, ValveProp>,
    valve_distances: &Vec<Vec<usize>>,
    cache: &mut HashMap<(Valve, Vec<Valve>, i32), i32>,
    current_valve: Valve,
    pending_valves: Vec<Valve>,
    remaining_steps: i32,
) -> i32 {
    let mut result = 0;
    let key = (current_valve, pending_valves.clone(), remaining_steps);
    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }

    for i in 0..pending_valves.len() {
        let next_pending_valve = pending_valves[i];

        // Number of steps required to move from `current_valve` to `next_valve`
        let steps = valve_distances[current_valve][next_pending_valve];

        // 1 more step to open the valve
        let next_remaining_steps = remaining_steps - steps as i32 - 1;

        if next_remaining_steps > 0 {
            let next_pending_valves = pending_valves[0..i]
                .to_vec()
                .into_iter()
                .chain(pending_valves[i + 1..].to_vec())
                .collect();

            let next_result = solve(
                all_valves,
                valve_distances,
                cache,
                next_pending_valve,
                next_pending_valves,
                next_remaining_steps,
            );

            // With the total pressure released by opening the `next_pending_valve`
            let current_result = next_result
                + all_valves.get(&next_pending_valve).unwrap().rate * next_remaining_steps;

            result = result.max(current_result);
        }
    }
    cache.insert(key, result);
    result
}

fn floyd_warshall(all_valves: &HashMap<Valve, ValveProp>) -> Vec<Vec<usize>> {
    let num_of_valves = all_valves.len();

    // divide by 2 in case of integer overflow later during addition
    let mut valve_distances = vec![vec![usize::MAX / 2; num_of_valves]; num_of_valves];

    for (valve, valve_prop) in all_valves {
        for leading_valve in &valve_prop.leading_valves {
            valve_distances[*valve][*leading_valve] = 1;
            valve_distances[*leading_valve][*valve] = 1;
        }
    }

    for i in 0..num_of_valves {
        valve_distances[i][i] = 0;
    }

    for k in 0..num_of_valves {
        for i in 0..num_of_valves {
            for j in 0..num_of_valves {
                valve_distances[i][j] =
                    valve_distances[i][j].min(valve_distances[i][k] + valve_distances[k][j]);
            }
        }
    }

    valve_distances
}

fn partitions(values: Vec<Valve>) -> Vec<(Vec<Valve>, Vec<Valve>)> {
    // We can assume the first value is always assigned to the left (due to symmetry)
    let mut result: Vec<Vec<Valve>> = vec![vec![values[0].clone()]];

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
    println!("day16: this can take a while...");
    let all_valves = parse(content);

    let valves_with_positive_flow_rate: Vec<Valve> = all_valves
        .clone()
        .into_iter()
        .filter(|(_value_name, valve_property)| valve_property.rate > 0)
        .map(|(value_name, _valve_property)| value_name)
        .collect_vec();

    let valve_distances = floyd_warshall(&all_valves);

    let mut cache: HashMap<(Valve, Vec<Valve>, i32), i32> = HashMap::new();

    let start = Instant::now();
    let part1 = solve(
        &all_valves,
        &valve_distances,
        &mut cache,
        0,
        valves_with_positive_flow_rate.clone(),
        30,
    );
    let duration = start.elapsed();
    println!("part1 time elapsed: {:?}", duration);

    let start = Instant::now();
    let part2 = partitions(valves_with_positive_flow_rate)
        .into_iter()
        .map(|(left, right)| {
            let left = solve(&all_valves, &valve_distances, &mut cache, 0, left, 26);
            let right = solve(&all_valves, &valve_distances, &mut cache, 0, right, 26);
            left + right
        })
        .max()
        .unwrap();
    let duration = start.elapsed();
    println!("part2 time elapsed: {:?}", duration);

    println!("day16 part1: {}", part1);
    println!("day16 part2: {}", part2);
    (part1.to_string(), part2.to_string())
}

pub fn day16() -> (String, String) {
    let content = fs::read_to_string("input/day16").unwrap();
    run(content)
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
        assert_eq!(part2, "2824");
    }
}
