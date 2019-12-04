use super::super::helpers::parser::*;
use crate::helpers::models::Point;
use std::collections::{HashSet, HashMap};
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

    if same_digit {
        println!("found one {}", number);
    }
    return same_digit;
}

pub fn part1(input: &Input<RangeInclusive<i32>>) -> Answer<i32> {
    let mut count = 0;
    for i in input.data.clone() {
        // TODO: Yuchen when need to clone here?
        count += meet_the_rule(i) as i32;
    }
    Answer { question: input.question, result: count }
}
