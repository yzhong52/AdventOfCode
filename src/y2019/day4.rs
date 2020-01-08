use super::super::helpers::parser::*;
use std::ops::RangeInclusive;
use crate::helpers::puzzle::Puzzle;

fn meet_the_rule(number: i32) -> bool {
    let mut remain = number;
    let mut last_digit = 10;
    let mut same_digit = false;
    while remain > 0 {
        let digit = remain % 10;
        remain /= 10;

        if digit > last_digit {
            return false;
        }
        same_digit = same_digit || (digit == last_digit);
        last_digit = digit;
    }
    return same_digit;
}

fn meet_the_rule_part2(number: i32) -> bool {
    let mut remain = number;
    let mut last_digit = 10;
    let mut same_digit_repeated_twice = false;
    let mut same_digit_count = 1;
    while remain > 0 {
        let digit = remain % 10;

        if digit > last_digit {
            return false;
        }

        if digit == last_digit {
            same_digit_count += 1;
        } else {
            if same_digit_count == 2 {
                same_digit_repeated_twice = true
            }
            same_digit_count = 1;
        }

        last_digit = digit;
        remain /= 10;
    }

    return same_digit_count == 2 || same_digit_repeated_twice;
}

pub fn part1(input: Input<RangeInclusive<i32>>) -> Answer<i32> {
    let result: i32 = input.data.map(meet_the_rule).map(|x| x as i32).sum();
    Answer { question: input.question, result }
}

pub fn part2(input: Input<RangeInclusive<i32>>) -> Answer<i32> {
    let result: i32 = input.data.map(meet_the_rule_part2).map(|x| x as i32).sum();
    Answer { question: input.question, result }
}

pub struct Day4 {}

impl Puzzle<Vec<i32>, i32> for Day4 {
    fn day(&self) -> i8 {
        4
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
