use super::super::helpers::parser::*;
use super::super::helpers::utils::*;
use super::day9::*;
use std::collections::{HashMap, VecDeque, BinaryHeap, HashSet};
use crate::helpers::models::BigPoint;
use std::thread::sleep;
use std::time::Duration;

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

struct Map {
    map: HashMap<BigPoint, char>,
    destination: BigPoint,
}

static MOVES: [Action; 4] = [
    Action { input: NORTH, direction: BigPoint { x: 0, y: 1 }, symbol: '^' },
    Action { input: SOUTH, direction: BigPoint { x: 0, y: -1 }, symbol: 'v' },
    Action { input: WEST, direction: BigPoint { x: -1, y: 0 }, symbol: '<' },
    Action { input: EAST, direction: BigPoint { x: 1, y: 0 }, symbol: '>' },
];

fn explore_map(input: &Vec<i128>, debug: bool) -> Map {
    let mut droid = SuperIntCodeComputer {
        numbers: input.clone(),
        index: 0,
        input_queue: VecDeque::new(),
        relative_base: 0,
        external_numbers: HashMap::new(),
    };

    let moves: [Action; 4] = [
        Action { input: NORTH, direction: BigPoint { x: 0, y: 1 }, symbol: '^' },
        Action { input: SOUTH, direction: BigPoint { x: 0, y: -1 }, symbol: 'v' },
        Action { input: WEST, direction: BigPoint { x: -1, y: 0 }, symbol: '<' },
        Action { input: EAST, direction: BigPoint { x: 1, y: 0 }, symbol: '>' },
    ];

    let mut next_move: &Action = &moves[0];
    let mut droid_pos: BigPoint = BigPoint::origin();
    let mut destination_pos: Option<BigPoint> = None;

    let mut map: HashMap<BigPoint, char> = HashMap::new();
    map.insert(BigPoint::origin(), next_move.symbol);

    loop {
        let current_move = next_move;

        droid.input_queue.push_back(current_move.input);

        match droid.run() {
            SuperIntCodeResult::Output(RESPONSE_WALL_HIT) => {
                let wall_pos = droid_pos.clone() + current_move.direction.clone();
                map.insert(wall_pos, WALL);

                // Turn left
                let new_direction = BigPoint {
                    x: -current_move.direction.y,
                    y: current_move.direction.x,
                };
                next_move = MOVES.iter().find(|x| x.direction == new_direction).unwrap();
            }
            SuperIntCodeResult::Output(RESPONSE_MOVED) => {
                if droid_pos == BigPoint::origin() {
                    map.insert(droid_pos.clone(), START);
                } else if Some(droid_pos.clone()) == destination_pos {
                    map.insert(droid_pos.clone(), TARGET);
                } else {
                    map.insert(droid_pos.clone(), EMPTY);
                }
                droid_pos += current_move.direction.clone();
                map.insert(droid_pos.clone(), current_move.symbol);

                // Turn right
                let new_direction = BigPoint {
                    x: current_move.direction.y,
                    y: -current_move.direction.x,
                };
                next_move = MOVES.iter().find(|x| x.direction == new_direction).unwrap();
            }
            SuperIntCodeResult::Output(RESPONSE_DESTINATION) => {
                map.insert(droid_pos.clone(), EMPTY);
                droid_pos += current_move.direction.clone();
                destination_pos = Some(droid_pos.clone());
                map.insert(droid_pos.clone(), EMPTY);
            }
            SuperIntCodeResult::Halted => break,
            _ => unimplemented!()
        };

        if debug {
            let buffer = get_screen_buffer(&map, UNKNOWN, 41, 41);
            print(&buffer);
            println!("Droid pos: {:?}", droid_pos);
            println!("Target pos: {:?}", droid_pos);
            println!("Map size: {} x {}", buffer.len(), buffer[0].len());

            sleep(Duration::from_millis(10));
        }
    }

    Map { map, destination: destination_pos.unwrap() }
}

//// Sort by distance to the origin
//impl std::cmp::Ord for BigPoint {
//    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//        let this = self.x * self.x + self.y * self.y;
//        let that = other.x * other.x + other.y * other.y;
//        that.cmp(&this)
//    }
//}
//
//impl std::cmp::PartialOrd for BigPoint {
//    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//        Some(self.cmp(&other))
//    }
//}

pub fn part1(input: Input<Vec<i128>>) -> Answer<usize> {
    println!("???");
    let map = explore_map(&input.data, false);

//    let mut visited = HashSet::new();
//
//    let mut queue = BinaryHeap::new();
//
//    visited.insert(BigPoint::origin());
//    queue.push((0, BigPoint::origin()));
//
    let mut distance_to_destination = 0;
//    while queue.len() > 0 {
//        let current_item = queue.pop().unwrap();
//
//        if current_item.1 == map.destination {
//            distance_to_destination = current_item.0;
//            break
//        }
//
//        for m in MOVES.iter() {
//            let new_position = current_item.1.clone() + m.direction.clone();
//            if !visited.contains(&new_position) && map.map.get(&new_position).unwrap() != &WALL {
//                queue.push((current_item.0 + 1, new_position.clone()));
//                visited.insert(new_position);
//            }
//        };
//        println!("{}", queue.len());
//    }

    Answer { question: input.question, result: distance_to_destination }
}

pub fn part2(input: Input<Vec<i128>>) -> Answer<i128> {
    Answer { question: input.question, result: 0 }
}
