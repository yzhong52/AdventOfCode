use crate::int_code_computers::super_int_code_computer::SuperIntCodeComputer;
use crate::helpers::parser::parse_numbers_by_comma;
use crate::helpers::puzzle::Puzzle;

fn run_day9(values: &Vec<i128>, input_value: i128) -> i128 {
    SuperIntCodeComputer::run_till_halt(values, vec![input_value])
}

pub struct Day9 {}

impl Puzzle<Vec<i128>, i128> for Day9 {
    fn day(&self) -> i8 {
        9
    }

    fn parser(&self) -> fn(String) -> Vec<i128> {
        parse_numbers_by_comma
    }

    fn part1(&self, input: &Vec<i128>) -> i128 {
        run_day9(&input, 1)
    }

    fn part2(&self, input: &Vec<i128>) -> i128 {
        run_day9(&input, 2)
    }
}
