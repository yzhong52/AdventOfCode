use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;

struct Blueprint {
    requirement: [[i32; 4]; 4],
}

#[derive(Eq, Hash, PartialEq, Debug)]
struct CacheKey {
    robots: Vec<i32>,
    resource: Vec<i32>,
    time: i32,
}

fn explore(
    blueprint: &Blueprint,
    cache: &mut HashMap<CacheKey, i32>,
    robots: Vec<i32>,
    resource: Vec<i32>,
    time: i32,
) -> i32 {
    let key = CacheKey {
        robots: robots.clone(),
        resource: resource.clone(),
        time,
    };
    if cache.contains_key(&key) {
        println!("skip! {:?}", key);
        return *cache.get(&key).unwrap();
    } else {
        println!("{:?}", key);
    }

    let resource = resource
        .iter()
        .zip(robots.iter())
        .map(|(resource, robot_count)| resource + robot_count)
        .collect_vec();
    if time == 6 {
        // times up!
        *resource.last().unwrap()
    } else {
        let mut best_option = 0;

        for i in 0..4 {
            let requirement = blueprint.requirement[i];
            let new_resource = resource
                .iter()
                .zip(requirement.iter())
                .map(|(available_resource, require_resource)| available_resource - require_resource)
                .collect_vec();

            let not_enough = new_resource.iter().any(|resource| *resource < 0);
            if not_enough {
                // not enough resource to build this robot
                continue;
            }

            let mut new_robots = robots.clone();
            new_robots[i] += 1;
            best_option = best_option.max(explore(
                &blueprint,
                cache,
                new_robots,
                new_resource,
                time + 1,
            ));
        }

        // not building new robot for now
        best_option = best_option.max(explore(&blueprint, cache, robots, resource, time + 1));
        cache.insert(key, best_option);
        best_option
    }
}

fn run(content: String) -> (String, String) {
    let requirement = [[4, 0, 0, 0], [2, 0, 0, 0], [3, 14, 0, 0], [2, 0, 7, 0]];
    let blueprint = Blueprint { requirement };
    let mut cache: HashMap<CacheKey, i32> = HashMap::new();
    let part1 = explore(
        &blueprint,
        &mut cache,
        vec![1, 0, 0, 0],
        vec![0, 0, 0, 0],
        1,
    );
    let part2 = 0;
    println!("day19 part1: {}", part1);
    println!("day19 part2: {}", part2);
    (part1.to_string(), part2.to_string())
}

pub fn day19() -> (String, String) {
    let content = fs::read_to_string("input/day19").unwrap();
    run(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day19_example_test() {
        let input = "
            Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
            Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."
            .to_string();

        let (part1, part2) = run(input);
        assert_eq!(part1, "64");
        assert_eq!(part2, "58");
    }

    #[test]
    fn day19_test() {
        let (part1, part2) = day19();
        assert_eq!(part1, "4282");
        assert_eq!(part2, "2452");
    }
}
