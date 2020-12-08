use super::super::helpers::parser::*;
use crate::helpers::models::SmallPoint;
use std::collections::{HashSet, HashMap};
use crate::helpers::puzzle::Puzzle;
use std::iter::FromIterator;

fn get_offset(ch: char) -> SmallPoint {
    match ch {
        'U' => SmallPoint { x: 0, y: 1 },
        'R' => SmallPoint { x: 1, y: 0 },
        'L' => SmallPoint { x: -1, y: 0 },
        'D' => SmallPoint { x: 0, y: -1 },
        _ => unimplemented!()
    }
}

fn get_points(line: &String) -> Vec<SmallPoint> {
    let mut points = vec![];
    let mut point = SmallPoint::origin();
    for action in line.split(',') {
        let steps = action[1..].parse::<i32>().unwrap();
        let offset = get_offset(action.chars().next().unwrap());

        for _ in 0..steps {
            point.x = point.x + offset.x;
            point.y = point.y + offset.y;
            points.push(point.clone())
        }
    }
    points
}

pub struct Day3 {}

impl Puzzle<Vec<String>, usize> for Day3 {

    fn day(&self) -> i8 {
        3
    }

    fn parser(&self) -> fn(String) -> Vec<String> {
        read_strings_by_line
    }

    fn part1(&self, input: &Vec<String>) -> usize {
        let line1_points: Vec<SmallPoint> = get_points(&input[0]);
        let line2_points = get_points(&input[1]);

        let set: HashSet<SmallPoint> = HashSet::from_iter(line1_points);

        let mut result = std::usize::MAX;

        for point in line2_points {
            if set.contains(&point) {
                result = usize::min(result, (i32::abs(point.x) + i32::abs(point.y)) as usize);
            }
        }
        result
    }

    fn part2(&self, input: &Vec<String>) -> usize {
        let mut map: HashMap<SmallPoint, usize> = HashMap::new();

        let line1_points: Vec<SmallPoint> = get_points(&input[0]);
        let line2_points = get_points(&input[1]);

        for (count, point) in line1_points.iter().enumerate() {
            if !map.contains_key(&point) {
                map.insert(point.clone(), count + 1);
            }
        }

        let mut result = std::usize::MAX;

        for (count, point) in line2_points.iter().enumerate() {
            if map.contains_key(&point) {
                result = usize::min(result, map.get(&point).unwrap() + count + 1);
            }
        }

        result
    }
}
