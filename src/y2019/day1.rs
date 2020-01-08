use super::super::helpers::parser::*;
use super::super::helpers::puzzle::Puzzle;

pub struct Day1 {}

impl Day1 {
    fn get_fuel(number: &i32) -> i32 {
        number / 3 - 2
    }

    fn get_fuel_recursive(number: &i32) -> i32 {
        let fuel = Day1::get_fuel(number);
        if fuel <= 0 {
            0
        } else {
            fuel + Day1::get_fuel_recursive(&fuel)
        }
    }
}

impl Puzzle<Vec<i32>, i32> for Day1 {
    fn day(&self) -> i8 {
        1
    }

    fn parser(&self) -> fn(String) -> Vec<i32> {
        parse_numbers_by_line
    }

    fn part1(&self, input: &Vec<i32>) -> i32 {
        input.iter().map(Day1::get_fuel).sum()
    }

    fn part2(&self, input: &Vec<i32>) -> i32 {
        input.iter().map(Day1::get_fuel_recursive).sum()
    }
}
