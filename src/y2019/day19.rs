use super::super::helpers::parser::*;
use crate::int_code_computers::super_int_code_computer::SuperIntCodeComputer;
use crate::int_code_computers::super_int_code_computer::SuperIntCodeResult;
use crate::helpers::puzzle::Puzzle;

fn test_location(x: i128, y: i128, data: &Vec<i128>) -> i128 {
    let mut drone = SuperIntCodeComputer::new(data.clone());

    drone.input(x);
    drone.input(y);

    match drone.run() {
        SuperIntCodeResult::Output(value) => value,
        SuperIntCodeResult::Halted => unimplemented!()
    }
}

pub struct Day19 {}

impl Puzzle<Vec<i128>, i128> for Day19 {
    fn day(&self) -> i8 {
        19
    }

    fn parser(&self) -> fn(String) -> Vec<i128> {
        parse_numbers_by_comma
    }

    fn part1(&self, input: &Vec<i128>) -> i128 {
        let mut final_result = 0;
        for i in 0..50 {
            for j in 0..50 {
                final_result += test_location(i, j, input);
            }
        }
        final_result
    }

    fn part2(&self, input: &Vec<i128>) -> i128 {
        // Taken from part 1
        let mut x = 11;
        let mut y = 14;

        let mut x_size = 0;
        let mut y_size = 0;

        loop {
            while test_location(x + x_size, y, input) == 1 {
                x_size += 1;
            }

            while test_location(x, y + y_size, input) == 1 {
                y_size += 1;
            }

            if x_size >= 100 && y_size >= 100 {
                break;
            }

            println!("Maximum grid for location ({}, {}) is ({}, {})", x, y, x_size, y_size);

            if x_size > y_size {
                x += 1;
                x_size -= 1;
            } else {
                y += 1;
                y_size -= 1;
            }
        }
        10000 * x + y
    }
}
