use crate::int_code_computers::super_int_code_computer::SuperIntCodeComputer;
use crate::helpers::parser::{Input, Answer};

fn run_day9(values: &Vec<i128>, input_value: i128) -> i128 {
    SuperIntCodeComputer::run_till_halt(values, vec![input_value])
}

pub fn part1(input: Input<Vec<i128>>) -> Answer<i128> {
    Answer { question: input.question, result: run_day9(&input.data, 1) }
}

pub fn part2(input: Input<Vec<i128>>) -> Answer<i128> {
    Answer { question: input.question, result: run_day9(&input.data, 2) }
}
