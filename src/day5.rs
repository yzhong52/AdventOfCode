use super::helpers::parser::*;

use std::cmp::min;

fn diff(c1: &char, c2: &char) -> i32 {
    return i32::abs(*c1 as i32 - *c2 as i32);
}

pub fn remains(input: String) -> Vec<char> {
    let mut stack: Vec<char> = Vec::new();

    for x in input.chars() {
        match stack.last() {
            Some(c) => {
                if diff(&x, c) == diff(&'A', &'a') {
                    // println!("Has some value {} - {} match", c, x);
                    stack.pop();
                } else {
                    // println!("Has some value {} pushed", c);
                    stack.push(x);
                }
            }
            None => {
                // println!("It is empty");
                stack.push(x);
            }
        }
    }
    return stack;
}

pub fn part1(input: Input<String>) -> Answer<usize> {
    let stack = remains(input.data);
    return Answer { question: input.question, result: stack.len() };
}

pub fn part2(input: Input<String>) -> Answer<usize> {
    let part1_result = remains(input.data);

    let mut result = part1_result.len();

    for d in 0..26 {
        let ignore_c1: char = ('a' as i32 + d) as u8 as char;
        let ignore_c2: char = ('A' as i32 + d) as u8 as char;

        let filtered_input: String = part1_result.iter()
            .filter(|x| **x != ignore_c1 && **x != ignore_c2)
            .collect();

        result = min(result, remains(filtered_input).len());
    }

    return Answer { question: input.question, result };
}