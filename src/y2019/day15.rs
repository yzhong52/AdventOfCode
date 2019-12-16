use super::super::helpers::parser::*;
use super::super::helpers::utils::*;
use super::day9::*;
use std::collections::{HashMap, VecDeque};
use crate::helpers::models::BigPoint;

//0: The repair droid hit a wall. Its position has not changed.
const RESPONSE_WALL_HIT: i128 = 0;
//1: The repair droid has moved one step in the requested direction.
const RESPONSE_MOVED: i128 = 1;
//2: The repair droid has moved one step in the requested direction; its new position is the location of the oxygen system.
const RESPONSE_DESTINATION: i128 = 2;

// Moves
type DIRECTION = i128;
const NORTH: DIRECTION = 1;
const SOUTH: i128 = 2;
const WEST: i128 = 3;
const EAST: i128 = 4;

// Annotations
type Annotation = char;
const START: char = '*';
const TARGET: char = '*';
const WALL: char = '#';
const DROID: char = 'D';

fn lets_go(input: &Vec<i128>) -> HashMap<BigPoint, char> {
    let mut map: HashMap<BigPoint, char> = HashMap::new();
    map.insert(BigPoint::origin(), '*');

    let mut droid = SuperIntCodeComputer {
        numbers: input.clone(),
        index: 0,
        input_queue: VecDeque::new(),
        relative_base: 0,
        external_numbers: HashMap::new(),
    };

    let command_index = 0;
    let commands = [NORTH, SOUTH, WEST, EAST];

    let directions: HashMap<DIRECTION, BigPoint> = [
        (NORTH, BigPoint { x: 0, y: 1 }),
        (SOUTH, BigPoint { x: 0, y: -1 }),
        (WEST, BigPoint { x: -1, y: 1 }),
        (EAST, BigPoint { x: 1, y: 0 }),
    ].iter().cloned().collect();

    loop {
        // droid.input_queue
        let x = match droid.run() {
            SuperIntCodeResult::Output(val) => val,
            SuperIntCodeResult::Halted => break,
        };

        let y = match droid.run() {
            SuperIntCodeResult::Output(val) => val,
            SuperIntCodeResult::Halted => unimplemented!(),
        };

        let tile_id = match droid.run() {
            SuperIntCodeResult::Output(val) => val,
            SuperIntCodeResult::Halted => unimplemented!(),
        };

        map.insert(BigPoint { x, y }, '?');
    }

    map
}

pub fn part1(input: Input<Vec<i128>>) -> Answer<usize> {
    Answer { question: input.question, result: 0 }
}

pub fn part2(input: Input<Vec<i128>>) -> Answer<i128> {
    Answer { question: input.question, result: 0 }
}
