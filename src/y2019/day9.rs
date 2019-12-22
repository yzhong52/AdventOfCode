use super::super::helpers::parser::*;

use std::collections::{VecDeque, HashMap};

const POSITION_MODE: i128 = 0;
const IMMEDIATE_MODE: i128 = 1;
// Parameters in mode 2, relative mode, behave very similarly to parameters in position mode: the
// parameter is interpreted as a position. Like position mode, parameters in relative mode can be
// read from or written to.
const RELATIVE_MODE: i128 = 2;

const OPERATION_ADDITION_1: i128 = 1;
const OPERATION_MULTIPLICATION_2: i128 = 2;
const OPERATION_INPUT_3: i128 = 3;
const OPERATION_OUTPUT_4: i128 = 4;
const OPERATION_JUMP_IF_TRUE_5: i128 = 5;
const OPERATION_JUMP_IF_FALSE_6: i128 = 6;
const OPERATION_LESS_THAN_7: i128 = 7;
const OPERATION_EQUAL_8: i128 = 8;
const OPERATION_RELATIVE_BASE_OFFSET_9: i128 = 9;

pub enum SuperIntCodeResult {
    Output(i128),
    Halted,
}

#[derive(Clone)]
pub struct SuperIntCodeComputer {
    pub index: usize,
    pub program: Vec<i128>,
    pub input_queue: VecDeque<i128>,
    pub relative_base: usize,
    pub external_numbers: HashMap<usize, i128>,
}

impl SuperIntCodeComputer {
    pub fn new(numbers: Vec<i128>) -> SuperIntCodeComputer {
        SuperIntCodeComputer {
            program: numbers,
            index: 0,
            input_queue: VecDeque::new(),
            relative_base: 0,
            external_numbers: HashMap::new(),
        }
    }

    pub fn input(&mut self, value: i128) {
        self.input_queue.push_back(value);
    }

    fn parse_number(&self, mode: i128, relative_base: usize) -> i128 {
        match mode {
            POSITION_MODE => self.read(self.read(self.index) as usize),
            IMMEDIATE_MODE => self.read(self.index as usize),
            RELATIVE_MODE => {
                // self.read(self.index) can be negative, therefore we need to convert it to int first
                let position = self.read(self.index) + relative_base as i128;
                self.read(position as usize)
            }
            i => unimplemented!("{}", i),
        }
    }

    fn save_number(&mut self, mode: i128, position: i128, value: i128) {
        match mode {
            POSITION_MODE => self.save(position as usize, value),
            RELATIVE_MODE => {
                let relative_position = (position + self.relative_base as i128) as usize;
                self.save(relative_position, value)
            }
            i => unimplemented!("{}", i),
        }
    }

    fn read(&self, pos: usize) -> i128 {
        let default_value = 0 as i128;
        if pos < self.program.len() {
            self.program[pos]
        } else {
            *self.external_numbers.get(&pos).unwrap_or_else({ || &default_value })
        }
    }

    fn save(&mut self, pos: usize, value: i128) {
        if pos < self.program.len() {
            self.program[pos] = value;
        } else {
            self.external_numbers.insert(pos, value);
        }
    }

    pub fn run(&mut self) -> SuperIntCodeResult {
        while self.read(self.index) != 99 {
            let current_instruction = self.read(self.index);
            let operation_code = current_instruction % 100;
            let mode1 = current_instruction / 100 % 10;
            let mode2 = current_instruction / 1000 % 10;
            let mode3 = current_instruction / 10000 % 10;
            self.index += 1;

            match operation_code {
                OPERATION_ADDITION_1 | OPERATION_MULTIPLICATION_2 | OPERATION_LESS_THAN_7 | OPERATION_EQUAL_8 => {
                    let parameter1 = self.parse_number(mode1, self.relative_base);
                    self.index += 1;
                    let parameter2 = self.parse_number(mode2, self.relative_base);
                    self.index += 1;

                    let value = match operation_code {
                        OPERATION_ADDITION_1 => parameter1 + parameter2,
                        OPERATION_MULTIPLICATION_2 => parameter1 * parameter2,
                        OPERATION_LESS_THAN_7 => (parameter1 < parameter2) as i128,
                        OPERATION_EQUAL_8 => (parameter1 == parameter2) as i128,
                        i => unimplemented!("{}", i),
                    };

                    let parameter3 = self.read(self.index);
                    self.index += 1;

                    self.save_number(mode3, parameter3, value);
                }
                OPERATION_INPUT_3 => {
                    let position = self.read(self.index);
                    self.index += 1;

                    let value = self.input_queue.pop_front().unwrap();
                    self.save_number(mode1, position, value);
                }
                OPERATION_OUTPUT_4 => {
                    let output_number = self.parse_number(mode1, self.relative_base);
                    self.index += 1;
                    return SuperIntCodeResult::Output(output_number);
                }
                OPERATION_JUMP_IF_TRUE_5 | OPERATION_JUMP_IF_FALSE_6 => {
                    let parameter1 = self.parse_number(mode1, self.relative_base);
                    self.index += 1;
                    let parameter2 = self.parse_number(mode2, self.relative_base);
                    self.index += 1;

                    self.index = match operation_code {
                        OPERATION_JUMP_IF_TRUE_5 if parameter1 != 0 => parameter2 as usize,
                        OPERATION_JUMP_IF_FALSE_6 if parameter1 == 0 => parameter2 as usize,
                        _ => self.index,
                    };
                }
                OPERATION_RELATIVE_BASE_OFFSET_9 => {
                    // let parameter1 = self.read(self.index);
                    let parameter1 = self.parse_number(mode1, self.relative_base);
                    self.index += 1;
                    self.relative_base = (self.relative_base as i128 + parameter1) as usize;
                }
                i => unimplemented!("{}", i),
            };
        }

        SuperIntCodeResult::Halted
    }
}

pub fn run_till_halt(values: &Vec<i128>, inputs: Vec<i128>) -> i128 {
    let mut computer = SuperIntCodeComputer {
        program: values.clone(),
        index: 0,
        input_queue: inputs.into_iter().collect(),
        relative_base: 0,
        external_numbers: HashMap::new(),
    };
    let mut final_output = 0;
    loop {
        match computer.run() {
            SuperIntCodeResult::Output(value) => {
                println!("Output value {}", value);
                final_output = value
            }
            SuperIntCodeResult::Halted => break
        }
    }
    final_output
}

fn run_day9(values: &Vec<i128>, input_value: i128) -> i128 {
    run_till_halt(values, vec![input_value])
}

pub fn part1(input: Input<Vec<i128>>) -> Answer<i128> {
    Answer { question: input.question, result: run_day9(&input.data, 1) }
}

pub fn part2(input: Input<Vec<i128>>) -> Answer<i128> {
    Answer { question: input.question, result: run_day9(&input.data, 2) }
}
