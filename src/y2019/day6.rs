use super::super::helpers::parser::*;
use std::collections::HashMap;
use crate::helpers::puzzle::Puzzle;

pub fn index_graph(data: Vec<String>) -> HashMap<String, String> {
    let mut orbits_map: HashMap<String, String> = HashMap::new();
    for line in data {
        let orbits: Vec<&str> = line.split(")").collect();
        orbits_map.insert(orbits[1].to_string(), orbits[0].to_string());
    }
    return orbits_map;
}

pub fn part1(input: Input<Vec<String>>) -> Answer<u32> {
    let orbits_map: HashMap<String, String> = index_graph(input.data);

    // Count the number fo parents that the given `orbit` has.
    let mut counts: HashMap<&String, u32> = HashMap::new();

    for orbit in orbits_map.keys() {
        if counts.contains_key(orbit) {
            // already computed, skip
            continue;
        }

        let mut stack = vec![orbit];

        let mut runner = orbit;
        while orbits_map.contains_key(runner) && !counts.contains_key(runner) {
            runner = orbits_map.get(runner).unwrap();
            stack.push(runner);
        }

        let mut last_count = **counts.get(runner).get_or_insert(&0);

        while stack.len() > 0 {
            counts.insert(stack.pop().unwrap(), last_count);
            last_count += 1;
        }
    }

    let result: u32 = counts.values().sum();

    return Answer { question: input.question, result };
}


fn path_to_root(orbits_map: &HashMap<String, String>, orbit: String) -> Vec<String> {
    let mut stack = vec![orbit.clone()];

    let mut runner = orbit;
    while orbits_map.contains_key(&runner) {
        runner = (*orbits_map.get(&runner).unwrap()).to_string();
        stack.push(runner.clone());
    }
    return stack;
}

pub fn part2(input: Input<Vec<String>>) -> Answer<usize> {
    let orbits_map: HashMap<String, String> = index_graph(input.data);

    let mut your_path = path_to_root(&orbits_map, "YOU".to_string());
    let mut santa_path = path_to_root(&orbits_map, "SAN".to_string());

    your_path.reverse();
    santa_path.reverse();


    let mut i = 0;
    while i < your_path.len() && i < santa_path.len() && your_path[i] == santa_path[i] {
        i += 1
    }

    let result = your_path.len() - i + santa_path.len() - i - 2;
    return Answer { question: input.question, result };
}

pub struct Day6 {}

impl Puzzle<Vec<i32>, i32> for Day6 {
    fn day(&self) -> i8 {
        unimplemented!()
    }

    fn parser(&self) -> fn(String) -> Vec<i32> {
        unimplemented!()
    }

    fn part1(&self, input: &Vec<i32>) -> i32 {
        unimplemented!()
    }

    fn part2(&self, input: &Vec<i32>) -> i32 {
        unimplemented!()
    }
}
