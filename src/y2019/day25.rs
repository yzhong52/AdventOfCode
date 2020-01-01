use crate::helpers::parser::{Answer, Input};
use crate::y2019::super_int_code_computer::{SuperIntCodeComputer, SuperIntCodeResult};
use crate::y2019::atomic_int_code_computer::{AtomicIntCodeComputer, AtomicIntCodeResult};
use std::io;
use std::io::BufRead;

pub fn part1(input: Input<Vec<i128>>) -> Answer<i128> {
    let mut computer = AtomicIntCodeComputer::new(input.data, "droid".to_string());

    loop {
        match computer.run() {
            AtomicIntCodeResult::Output(value) => {
                print!("{}", value as u8 as char)
            },
            AtomicIntCodeResult::WaitingInput => {
                let mut line = String::new();
                let stdin = io::stdin();
                stdin.lock().read_line(&mut line).expect("Could not read line");
                let commands = line.chars().into_iter().map(|c| c as i128).collect();
                computer.input_multiple(commands);
            },
            AtomicIntCodeResult::Halted => {},
        }
    }

    Answer { question: input.question, result: 0 }
}

pub fn part2(input: Input<Vec<i128>>) -> Answer<i128> {
    Answer { question: input.question, result: 0 }
}

