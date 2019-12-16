use super::super::helpers::parser::*;
use super::super::helpers::utils::*;
use super::day9::*;
use std::collections::{HashMap, VecDeque};
use crate::helpers::models::BigPoint;
use std::thread::sleep;
use std::time::Duration;
use rand::prelude::*;

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
const START: Annotation = 'S';
const TARGET: Annotation = 'T';
const WALL: Annotation = '#';
const DROID: Annotation = 'D';
const EMPTY: Annotation = '.';

fn lets_go(input: &Vec<i128>, debug: bool) -> HashMap<BigPoint, char> {
    let mut map: HashMap<BigPoint, char> = HashMap::new();
    map.insert(BigPoint::origin(), DROID);

    let mut droid = SuperIntCodeComputer {
        numbers: input.clone(),
        index: 0,
        input_queue: VecDeque::new(),
        relative_base: 0,
        external_numbers: HashMap::new(),
    };

    let command_index = 0;
    let moves = [NORTH, SOUTH, WEST, EAST];

    let directions: HashMap<DIRECTION, BigPoint> = [
        (NORTH, BigPoint { x: 0, y: 1 }),
        (SOUTH, BigPoint { x: 0, y: -1 }),
        (WEST, BigPoint { x: -1, y: 0 }),
        (EAST, BigPoint { x: 1, y: 0 }),
    ].iter().cloned().collect();

    let mut last_move = NORTH;
    let mut droid_pos: BigPoint = BigPoint::origin();
    let mut destination_pos: Option<BigPoint> = None;

    let mut rng = rand::thread_rng();

    loop {
        let move_index = rng.gen_range(0, moves.len());
        last_move = moves[move_index];

        droid.input_queue.push_back(last_move);

        let direction = directions.get(&last_move).unwrap();

        match droid.run() {
            SuperIntCodeResult::Output(RESPONSE_WALL_HIT) => {
                let wall_pos = droid_pos.clone() + direction.clone();
                map.insert(wall_pos, WALL);
            }
            SuperIntCodeResult::Output(RESPONSE_MOVED) => {
                map.insert(droid_pos.clone(), EMPTY);
                droid_pos += direction.clone();
                map.insert(droid_pos.clone(), DROID);
            }
            SuperIntCodeResult::Output(RESPONSE_DESTINATION) => {
                map.insert(droid_pos.clone(), EMPTY);
                droid_pos += direction.clone();
                destination_pos = Some(droid_pos.clone());
                map.insert(droid_pos.clone(), EMPTY);
            }
            SuperIntCodeResult::Halted => break,
            _ => unimplemented!()
        };

        if debug {
            let buffer = get_screen_buffer(&map, ' ');
            print(&buffer);
            println!("Droid pos: {:?}", droid_pos);
            println!("Target pos: {:?}", droid_pos);

            sleep(Duration::from_millis(200));
        }
    }

    map
}

pub fn part1(input: Input<Vec<i128>>) -> Answer<usize> {
    lets_go(&input.data, true);
    Answer { question: input.question, result: 0 }
}

pub fn part2(input: Input<Vec<i128>>) -> Answer<i128> {
    Answer { question: input.question, result: 0 }
}
