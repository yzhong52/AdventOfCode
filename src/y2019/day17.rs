use super::super::helpers::parser::*;
use super::super::helpers::utils::*;
use super::day9::*;
use std::collections::{HashMap, VecDeque, BinaryHeap, HashSet};
use crate::helpers::models::{BigPoint, _Point};
use std::thread::sleep;
use std::time::Duration;
use std::char;
use std::fmt::{Debug, Formatter, Error};
use std::process::exit;

const INTERSECTION: char = 'O';
const FRAME: char = '#';

fn detect_scaffold(input: &Vec<i128>, debug: bool) -> Vec<Vec<char>> {
    let mut vacuum_robot = SuperIntCodeComputer {
        numbers: input.clone(),
        index: 0,
        input_queue: VecDeque::new(),
        relative_base: 0,
        external_numbers: HashMap::new(),
    };

    let mut scaffold: Vec<Vec<char>> = vec![vec![]; 0];
    let mut row = vec![];
    loop {
        match vacuum_robot.run() {
            SuperIntCodeResult::Output(value) => {
                match value as u8 as char {
                    '\n' => {
                        // Need this check because the robot output an extra line break at the end
                        if row.len() > 0 {
                            scaffold.push(row.clone());
                        }
                        row = vec![];
                    }
                    ch => {
                        row.push(ch)
                    }
                }
            }
            SuperIntCodeResult::Halted => break,
        };
    }

    scaffold
}


pub fn part1(input: Input<Vec<i128>>) -> Answer<usize> {
    let mut scaffold: Vec<Vec<char>> = detect_scaffold(&input.data, true);

    let mut result = 0;
    for r in 1..&scaffold.len() - 1 {
        for c in 1..&scaffold[r].len() - 1 {
            if &scaffold[r][c] == &FRAME
                && &scaffold[r + 1][c] == &FRAME && &scaffold[r][c + 1] == &FRAME
                && &scaffold[r - 1][c] == &FRAME && &scaffold[r][c - 1] == &FRAME {
                result += r * c;
            }
        }
    }

    Answer { question: input.question, result }
}

fn start_pos(scaffold: &Vec<Vec<char>>) -> BigPoint {
    for r in 0..scaffold.len() {
        for c in 0..scaffold[r].len() {
            if scaffold[r][c] == '^' {
                return BigPoint { x: r as i128, y: c as i128 };
            }
        }
    }
    BigPoint::origin()
}

#[derive(Clone)]
enum Action {
    Forward(u8),
    TurnLeft,
    TurnRight,
}

impl Debug for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Action::Forward(value) => write!(f, "{}", value),
            Action::TurnLeft => write!(f, "L"),
            Action::TurnRight => write!(f, "R"),
        }
    }
}


// TODO: Yuchen - let's assume we visit each place maximum once for now.
// Ok, this has 200k different combination :'(
fn path_search(
    visited: &mut Vec<Vec<i32>>,
    to_visit: &mut HashSet<BigPoint>,
    actions: Vec<Action>,
    current_position: BigPoint,
    facing_direction: BigPoint,
    solutions: &mut Vec<Vec<Action>>,
    scaffold: &Vec<Vec<char>>)
{
    to_visit.remove(&current_position);

    // if no action can be taken, then we reach the end
    if to_visit.is_empty() {
        solutions.push(actions.clone())
    }

    let max_x = visited.len();
    let max_y = visited[0].len();

    println!("-->-->-->-->-->--> {:?}", actions);
    for i in 0..max_x {
        for j in 0..max_y {
            if visited[i][j] > 0 {
                print!("{} ", visited[i][j])
            } else {
                print!("{} ", scaffold[i][j]);
            }
        }
        println!()
    }
    sleep(Duration::from_millis(80));

    let move_directions: Vec<BigPoint> = vec![
        facing_direction.clone(),
        facing_direction.turn_left(),
        facing_direction.turn_right()
    ];
    for next_dir in move_directions.iter() {
        let next_position = current_position.clone() + next_dir.clone();

        if next_position.x >= 0 && next_position.x < max_x as i128 &&
            next_position.y >= 0 && next_position.y < max_y as i128 {
            let visited_count = visited[next_position.x as usize][next_position.y as usize];
            let is_scaffold = scaffold[next_position.x as usize][next_position.y as usize] == FRAME;
            let is_intersection = scaffold[next_position.x as usize][next_position.y as usize] == INTERSECTION;

            if visited_count == 0 && is_scaffold || is_intersection {

                // Let's assume the robot don't turn around 180 degrees for now
                if next_dir.clone().dot_product(facing_direction.clone()) >= 0 {
                    let mut next_actions = actions.clone();


                    if facing_direction.turn_right() == *next_dir {
                        next_actions.push(Action::TurnRight);
                        next_actions.push(Action::Forward(0));
                    } else if facing_direction.turn_left() == *next_dir {
                        next_actions.push(Action::TurnLeft);
                        next_actions.push(Action::Forward(0));
                    }

                    // Same direction, keep moving
                    match next_actions.pop().unwrap() {
                        Action::Forward(value) => {
                            next_actions.push(Action::Forward(value + 1))
                        }
                        _ => unimplemented!()
                    }

                    visited[next_position.x as usize][next_position.y as usize] += 1;
                    path_search(
                        visited,
                        to_visit,
                        next_actions,
                        next_position.clone(),
                        next_dir.clone(),
                        solutions,
                        &scaffold,
                    );
                    visited[next_position.x as usize][next_position.y as usize] -= 1;
                }
            }
        }
    }

    to_visit.insert(current_position);
}

pub fn part2(input: Input<Vec<i128>>) -> Answer<usize> {
    let mut scaffold: Vec<Vec<char>> = detect_scaffold(&input.data, true);
    let mut to_visit: HashSet<BigPoint> = HashSet::new();
    for r in 1..&scaffold.len() - 1 {
        for c in 1..&scaffold[r].len() - 1 {
            if &scaffold[r][c] == &FRAME
                && &scaffold[r + 1][c] == &FRAME && &scaffold[r][c + 1] == &FRAME
                && &scaffold[r - 1][c] == &FRAME && &scaffold[r][c - 1] == &FRAME {
                scaffold[r][c] = INTERSECTION;
                to_visit.insert(BigPoint { x: r as i128, y: c as i128 });
            }
        }
    }

    // find the starting position of the robot
    let start = start_pos(&scaffold);
    println!("Starting position is: {:?}", start);

    let mut visited: Vec<Vec<i32>> = vec![vec![0; scaffold[0].len()]; scaffold.len()];
    let mut solutions: Vec<Vec<Action>> = vec![vec![]];
    let empty_actions: Vec<Action> = vec![];
    let direction = BigPoint { x: -1, y: 0 };
    path_search(
        visited.as_mut(),
        &mut to_visit,
        empty_actions,
        start,
        direction,
        solutions.as_mut(),
        &scaffold);

    println!("{}", solutions.len());
    println!("{:?}", solutions);

    Answer { question: input.question, result: 0 }
}
