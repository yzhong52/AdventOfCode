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
    let mut workers = [0; 5];
    let mut assigned_job = vec![String::new(); 5];

    let dependent_map: HashMap<String, HashSet<String>> = index_dependency(&input.data);
    let mut prerequisite_map: HashMap<String, HashSet<String>> = index_prerequisite(&input.data);

    // Find jobs that without prerequisite
    let mut heap: BinaryHeap<Reverse<String>> = BinaryHeap::new();
    for (key, prerequisite) in &prerequisite_map {
        if prerequisite.len() == 0 {
            heap.push(Reverse(key.to_string()));
        }
    }

    let mut result: String = String::new();
    while !heap.is_empty() {
        let soonest_finished_time = workers.iter().min().unwrap().clone();
        let mut available_worker = workers.iter().filter(|x| **x == soonest_finished_time).count().clone();

        let mut next_up_tasks: Vec<Reverse<String>> = Vec::new();
        while !heap.is_empty() && available_worker > 0 {
            available_worker -= 1;
            next_up_tasks.push(heap.pop().unwrap());
        }
        println!("next_up_tasks: {:?}", next_up_tasks.iter().map(|x| x.0.to_string()).collect::<Vec<String>>());

        let mut available_worker_index: usize = 0;
        for current in next_up_tasks {
            let ch = current.0.chars().next().unwrap();
            let finish_time: i32 = 1 + (ch as i32 - 'A' as i32) + 1;

            while workers[available_worker_index] != soonest_finished_time {
                available_worker_index += 1
            }

            result += &assigned_job[available_worker_index];
            workers[available_worker_index] = workers[available_worker_index] + finish_time;
            assigned_job[available_worker_index] = current.0.to_string();
            available_worker_index += 1;

            println!("Workers finish time: {:?}", workers);
            println!("Workers assigned task: {:?}", assigned_job);

            // Remove prerequisite for dependents
            let dependent = dependent_map.get(&current.0).unwrap();
            for child in dependent {
                let prerequisite = prerequisite_map.get_mut(child).unwrap();
                prerequisite.remove(&current.0);

                if prerequisite.is_empty() {
                    heap.push(Reverse(child.to_string()));
                }
            }
        }
    }

    Answer { question: input.question, result }
}
