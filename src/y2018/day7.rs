use super::super::helpers::parser::*;
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

pub fn part2(input: Input<Vec<String>>) -> Answer<i32> {
    const TOTAL_WORKER_COUNT: usize = 5;
    const JOB_CONSTANT_AMOUNT: i32 = 60;

    let mut workers = [0; TOTAL_WORKER_COUNT];
    let mut assigned_job = vec![String::new(); TOTAL_WORKER_COUNT];

    let dependent_map: HashMap<String, HashSet<String>> = index_dependency(&input.data);
    let mut prerequisite_map: HashMap<String, HashSet<String>> = index_prerequisite(&input.data);

    // Find jobs that without prerequisite
    let mut ready_tasks: BinaryHeap<Reverse<String>> = BinaryHeap::new();
    for (key, prerequisite) in &prerequisite_map {
        if prerequisite.len() == 0 {
            ready_tasks.push(Reverse(key.to_string()));
        }
    }

    // The earliest that the next worker will become available
    let mut soonest_finished_time: i32 = 0;

    while !ready_tasks.is_empty() || assigned_job.iter().filter(|x| !x.is_empty()).count() > 0 {
        if !ready_tasks.is_empty() && assigned_job.iter().filter(|x| !x.is_empty()).count() < TOTAL_WORKER_COUNT {
            let mut available_worker_count = workers.iter().filter(|x| **x <= soonest_finished_time).count().clone();

            let mut next_up_tasks: Vec<String> = Vec::new();
            while !ready_tasks.is_empty() && available_worker_count > 0 {
                available_worker_count -= 1;
                next_up_tasks.push(ready_tasks.pop().unwrap().0);
            }
            // println!("next_up_tasks: {:?}", next_up_tasks);

            let mut available_worker_index: usize = 0;
            for current in next_up_tasks {
                let ch = current.chars().next().unwrap();
                let finish_time: i32 = soonest_finished_time + JOB_CONSTANT_AMOUNT + (ch as i32 - 'A' as i32) + 1;

                while workers[available_worker_index] > soonest_finished_time {
                    available_worker_index += 1
                }

                workers[available_worker_index] = workers[available_worker_index] + finish_time;
                assigned_job[available_worker_index] = current;
                available_worker_index += 1;

                // println!("Workers finish time: {:?}", workers);
                // println!("Workers assigned task: {:?}", assigned_job);
            }
        } else {
            // Finish running some task(s)
            soonest_finished_time = workers.iter().filter(|x| **x != 0).min().unwrap().clone();

            for (w, a) in workers.iter().zip(assigned_job.iter()) {
                if *w == soonest_finished_time {
                    // println!("Job '{}' finished at time '{}'.", a, w);

                    let current = a;
                    // Remove prerequisite for dependents
                    let dependent = dependent_map.get(current).unwrap();
                    for child in dependent {
                        let prerequisite = prerequisite_map.get_mut(child).unwrap();
                        prerequisite.remove(current);

                        if prerequisite.is_empty() {
                            ready_tasks.push(Reverse(child.to_string()));
                        }
                    }
                }
            }

            for i in 0..TOTAL_WORKER_COUNT {
                if workers[i] == soonest_finished_time {
                    workers[i] = 0;
                    assigned_job[i] = String::new();
                }
            }
        }
    }

    Answer { question: input.question, result: soonest_finished_time }
}
