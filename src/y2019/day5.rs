use super::super::helpers::parser::*;
use std::collections::VecDeque;

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

enum IntCodeResult {
    Output(i32),
    Halted,
}

struct IntCodeComputer {
    index: usize,
    numbers: Vec<i32>,
    input_queue: VecDeque<i32>,
}

impl IntCodeComputer {
    pub fn run(&mut self) -> IntCodeResult {
        while self.numbers[self.index] != 99 {
            let current_instruction = self.numbers[self.index];
            let operation_code = current_instruction % 100;
            let mode1 = current_instruction / 100 % 10;
            let mode2 = current_instruction / 1000 % 10;

            self.index += 1;

            match operation_code {
                OPERATION_ADDITION_1 | OPERATION_MULTIPLICATION_2 | OPERATION_LESS_THAN_7 | OPERATION_EQUAL_8 => {
                    let parameter1 = parse_number(&self.numbers, mode1, self.index);
                    let parameter2 = parse_number(&self.numbers, mode2, self.index + 1);
                    let parameter3 = self.numbers[self.index + 2] as usize;
                    self.numbers[parameter3] = match operation_code {
                        OPERATION_ADDITION_1 => parameter1 + parameter2,
                        OPERATION_MULTIPLICATION_2 => parameter1 * parameter2,
                        OPERATION_LESS_THAN_7 => (parameter1 < parameter2) as i32,
                        OPERATION_EQUAL_8 => (parameter1 == parameter2) as i32,
                        i => unimplemented!("{}", i),
                    };
                    self.index += 3;
                }
                OPERATION_INPUT_3 => {
                    let position = self.numbers[self.index] as usize;
                    self.numbers[position] = self.input_queue.pop_front().unwrap();
                    self.index += 1;
                }
                OPERATION_OUTPUT_4 => {
                    let output_number = parse_number(&self.numbers, mode1, self.index);
                    self.index += 1;
                    return IntCodeResult::Output(output_number);
                }
                OPERATION_JUMP_IF_TRUE_5 | OPERATION_JUMP_IF_FALSE_6 => {
                    let parameter1 = parse_number(&self.numbers, mode1, self.index);
                    let parameter2 = parse_number(&self.numbers, mode2, self.index + 1);
                    self.index = match operation_code {
                        OPERATION_JUMP_IF_TRUE_5 if parameter1 != 0 => parameter2 as usize,
                        OPERATION_JUMP_IF_FALSE_6 if parameter1 == 0 => parameter2 as usize,
                        _ => self.index + 2,
                    }
                }
                i => unimplemented!("{}", i),
            };
        }

        IntCodeResult::Halted
    }
}

pub fn run_till_halt(values: &Vec<i32>, inputs: Vec<i32>) -> i32 {
    let mut computer = IntCodeComputer { numbers: values.clone(), index: 0, input_queue: inputs.into_iter().collect() };
    let mut final_output = 0;
    loop {
        match computer.run() {
            IntCodeResult::Output(value) => final_output = value,
            IntCodeResult::Halted => break
        }
    }
    final_output
}

fn run_day5(values: &Vec<i32>, input_value: i32) -> i32 {
    run_till_halt(values, vec![input_value])
}

pub fn part1(input: Input<Vec<i32>>) -> Answer<i32> {
    Answer { question: input.question, result: run_day5(&input.data, 1) }
}

pub fn part2(input: Input<Vec<i32>>) -> Answer<i32> {
    Answer { question: input.question, result: run_day5(&input.data, 5) }
}
