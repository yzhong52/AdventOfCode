use super::super::helpers::parser::*;

fn get_fuel(number: &i32) -> i32 {
    number / 3 - 2
}

fn get_fuel_recursive(number: &i32) -> i32 {
    let fuel = get_fuel(number);
    if fuel <= 0 {
        0
    } else {
        fuel + get_fuel_recursive(&fuel)
    }
}

pub fn part1(input: Input<Vec<i32>>) -> Answer<i32> {
    let total = input.data.iter().map(get_fuel).sum();
    Answer { question: input.question, result: total }
}

pub fn part2(input: Input<Vec<i32>>) -> Answer<i32> {
    let total = input.data.iter().map(get_fuel_recursive).sum();
    Answer { question: input.question, result: total }
}
