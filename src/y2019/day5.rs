use super::super::helpers::parser::*;

const POSITION_MODE: i32 = 0;
const IMMEDIATE_MODE: i32 = 1;

const ADDITIONS_OPERATION: i32 = 1;
const MULTIPLICATION_OPERATION: i32 = 2;

fn compute(values: Vec<i32>, initial_value: i32) -> i32 {
    let mut numbers = values.clone();

    let mut current_value: Option<i32> = Some(initial_value);

    let mut index: usize = 0;
    while numbers[index] != 99 {
        let current_instruction = numbers[index];
        let operation = current_instruction % 100;
        index += 1;

        println!("Operation token is {}", operation);
        match operation {
            ADDITIONS_OPERATION | MULTIPLICATION_OPERATION => {
                let value1: i32 = match current_instruction / 100 % 10 {
                    POSITION_MODE => {
                        println!("Value1 is at '{}' is '{}'", numbers[index], numbers[numbers[index] as usize]);
                        numbers[numbers[index] as usize]
                    }
                    IMMEDIATE_MODE => numbers[index as usize],
                    i => unimplemented!("{}", i),
                };
                index += 1;

                let value2: i32 = match current_instruction / 1000 % 100 {
                    POSITION_MODE => {
                        println!("Value2 is at '{}' is '{}'", numbers[index], numbers[numbers[index] as usize]);
                        numbers[numbers[index] as usize]
                    }
                    IMMEDIATE_MODE => numbers[index as usize],
                    i => unimplemented!("{}", i),
                };
                index += 1;

                let calculated_result: i32 = match operation {
                    ADDITIONS_OPERATION => value1 + value2,
                    MULTIPLICATION_OPERATION => value1 * value2,
                    i => unimplemented!("{}", i),
                };

                // write result
                match operation / 10000 % 1000 {
                    POSITION_MODE => {
                        // TODO: Yuchen - gives errors on one line
                        // numbers[numbers[index] as usize] = calculated_result
                        // --------^^^^^^^-----------------
                        let position = numbers[index] as usize;
                        numbers[position] = calculated_result
                    }
                    i => unimplemented!("{}", i),
                }
                index += 1;

                println!("Next 10 numbers: {:?}", &numbers[index..index + 10]);
            }
            3 => {
                // takes a single integer as input and saves it to the position given by its only parameter
                println!("Input mode to address {}", numbers[index]);
                let position = numbers[index] as usize;
                numbers[position] = current_value.unwrap();
                current_value = None;
                index += 1;
            }
            4 => {
                // outputs the value of its only parameter
                println!("Output mode from address {}", numbers[index]);
                current_value = Some(numbers[numbers[index] as usize]);
                index += 1;
            }
            i => unimplemented!("{}", i),
        };
    }

    current_value.unwrap()
}

pub fn part1(input: Input<Vec<i32>>) -> Answer<i32> {
    Answer { question: input.question, result: compute(input.data, 1) }
}

pub fn part2(input: Input<Vec<i32>>) -> Answer<i32> {
    Answer { question: input.question, result: 9999 }
}
