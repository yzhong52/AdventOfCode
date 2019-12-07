use super::super::helpers::parser::*;

const POSITION_MODE: i32 = 0;
const IMMEDIATE_MODE: i32 = 1;

const OPERATION_ADDITION_1: i32 = 1;
const OPERATION_MULTIPLICATION_2: i32 = 2;
const OPERATION_INPUT_3: i32 = 3;
const OPERATION_OUTPUT_4: i32 = 4;
const OPERATION_JUMP_IF_TRUE_5: i32 = 5;
const OPERATION_JUMP_IF_FALSE_6: i32 = 6;
const OPERATION_LESS_THAN_7: i32 = 7;
const OPERATION_EQUAL_8: i32 = 8;

fn parse_number(numbers: &Vec<i32>, mode: i32, index: usize) -> i32 {
    match mode {
        POSITION_MODE => numbers[numbers[index] as usize],
        IMMEDIATE_MODE => numbers[index as usize],
        i => unimplemented!("{}", i),
    }
}

pub fn compute(values: &Vec<i32>, input_values: Vec<i32>) -> i32 {
    let mut input_index = 0;
    let mut numbers = values.clone();

    let mut output_number: Option<i32> = None;

    let mut index: usize = 0;
    while numbers[index] != 99 {
        let current_instruction = numbers[index];
        let operation_code = current_instruction % 100;
        let mode1 = current_instruction / 100 % 10;
        let mode2 = current_instruction / 1000 % 10;

        index += 1;

        match operation_code {
            OPERATION_ADDITION_1 | OPERATION_MULTIPLICATION_2 | OPERATION_LESS_THAN_7 | OPERATION_EQUAL_8 => {
                let parameter1 = parse_number(&numbers, mode1, index);
                let parameter2 = parse_number(&numbers, mode2, index + 1);
                let parameter3 = numbers[index + 2] as usize;
                numbers[parameter3] = match operation_code {
                    OPERATION_ADDITION_1 => parameter1 + parameter2,
                    OPERATION_MULTIPLICATION_2 => parameter1 * parameter2,
                    OPERATION_LESS_THAN_7 => (parameter1 < parameter2) as i32,
                    OPERATION_EQUAL_8 => (parameter1 == parameter2) as i32,
                    i => unimplemented!("{}", i),
                };
                index += 3;
            }
            OPERATION_INPUT_3 => {
                let position = numbers[index] as usize;
                numbers[position] = input_values[input_index];
                input_index += 1;
                index += 1;
            }
            OPERATION_OUTPUT_4 => {
                output_number = Some(parse_number(&numbers, mode1, index));
                index += 1;
            }
            OPERATION_JUMP_IF_TRUE_5 | OPERATION_JUMP_IF_FALSE_6 => {
                let parameter1 = parse_number(&numbers, mode1, index);
                let parameter2 = parse_number(&numbers, mode2, index + 1);
                index = match operation_code {
                    OPERATION_JUMP_IF_TRUE_5 if parameter1 != 0 => parameter2 as usize,
                    OPERATION_JUMP_IF_FALSE_6 if parameter1 == 0 => parameter2 as usize,
                    _ => index + 2,
                }
            }
            i => unimplemented!("{}", i),
        };
    }

    output_number.unwrap()
}

fn compute_day5(values: Vec<i32>, initial_value: i32) -> i32 {
    compute(&values, vec![initial_value])
}

pub fn part1(input: Input<Vec<i32>>) -> Answer<i32> {
    Answer { question: input.question, result: compute_day5(input.data, 1) }
}

pub fn part2(input: Input<Vec<i32>>) -> Answer<i32> {
    Answer { question: input.question, result: compute_day5(input.data, 5) }
}
