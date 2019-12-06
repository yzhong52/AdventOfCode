use super::super::helpers::parser::*;
use std::ops::RangeInclusive;

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

pub fn part1(input: &Input<RangeInclusive<i32>>) -> Answer<i32> {
    let mut count = 0;
    for i in input.data.clone() {
        // TODO: Yuchen when need to clone here?
        count += meet_the_rule(i) as i32;
    }
    Answer { question: input.question, result: count }
}


pub fn part2(input: &Input<RangeInclusive<i32>>) -> Answer<i32> {
    let mut count = 0;
    for i in input.data.clone() {
        // TODO: Yuchen when need to clone here?
        count += meet_the_rule_part2(i) as i32;
    }
    Answer { question: input.question, result: count }
}
