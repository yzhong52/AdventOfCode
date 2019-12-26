use super::super::helpers::parser::*;
use super::super::helpers::utils::*;
use super::day9::*;
use std::collections::HashMap;
use crate::helpers::models::Point;

const WHITE_COLOR: bool = true;
const BLACK_COLOR: bool = false;

const TURN_LEFT: i128 = 0;
const TURN_RIGHT: i128 = 1;

pub fn part2(input: Input<Vec<i128>>) -> Answer<String> {
    let map: HashMap<Point, bool> = painting(&input.data, WHITE_COLOR);

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

fn painting(input: &Vec<i128>, initial_color: bool) -> HashMap<Point, bool> {
    let mut map: HashMap<Point, bool> = HashMap::new();
    map.insert(Point::origin(), initial_color);

    let mut robot = SuperIntCodeComputer::new(input.clone());

    let mut pos = Point::origin();
    let mut dir = Point { x: 0, y: 1 };
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
                    TURN_LEFT => dir = Point { x: -dir.y, y: dir.x }, // Turn left 90 degrees
                    TURN_RIGHT => dir = Point { x: dir.y, y: -dir.x }, // Turn right 90 degrees
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
