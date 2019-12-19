use super::super::helpers::parser::*;
use super::super::helpers::utils::*;
use super::day9::*;
use std::collections::{HashMap, VecDeque, BinaryHeap, HashSet};
use crate::helpers::models::BigPoint;
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
    let mut scaffold = detect_scaffold(&input.data, true);

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

pub fn part2(input: Input<Vec<i128>>) -> Answer<usize> {
    let mut scaffold = detect_scaffold(&input.data, true);

    Answer { question: input.question, result: 0 }
}
