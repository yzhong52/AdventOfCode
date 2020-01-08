use super::super::helpers::parser::*;
use crate::helpers::puzzle::Puzzle;

fn compute(input: &Vec<i32>, noun: i32, verb: i32) -> i32 {
    let mut numbers = input.clone();
    let mut index: usize = 0;
    numbers[1] = noun;
    numbers[2] = verb;

    while numbers[index] != 99 {
        let operator = numbers[index];
        let number1 = numbers[numbers[index as usize + 1] as usize];
        let number2 = numbers[numbers[index + 2] as usize];
        let result_index = numbers[index + 3] as usize;
        numbers[result_index] = match operator {
            1 => number1 + number2,
            2 => number1 * number2,
            _ => unimplemented!(),
        };
        index += 4;
    }

    numbers[0]
}

pub struct Day2 {}

impl Puzzle<Vec<i32>, i32> for Day2 {
    fn day(&self) -> i8 {
        2
    }

    fn parser(&self) -> fn(String) -> Vec<i32> {
        read_numbers_by_comma
    }

    fn part1(&self, input: &Vec<i32>) -> i32 {
        compute(&input, 12, 2)
    }

    fn part2(&self, input: &Vec<i32>) -> i32 {
        for noun in 0..100 {
            for verb in 0..100 {
                if compute(&input, noun, verb) == 19690720 {
                    return 100 * noun + verb;
                }
            }
        }
        unimplemented!()
    }
}
