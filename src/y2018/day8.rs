use super::super::helpers::parser::*;

fn parse_tree(numbers: &Vec<i32>, i: &mut usize) -> i32 {
    if *i >= numbers.len() {
        return 0;
    }

    let child_count = numbers[*i];
    *i += 1;
    let meta_data_count = numbers[*i];
    *i += 1;

    let mut total = 0;
    for _ in 0..child_count {
        total += parse_tree(numbers, i);
    }
    for _ in 0..meta_data_count {
        total += numbers[*i];
        *i += 1;
    }
    total
}

pub fn part1(input: Input<Vec<i32>>) -> Answer<i32> {
    let mut index: usize = 0;
    let result = parse_tree(&input.data, &mut index);
    Answer { question: input.question, result }
}
