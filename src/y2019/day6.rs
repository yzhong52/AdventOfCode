use std::collections::HashMap;
use crate::helpers::puzzle::Puzzle;
use crate::helpers::parser::read_strings_by_line;

pub struct Day6 {}

impl Day6 {
    pub fn index_graph(data: &Vec<String>) -> HashMap<String, String> {
        let mut orbits_map: HashMap<String, String> = HashMap::new();
        for line in data {
            let orbits: Vec<&str> = line.split(")").collect();
            orbits_map.insert(orbits[1].to_string(), orbits[0].to_string());
        }
        return orbits_map;
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
}

impl Puzzle<Vec<String>, usize> for Day6 {
    fn day(&self) -> i8 {
        6
    }

    fn parser(&self) -> fn(String) -> Vec<String> {
        read_strings_by_line
    }

    fn part1(&self, input: &Vec<String>) -> usize {
        let orbits_map: HashMap<String, String> = Day6::index_graph(input);

        // Count the number fo parents that the given `orbit` has.
        let mut counts: HashMap<&String, usize> = HashMap::new();

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

        counts.values().sum()
    }

    fn part2(&self, input: &Vec<String>) -> usize {
        let orbits_map: HashMap<String, String> = Day6::index_graph(input);

        let mut your_path = Day6::path_to_root(&orbits_map, "YOU".to_string());
        let mut santa_path = Day6::path_to_root(&orbits_map, "SAN".to_string());

        your_path.reverse();
        santa_path.reverse();

        let mut i = 0;
        while i < your_path.len() && i < santa_path.len() && your_path[i] == santa_path[i] {
            i += 1
        }

        (your_path.len() - i + santa_path.len() - i - 2) as usize
    }
}
