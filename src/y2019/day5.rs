use crate::int_code_computers::basic_int_code_computer::IntCodeComputer;
use crate::helpers::parser::{Input, Answer};

pub fn part1(input: Input<Vec<i32>>) -> Answer<i32> {
    let result = IntCodeComputer::run_till_halt(&input.data, vec![1]);
    Answer { question: input.question, result }
}

pub fn part2(input: Input<Vec<i32>>) -> Answer<i32> {
    let result = IntCodeComputer::run_till_halt(&input.data, vec![5]);
    Answer { question: input.question, result }
}
