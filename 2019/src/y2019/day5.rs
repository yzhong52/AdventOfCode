use crate::int_code_computers::basic_int_code_computer::IntCodeComputer;
use crate::helpers::parser::read_numbers_by_comma;
use crate::helpers::puzzle::Puzzle;

pub struct Day5 {}

impl Puzzle<Vec<i32>, i32> for Day5 {
    fn day(&self) -> i8 {
        5
    }

    fn parser(&self) -> fn(String) -> Vec<i32> {
        read_numbers_by_comma
    }

    fn part1(&self, input: &Vec<i32>) -> i32 {
        IntCodeComputer::run_till_halt(&input, vec![1])
    }

    fn part2(&self, input: &Vec<i32>) -> i32 {
        IntCodeComputer::run_till_halt(&input, vec![5])
    }
}
