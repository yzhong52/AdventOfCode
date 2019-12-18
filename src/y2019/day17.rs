use super::super::helpers::parser::*;
use super::super::helpers::utils::*;
use super::day9::*;
use std::collections::{HashMap, VecDeque, BinaryHeap, HashSet};
use crate::helpers::models::BigPoint;
use std::thread::sleep;
use std::time::Duration;
use std::char;

// Responses
type ResponseCode = i128;

//0: The repair droid hit a wall. Its position has not changed.
const RESPONSE_WALL_HIT: ResponseCode = 0;
//1: The repair droid has moved one step in the requested direction.
const RESPONSE_MOVED: ResponseCode = 1;
//2: The repair droid has moved one step in the requested direction; its new position is the location of the oxygen system.
const RESPONSE_DESTINATION: ResponseCode = 2;

// Moves
type Instruction = i128;

const NORTH: Instruction = 1;
const SOUTH: Instruction = 2;
const WEST: Instruction = 3;
const EAST: Instruction = 4;

// Annotations
type Annotation = char;

const START: Annotation = 'S';
const TARGET: Annotation = 'T';
const WALL: Annotation = '#';
const EMPTY: Annotation = ' ';
const UNKNOWN: Annotation = '.';

struct Action {
    input: Instruction,
    direction: BigPoint,
    symbol: char,
}

struct ExploredMap {
    map: HashMap<BigPoint, char>,
    destination: BigPoint,
}

static ACTIONS: [Action; 4] = [
    Action { input: NORTH, direction: BigPoint { x: 0, y: 1 }, symbol: '^' },
    Action { input: SOUTH, direction: BigPoint { x: 0, y: -1 }, symbol: 'v' },
    Action { input: WEST, direction: BigPoint { x: -1, y: 0 }, symbol: '<' },
    Action { input: EAST, direction: BigPoint { x: 1, y: 0 }, symbol: '>' },
];

fn explore_map(input: &Vec<i128>, debug: bool) -> ExploredMap {
    let mut droid = SuperIntCodeComputer {
        numbers: input.clone(),
        index: 0,
        input_queue: VecDeque::new(),
        relative_base: 0,
        external_numbers: HashMap::new(),
    };

    let mut next_action: &Action = &ACTIONS[0];
    let mut droid_pos: BigPoint = BigPoint::origin();
    let mut destination_pos: Option<BigPoint> = None;

    let mut map: HashMap<BigPoint, char> = HashMap::new();
    map.insert(BigPoint::origin(), next_action.symbol);

    let mut unknown_land = HashSet::new();
    for action in ACTIONS.iter() {
        unknown_land.insert(BigPoint::origin() + action.direction.clone());
    }
    loop {
        let current_move = next_action;

        droid.input_queue.push_back(current_move.input);

        match droid.run() {
            SuperIntCodeResult::Output(value) => {
                print!("{}", value as u8 as char)
            },
            SuperIntCodeResult::Halted => break,
        };

        if debug {

            // sleep(Duration::from_millis(24));
        }
    }

    ExploredMap { map, destination: BigPoint::origin() }
}

pub fn part1(input: Input<Vec<i128>>) -> Answer<usize> {
    let explored = explore_map(&input.data, true);

    Answer { question: input.question, result: 0 }
}

pub fn part2(input: Input<Vec<i128>>) -> Answer<usize> {
    Answer { question: input.question, result: 0 }
}
