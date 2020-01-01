use super::super::helpers::parser::*;
use crate::helpers::models::Point;
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;

fn get_offset(ch: char) -> Point {
    match ch {
        'U' => Point { x: 0, y: 1 },
        'R' => Point { x: 1, y: 0 },
        'L' => Point { x: -1, y: 0 },
        'D' => Point { x: 0, y: -1 },
        _ => unimplemented!()
    }
}

fn get_points(line: &String) -> Vec<Point> {
    let mut points = vec![];
    let mut point = Point::origin();
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

pub fn part1(input: &Input<Vec<String>>) -> Answer<i32> {
    let line1_points: Vec<Point> = get_points(&input.data[0]);
    let line2_points = get_points(&input.data[1]);

    let mut set: HashSet<Point> = HashSet::from_iter(line1_points);

    let mut result = std::i32::MAX;

    for point in line2_points {
        if set.contains(&point) {
            result = i32::min(result, i32::abs(point.x) + i32::abs(point.y));
        }
    }

    Answer { question: input.question, result }
}

pub fn part2(input: &Input<Vec<String>>) -> Answer<usize> {
    let mut map: HashMap<Point, usize> = HashMap::new();

    let line1_points: Vec<Point> = get_points(&input.data[0]);
    let line2_points = get_points(&input.data[1]);

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

    Answer { question: input.question, result }
}
