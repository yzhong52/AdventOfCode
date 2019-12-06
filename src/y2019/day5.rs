use super::super::helpers::parser::*;
use std::process::exit;

const POSITION_MODE: i32 = 0;
const IMMEDIATE_MODE: i32 = 1;

const OPERATION_ADDITIONS: i32 = 1;
const OPERATION_MULTIPLICATION: i32 = 2;
const OPERATION_INPUT: i32 = 3;
const OPERATION_OUTPUT: i32 = 4;
const OPERATION_JUMP_IF_TRUE: i32 = 5;
const OPERATION_JUMP_IF_FALSE: i32 = 6;
const OPERATION_LESS_THAN: i32 = 7;
const OPERATION_EQUAL: i32 = 8;

fn parse_number(numbers: &Vec<i32>, mode: i32, index: usize) -> i32 {
    match mode {
        POSITION_MODE => {
            println!("Value1 is at '{}' is '{}'", numbers[index], numbers[numbers[index] as usize]);
            numbers[numbers[index] as usize]
        }
        IMMEDIATE_MODE => numbers[index as usize],
        i => unimplemented!("{}", i),
    }
}

fn compute(values: Vec<i32>, initial_value: i32) -> i32 {
    let mut numbers = values.clone();

    let mut current_value: Option<i32> = Some(initial_value);
    println!("{:?}", numbers);

    let mut index: usize = 0;
    while numbers[index] != 99 {
        let current_instruction = numbers[index];
        index += 1;

        let operation_code = current_instruction % 100;
        let mode1 = current_instruction / 100 % 10;
        let mode2 = current_instruction / 1000 % 10;

        println!("Current instruction is {}", current_instruction);
        println!("Operation token is {}", operation_code);
        match operation_code {
            OPERATION_ADDITIONS | OPERATION_MULTIPLICATION | OPERATION_LESS_THAN | OPERATION_EQUAL => {
                let parameter1 = parse_number(&numbers, mode1, index);
                let parameter2 = parse_number(&numbers, mode2, index + 1);
                let parameter3 = numbers[index + 2] as usize;
                numbers[parameter3] = match operation_code {
                    OPERATION_ADDITIONS => parameter1 + parameter2,
                    OPERATION_MULTIPLICATION => parameter1 * parameter2,
                    OPERATION_LESS_THAN => (parameter1 < parameter2) as i32,
                    OPERATION_EQUAL => (parameter1 == parameter2) as i32,
                    i => unimplemented!("{}", i),
                };
                index += 3;
            }
            OPERATION_INPUT => {
                let position = numbers[index] as usize;
                numbers[position] = current_value.unwrap();
                current_value = None;
                index += 1;
            }
            OPERATION_OUTPUT => {
                current_value = Some(numbers[numbers[index] as usize]);
                index += 1;
            }
            OPERATION_JUMP_IF_TRUE | OPERATION_JUMP_IF_FALSE => {
                println!("Let's just . Current index {}", index);

                let parameter1 = parse_number(&numbers, mode1, index);
                let parameter2 = parse_number(&numbers, mode2, index + 1);
                println!("index {} p1 {} p2 {}", index, parameter1, parameter2);
                match operation_code {
                    OPERATION_JUMP_IF_TRUE => {
                        if parameter1 != 0 {
                            index = parameter2 as usize
                        } else {
                            index += 2
                        }
                    }
                    OPERATION_JUMP_IF_FALSE => {
                        if parameter1 == 0 {
                            index = parameter2 as usize
                        } else {
                            index += 2
                        }
                    }
                    i => unimplemented!("{}", i),
                }
                println!("Index {}", index);
            }
            i => unimplemented!("{}", i),
        };
        println!("Index is set to {}", index);
        println!("{:?}", numbers);

        for x in numbers.iter().enumerate() {
            print!("[{}]{},", x.0, x.1);
        }
        println!("\n\n----");
    }

    current_value.unwrap()
}

pub fn part1(input: Input<Vec<i32>>) -> Answer<i32> {
    Answer { question: input.question, result: compute(input.data, 1) }
//    Answer { question: input.question, result: 0 }
}

pub fn part2(input: Input<Vec<i32>>) -> Answer<i32> {
    let data = vec![3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
                    1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
                    999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99];
    Answer { question: input.question, result: compute(data, 5) }
}
