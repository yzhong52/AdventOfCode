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

fn parse_tree2(numbers: &Vec<i32>, i: &mut usize) -> i32 {
    if *i >= numbers.len() {
        return 0;
    }

    let child_count = numbers[*i];
    *i += 1;
    let meta_data_count = numbers[*i];
    *i += 1;

    let children_results: Vec<i32> = (0..child_count).map(|_| parse_tree2(numbers, i)).collect();

    let meta_data: Vec<i32> = (0..meta_data_count).map(|m| numbers[*i + m as usize]).collect();
    *i += meta_data_count as usize;

    if children_results.len() == 0 {
        return meta_data.iter().sum();
    } else {
        return meta_data.iter().map(|child_index| {
            if *child_index >= 1 && *child_index <= children_results.len() as i32 {
                return *children_results.get((*child_index - 1) as usize).unwrap();
            } else {
                0
            }
        }).sum();
    }
}

pub fn part1(input: Input<Vec<i32>>) -> Answer<i32> {
    let mut index: usize = 0;
    let result = parse_tree(&input.data, &mut index);
    Answer { question: input.question, result }
}

pub fn part2(input: Input<Vec<i32>>) -> Answer<i32> {
    let mut index: usize = 0;
    let result = parse_tree2(&input.data, &mut index);
    Answer { question: input.question, result }
}
