use super::super::helpers::parser::*;
use super::day9::*;
use std::collections::{HashMap, VecDeque};
use crate::helpers::models::Point;

fn combinations(current: Vec<i128>) -> Vec<Vec<i128>> {
    if current.len() == 1 {
        return vec![current];
    } else {
        let mut result: Vec<Vec<i128>> = vec![];

        for i in 0..current.len() {
            let mut remain = current.clone();
            let removed_phase = remain[i];
            remain.remove(i);
            let next = combinations(remain.clone());
            for mut n in next {
                n.push(removed_phase);
                result.push(n)
            }
        }
        result
    }
}

pub fn part2(input: Input<Vec<i128>>) -> Answer<i128> {
    Answer { question: input.question, result: 0 }
}

pub fn part1(input: Input<Vec<i128>>) -> Answer<usize> {
    let mut map: HashMap<Point, bool> = HashMap::new();

    let mut robot = SuperIntCodeComputer {
        numbers: input.data.clone(),
        index: 0,
        input_queue: VecDeque::new(),
        relative_base: 0,
        external_numbers: HashMap::new(),
    };

    let mut pos = Point::origin();
    let mut dir = Point { x: 0, y: 1 };
    loop {
        let color = map.get(&pos).unwrap_or(&false);
        robot.input_queue.push_back(*color as i128);

        println!("Pos: {:?}, Color: {}", pos, *color);

        match robot.run() {
            SuperIntCodeResult::Output(val) => {
                println!("Value {}", val);

                map.insert(pos.clone(), val != 0);
            }
            SuperIntCodeResult::Halted => break,
        }

        match robot.run() {
            SuperIntCodeResult::Output(val) => {
                println!("Value {}", val);

                match val {
                    0 => dir = Point { x: -dir.y, y: dir.x }, // Turn left 90 degrees
                    1 => dir = Point { x: dir.y, y: -dir.x }, // Turn right 90 degrees
                    _ => unimplemented!()
                }
            }
            SuperIntCodeResult::Halted => break,
        }
        pos.x = dir.x + pos.x;
        pos.y = dir.y + pos.y;
    }

    Answer { question: input.question, result: map.len() }
}
