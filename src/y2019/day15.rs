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
    while unknown_land.len() > 0 {
        let current_move = next_action;

        droid.input_queue.push_back(current_move.input);

        match droid.run() {
            SuperIntCodeResult::Output(RESPONSE_WALL_HIT) => {
                let wall_pos = droid_pos.clone() + current_move.direction.clone();
                map.insert(wall_pos.clone(), WALL);

                // Turn left
                let new_direction = BigPoint {
                    x: -current_move.direction.y,
                    y: current_move.direction.x,
                };
                next_action = ACTIONS.iter().find(|x| x.direction == new_direction).unwrap();
                unknown_land.remove(&wall_pos);
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

                unknown_land.remove(&droid_pos);
                for action in ACTIONS.iter() {
                    let nearby_position = droid_pos.clone() + action.direction.clone();
                    if !map.contains_key(&nearby_position) {
                        unknown_land.insert(nearby_position);
                    }
                }

                // Turn right
                let new_direction = BigPoint {
                    x: current_move.direction.y,
                    y: -current_move.direction.x,
                };
                next_action = ACTIONS.iter().find(|x| x.direction == new_direction).unwrap();
            }
            SuperIntCodeResult::Output(RESPONSE_DESTINATION) => {
                map.insert(droid_pos.clone(), EMPTY);
                droid_pos += current_move.direction.clone();
                destination_pos = Some(droid_pos.clone());
                map.insert(droid_pos.clone(), EMPTY);

                unknown_land.remove(&droid_pos);
                for action in ACTIONS.iter() {
                    let nearby_position = droid_pos.clone() + action.direction.clone();
                    if !map.contains_key(&nearby_position) {
                        unknown_land.insert(nearby_position);
                    }
                }
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
            println!("Unknown tiles: {}", unknown_land.len());

            sleep(Duration::from_millis(20));
        }
    }

    ExploredMap { map, destination: destination_pos.unwrap() }
}

// Sort by distance to the origin
impl std::cmp::Ord for BigPoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let this = self.x * self.x + self.y * self.y;
        let that = other.x * other.x + other.y * other.y;
        that.cmp(&this)
    }
}

// https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
// `target` is, if `None`, it will return the last furthest tile on the map
fn dijkstra(map: &HashMap<BigPoint, char>, source: BigPoint, target: Option<BigPoint>) -> usize {
    let mut visited = HashSet::new();

    let mut queue = BinaryHeap::new();

    visited.insert(source.clone());
    queue.push((0, source));

    let mut distance_to_destination = 0;
    while queue.len() > 0 {
        let current_item = queue.pop().unwrap();
        println!("Current {:?}", current_item.clone());
        println!("Queued {:?}", queue.len());

        distance_to_destination = current_item.0;

        match target {
            Some(value) if value == current_item.1.clone() => {
                break
            }
            _ => ()
        }

        for m in ACTIONS.iter() {
            let new_position = current_item.1.clone() + m.direction.clone();
            if !visited.contains(&new_position) && map.get(&new_position).unwrap() != &WALL {
                queue.push((current_item.0 + 1, new_position.clone()));
                visited.insert(new_position);
            }
        };
    }

    distance_to_destination
}

impl std::cmp::PartialOrd for BigPoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

pub fn part1(input: Input<Vec<i128>>) -> Answer<usize> {
    let explored = explore_map(&input.data, false);
    let distance_to_destination = dijkstra(
        &explored.map,
        BigPoint::origin(),
        Some(explored.destination),
    );
    Answer { question: input.question, result: distance_to_destination }
}

pub fn part2(input: Input<Vec<i128>>) -> Answer<usize> {
    let explored = explore_map(&input.data, false);
    let distance_to_destination = dijkstra(
        &explored.map,
        explored.destination,
        None
    );
    Answer { question: input.question, result: distance_to_destination }
}
