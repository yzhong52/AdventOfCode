use super::super::helpers::parser::*;

pub fn part1(input: Input<Vec<i32>>) -> Answer<i32> {
    let mut total = 0;
    for number in input.data {
        total += get_fuel(number);
    }
    return Answer { question: input.question, result: total };
}

fn get_fuel(number: i32) -> i32 {
    return number / 3 - 2;
}

fn get_fuel_recursive(number: i32) -> i32 {
    let fuel = get_fuel(number);
    if fuel <= 0 {
        return 0;
    } else {
        return fuel + get_fuel_recursive(fuel);
    }
}

pub fn part2(input: Input<Vec<i32>>) -> Answer<i32> {
    let mut total = 0;
    for number in input.data {
        total += get_fuel_recursive(number);
    }
    return Answer { question: input.question, result: total };
}
