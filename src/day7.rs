use super::helpers::parser::*;
use std::collections::{HashMap, HashSet};
use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub fn part1(input: Input<Vec<String>>) -> Answer<String> {
    let mut dependent_map: HashMap<String, HashSet<String>> = HashMap::new();
    let mut prerequisite_map: HashMap<String, HashSet<String>> = HashMap::new();

    for line in input.data {
        // e.g. "Step G must be finished before step W can begin."
        let from = extract_between_plus(line.as_str(), "Step ", " must be finished before step ");
        let to = extract_between_plus(line.as_str(), " must be finished before step ", " can begin.");

        prerequisite_map.entry(from.clone()).or_insert(HashSet::<String>::new());
        let prerequisite = prerequisite_map.entry(to.clone()).or_insert(HashSet::<String>::new());
        prerequisite.insert(from.clone());

        dependent_map.entry(to.clone()).or_insert(HashSet::<String>::new());
        let dependent = dependent_map.entry(from).or_insert(HashSet::<String>::new());
        dependent.insert(to);
    }

    let mut heap: BinaryHeap<Reverse<String>> = BinaryHeap::new();
    for (key, prerequisite) in &prerequisite_map {
        if prerequisite.len() == 0 {
            heap.push(Reverse(key.to_string()));
        }
    }

    let mut result: String = "".to_string();
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
