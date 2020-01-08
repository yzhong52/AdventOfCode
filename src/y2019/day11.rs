use super::super::helpers::parser::*;
use super::super::helpers::utils::*;
use std::collections::HashMap;
use crate::helpers::models::SmallPoint;
use crate::int_code_computers::super_int_code_computer::{SuperIntCodeComputer, SuperIntCodeResult};

const WHITE_COLOR: bool = true;
const BLACK_COLOR: bool = false;

const TURN_LEFT: i128 = 0;
const TURN_RIGHT: i128 = 1;

pub fn part2(input: Input<Vec<i128>>) -> Answer<String> {
    let map: HashMap<SmallPoint, bool> = painting(&input.data, WHITE_COLOR);

    let rect = get_rectangle(&map);

    let mut hull = vec![vec![' '; (rect.upper.x - rect.lower.x + 1) as usize]; (rect.upper.y - rect.lower.y + 1) as usize];

    for (p, color) in &map {
        if *color {
            hull[(rect.upper.y - p.y) as usize][(p.x - rect.lower.x) as usize] = '*'
        }
    }

    let mut result = String::new();
    result.push('\n');
    for row in hull {
        for c in row {
            result.push(' ');
            result.push(c);
            result.push(' ');
        }
        result.push('\n');
    }

    Answer { question: input.question, result }
}

fn painting(input: &Vec<i128>, initial_color: bool) -> HashMap<SmallPoint, bool> {
    let mut map: HashMap<SmallPoint, bool> = HashMap::new();
    map.insert(SmallPoint::origin(), initial_color);

    let mut robot = SuperIntCodeComputer::new(input.clone());

    let mut pos = SmallPoint::origin();
    let mut dir = SmallPoint { x: 0, y: 1 };
    loop {
        let color = map.get(&pos).unwrap_or(&false);
        robot.input_queue.push_back(*color as i128);
        match robot.run() {
            SuperIntCodeResult::Output(val) => {
                map.insert(pos.clone(), val != 0);
            }
            SuperIntCodeResult::Halted => break,
        }

        match robot.run() {
            SuperIntCodeResult::Output(val) => {
                match val {
                    TURN_LEFT => dir = dir.turn_left(),
                    TURN_RIGHT => dir = dir.turn_right(),
                    _ => unimplemented!()
                }
            }
            SuperIntCodeResult::Halted => break,
        }
        pos.x = dir.x + pos.x;
        pos.y = dir.y + pos.y;
    }

    map
}

pub fn part1(input: Input<Vec<i128>>) -> Answer<usize> {
    let map = painting(&input.data, BLACK_COLOR);
    Answer { question: input.question, result: map.len() }
}
