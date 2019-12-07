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

    let mut output_number: Option<i32> = Some(initial_value);
    println!("{:?}", numbers);

    let mut index: usize = 0;
    while numbers[index] != 99 {
        let current_instruction = numbers[index];
        index += 1;

        let operation_code = current_instruction % 100;
        let mode1 = current_instruction / 100 % 10;
        let mode2 = current_instruction / 1000 % 10;
        let mode3 = current_instruction / 10000 % 10;

        println!("Current instruction is {}", current_instruction);
        println!("Operation token is {}", operation_code);
        match operation_code {
            OPERATION_ADDITION_1 | OPERATION_MULTIPLICATION_2 | OPERATION_LESS_THAN_7 | OPERATION_EQUAL_8 => {
                let parameter1 = parse_number(&numbers, mode1, index);
                let parameter2 = parse_number(&numbers, mode2, index + 1);
                let parameter3 = match mode3 {
                    POSITION_MODE => numbers[index + 2] as usize,
                    IMMEDIATE_MODE => {
                        // TODO: Yuchen - not needed.
                        println!("IMMEDIATE_MODE for parameter 3");
                        index + 2
                    },
                    i => unimplemented!("{}", i),
                };
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
                println!("OPT Input");
                let position = numbers[index] as usize;
                numbers[position] = initial_value;
                index += 1;
            }
            OPERATION_OUTPUT_4 => {
                println!("OPT Output");
                output_number = Some(numbers[numbers[index] as usize]);
                index += 1;
            }
            OPERATION_JUMP_IF_TRUE_5 | OPERATION_JUMP_IF_FALSE_6 => {
                println!("OPT Jump");
                let parameter1 = parse_number(&numbers, mode1, index);
                let parameter2 = parse_number(&numbers, mode2, index + 1);
                println!("parameter1 {} parameter2 {}", parameter1, parameter2);
                match operation_code {
                    OPERATION_JUMP_IF_TRUE_5 => {
                        if parameter1 != 0 {
                            index = parameter2 as usize
                        } else {
                            index += 2
                        }
                    }
                    OPERATION_JUMP_IF_FALSE_6 => {
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

        for x in numbers.iter().enumerate() {
            print!("[{}]{},\t", x.0, x.1);
            if (x.0 + 1) % 10 == 0 {
                println!();
            }
        }
        println!("\n\n----");
    }

    output_number.unwrap()
}

pub fn part1(input: Input<Vec<i32>>) -> Answer<i32> {
    // Answer { question: input.question, result: compute(input.data, 1) }
    Answer { question: input.question, result: 0 }
}

pub fn part2(input: Input<Vec<i32>>) -> Answer<i32> {
    let data = vec![3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
                    1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
                    999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99];
    Answer { question: input.question, result: compute(data, 5) }
}
