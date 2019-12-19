use super::super::helpers::parser::*;
use super::super::helpers::utils::*;
use super::day9::*;
use std::collections::{HashMap, VecDeque, BinaryHeap, HashSet};
use crate::helpers::models::{BigPoint, _Point};
use std::thread::sleep;
use std::time::Duration;
use std::char;

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
            if &scaffold[r][c] == &'#' && &scaffold[r + 1][c] == &'#' && &scaffold[r - 1][c] == &'#' && &scaffold[r][c + 1] == &'#' && &scaffold[r][c - 1] == &'#' {
                scaffold[r][c] = '0';
                result += r * c;
            }
        }
    }

    for r in 0..scaffold.len() {
        for c in 0..scaffold[r].len() {
            print!("{} ", scaffold[r][c]);
        }
        println!()
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

#[derive(Debug, Clone)]
enum Action {
    Forward(u8),
    TurnLeft,
    TurnRight,
}


// TODO: Yuchen - let's assume we visit each place maximum once for now.
fn path_search(
    visited: &mut Vec<Vec<i32>>,
    actions: Vec<Action>,
    current_position: BigPoint,
    facing_direction: BigPoint,
    solutions: &Vec<Vec<Action>>,
    scaffold: &Vec<Vec<char>>)
{
    let move_directions: [_Point<i128>; 4] = [
        BigPoint { x: -1, y: 0 },
        BigPoint { x: 1, y: 0 },
        BigPoint { x: 0, y: -1 },
        BigPoint { x: 0, y: 1 },
    ];

    let max_x = visited.len();
    let max_y = visited[0].len();

    for next_dir in move_directions.iter() {
        let next_position = current_position.clone() + next_dir.clone();

        if next_position.x >= 0 && next_position.x < max_x as i128 &&
            next_position.y >= 0 && next_position.y <= max_y as i128 &&
            visited[next_position.x as usize][next_position.y as usize] == 0 && // haven't visit
            scaffold[next_position.x as usize][next_position.y as usize] == '#' // is scaffold
        {
            if next_dir.x != facing_direction.x && next_dir.y != facing_direction.y {
                // Let's assume the robot don't turn around 180 degrees for now
                unimplemented!()
            } else if next_dir.x == facing_direction.x || next_dir.y == facing_direction.y {
                // Same direction, keep moving
                let mut next_actions = actions.clone();
                match next_actions.pop().unwrap() {
                    Action::Forward(value) => {
                        next_actions.push(Action::Forward(value + 1))
                    }
                    _ => unimplemented!()
                }

                visited[next_position.x as usize][next_position.y as usize] += 1;
                path_search(
                    visited,
                    next_actions,
                    next_position.clone(),
                    next_dir.clone(),
                    &solutions,
                    &scaffold,
                );
                visited[next_position.x as usize][next_position.y as usize] -= 1;
            }
        }
    }
}

pub fn part2(input: Input<Vec<i128>>) -> Answer<usize> {
    let mut scaffold: Vec<Vec<char>> = detect_scaffold(&input.data, true);

    // find the starting position of the robot
    let start = start_pos(&scaffold);
    println!("{:?}", start);

    let mut visited: Vec<Vec<i32>> = vec![vec![0; scaffold[0].len()]; scaffold.len()];
    let solutions: Vec<Vec<Action>> = vec![vec![]];
    let empty_actions: Vec<Action> = vec![];
    let direction = BigPoint { x: -1, y: 0 };
    path_search(
        visited.as_mut(),
        empty_actions,
        start,
        direction,
        &solutions,
        &scaffold);

    println!("{}", solutions.len());
    println!("{:?}", solutions);

    Answer { question: input.question, result: 0 }
}
