use super::helpers::parser::*;
use std::collections::{HashMap, HashSet};
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn parse_dependency(line: &String) -> (String, String) {
    // e.g. "Step G must be finished before step W can begin."
    let from = extract_between_plus(line.as_str(), "Step ", " must be finished before step ");
    let to = extract_between_plus(line.as_str(), " must be finished before step ", " can begin.");
    return (from, to);
}

fn index_dependency(input: &Vec<String>) -> HashMap<String, HashSet<String>> {
    let mut dependent_map: HashMap<String, HashSet<String>> = HashMap::new();

    for line in input {
        let (from, to) = parse_dependency(line);

        dependent_map.entry(to.clone()).or_insert(HashSet::<String>::new());

        let dependent = dependent_map.entry(from).or_insert(HashSet::<String>::new());
        dependent.insert(to);
    }
    return dependent_map;
}

fn index_prerequisite(input: &Vec<String>) -> HashMap<String, HashSet<String>> {
    let mut prerequisite_map: HashMap<String, HashSet<String>> = HashMap::new();

    for line in input {
        let (from, to) = parse_dependency(line);

        prerequisite_map.entry(from.clone()).or_insert(HashSet::<String>::new());

        let prerequisite = prerequisite_map.entry(to.clone()).or_insert(HashSet::<String>::new());
        prerequisite.insert(from.clone());
    }

    return prerequisite_map;
}

pub fn part1(input: Input<Vec<String>>) -> Answer<String> {
    let dependent_map: HashMap<String, HashSet<String>> = index_dependency(&input.data);
    let mut prerequisite_map: HashMap<String, HashSet<String>> = index_prerequisite(&input.data);

    let mut heap: BinaryHeap<Reverse<String>> = BinaryHeap::new();
    for (key, prerequisite) in &prerequisite_map {
        if prerequisite.len() == 0 {
            heap.push(Reverse(key.to_string()));
        }
    }

    let mut result: String = String::new();
    while !heap.is_empty() {
        let current: Reverse<String> = heap.pop().unwrap();

        let dependent = dependent_map.get(&current.0).unwrap();
        for child in dependent {
            let prerequisite = prerequisite_map.get_mut(child).unwrap();
            prerequisite.remove(&current.0);

            if prerequisite.is_empty() {
                heap.push(Reverse(child.to_string()));
            }
        }

        result += &current.0;
    }

    Answer { question: input.question, result }
}

pub fn part2(input: Input<Vec<String>>) -> Answer<String> {
    let mut result: String = String::new();
    let mut workers = [0; 5];

    let dependent_map: HashMap<String, HashSet<String>> = index_dependency(&input.data);
    let mut prerequisite_map: HashMap<String, HashSet<String>> = index_prerequisite(&input.data);

    let mut heap: BinaryHeap<Reverse<String>> = BinaryHeap::new();
    for (key, prerequisite) in &prerequisite_map {
        if prerequisite.len() == 0 {
            heap.push(Reverse(key.to_string()));
        }
    }

    let mut result: String = String::new();
    while !heap.is_empty() {
        let minimum_finished_time = workers.iter().min().unwrap();
        let mut minimum_available_worker = workers.iter().filter(|x| *x == minimum_finished_time).count();

        let mut next_up_tasks: Vec<Reverse<String>> = Vec::new();
        while !heap.is_empty() && minimum_available_worker > 0 {
            minimum_available_worker -= 1;
            next_up_tasks.push(heap.pop().unwrap());
        }

        for current in next_up_tasks {
            let dependent = dependent_map.get(&current.0).unwrap();
            for child in dependent {
                let prerequisite = prerequisite_map.get_mut(child).unwrap();
                prerequisite.remove(&current.0);

                if prerequisite.is_empty() {
                    heap.push(Reverse(child.to_string()));
                }
            }
            result += &current.0;
        }
    }

    Answer { question: input.question, result }
}
