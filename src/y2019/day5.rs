use crate::int_code_computers::basic_int_code_computer::IntCodeComputer;
use crate::helpers::parser::{Input, Answer};
use crate::helpers::puzzle::Puzzle;

pub fn part1(input: Input<Vec<i32>>) -> Answer<i32> {
    let result = IntCodeComputer::run_till_halt(&input.data, vec![1]);
    Answer { question: input.question, result }
}

pub fn part2(input: Input<Vec<i32>>) -> Answer<i32> {
    let result = IntCodeComputer::run_till_halt(&input.data, vec![5]);
    Answer { question: input.question, result }
}

pub struct Day5 {}

impl Puzzle<Vec<i32>, i32> for Day5 {
    fn day(&self) -> i8 {
        unimplemented!()
    }

    fn parser(&self) -> fn(String) -> Vec<i32> {
        unimplemented!()
    }

    fn part1(&self, input: &Vec<i32>) -> i32 {
        unimplemented!()
    }

    fn part2(&self, input: &Vec<i32>) -> i32 {
        unimplemented!()
    }
}
