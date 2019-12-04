use super::super::helpers::parser::*;
use crate::helpers::models::Point;
use std::collections::{HashSet, HashMap};
use std::borrow::Borrow;

fn get_offset(ch: char) -> Point {
    match ch {
        'U' => Point { x: 0, y: 1 },
        'R' => Point { x: 1, y: 0 },
        'L' => Point { x: -1, y: 0 },
        'D' => Point { x: 0, y: -1 },
        _ => unimplemented!()
    }
}

pub fn part1(input: &Input<Vec<String>>) -> Answer<i32> {
    let mut set: HashSet<Point> = HashSet::new();

    let line1 = &input.data[0];
    let line2 = &input.data[1];

    // TODO: Yuchen - dedup
    let mut point  = Point::origin();
    for action in line1.split(',') {
        let steps = action[1..].parse::<i32>().unwrap();
        let offset = get_offset(action.chars().next().unwrap());

        for _ in 0..steps {
            point.x = point.x + offset.x;
            point.y = point.y + offset.y;

            set.insert(point.clone());
        }
    }

    let mut result = std::i32::MAX;

    // TODO: Yuchen - dedup
    let mut point2  = Point::origin();
    for action in line2.split(',') {
        let steps = action[1..].parse::<i32>().unwrap();
        let offset = get_offset(action.chars().next().unwrap());

        for _ in 0..steps {
            point2.x = point2.x + offset.x;
            point2.y = point2.y + offset.y;

            if set.contains(&point2) {
                // println!("Oops, found one {:?}", point2);
                result = i32::min(result, i32::abs(point2.x) + i32::abs(point2.y));
            }
        }
    }

    Answer { question: input.question, result }
}

pub fn part2(input: &Input<Vec<String>>) -> Answer<i32> {
    let mut map: HashMap<Point, i32> = HashMap::new();

    let line1 = &input.data[0];
    let line2 = &input.data[1];

    // TODO: Yuchen - dedup
    let mut point  = Point::origin();
    let mut count: i32 = 0;
    for action in line1.split(',') {
        let steps = action[1..].parse::<i32>().unwrap();
        let offset = get_offset(action.chars().next().unwrap());

        for _ in 0..steps {
            point.x = point.x + offset.x;
            point.y = point.y + offset.y;
            count = count + 1;
            if !map.contains_key(&point) {
                map.insert(point.clone(), count);
            }
        }
    }

    let mut result = std::i32::MAX;

    // TODO: Yuchen - dedup
    let mut point2 = Point::origin();
    let mut count2: i32 = 0;
    for action in line2.split(',') {
        let steps = action[1..].parse::<i32>().unwrap();
        let offset = get_offset(action.chars().next().unwrap());

        for _ in 0..steps {
            point2.x = point2.x + offset.x;
            point2.y = point2.y + offset.y;
            count2 = count2 + 1;
            if map.contains_key(&point2) {
                // println!("Found an intersection {:?}", point2);
                result = i32::min(result, map.get(&point2).unwrap() + count2);
            }
        }
    }

    Answer { question: input.question, result }
}
