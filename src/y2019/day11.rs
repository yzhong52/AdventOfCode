use super::super::helpers::parser::*;
use super::super::helpers::utils::*;
use std::collections::HashMap;
use crate::helpers::models::SmallPoint;
use crate::int_code_computers::super_int_code_computer::{SuperIntCodeComputer, SuperIntCodeResult};
use crate::helpers::puzzle::Puzzle;

const WHITE_COLOR: bool = true;
const BLACK_COLOR: bool = false;

const TURN_LEFT: i128 = 0;
const TURN_RIGHT: i128 = 1;

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

pub struct Day11 {}

impl Puzzle<Vec<i128>, String> for Day11 {
    fn day(&self) -> i8 {
        11
    }

    fn parser(&self) -> fn(String) -> Vec<i128> {
        parse_numbers_by_comma
    }

    fn part1(&self, input: &Vec<i128>) -> String {
        let result = painting(&input, BLACK_COLOR).len();
        // Just so that we don't have to create different return types for part 1 and part 2
        format!("{}", result)
    }

    fn part2(&self, input: &Vec<i128>) -> String {
        let map: HashMap<SmallPoint, bool> = painting(&input, WHITE_COLOR);

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

        result
    }
}

