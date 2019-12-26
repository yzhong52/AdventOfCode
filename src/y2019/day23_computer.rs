use super::super::helpers::parser::*;

use std::collections::{VecDeque, HashMap};
use tokio::sync::Semaphore;
use std::thread::sleep;
use std::time::Duration;
use std::sync::Mutex;

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

pub enum AtomicIntCodeResult {
    Output(i128),
    Halted,
}

pub struct AtomicIntCodeComputer {
    pub index: usize,
    pub instructions: Vec<i128>,
    pub input_queue: VecDeque<i128>,
    pub inputs: Mutex<VecDeque<i128>>,
    pub relative_base: usize,
    pub external_numbers: HashMap<usize, i128>,
    semaphore: Semaphore,
}

impl AtomicIntCodeComputer {
    pub fn new(instructions: Vec<i128>) -> AtomicIntCodeComputer {
        AtomicIntCodeComputer {
            instructions,
            index: 0,
            input_queue: VecDeque::new(),
            inputs: Mutex::new(VecDeque::new()),
            relative_base: 0,
            external_numbers: HashMap::new(),
            semaphore: Semaphore::new(0),
        }
    }

    pub fn input(&self, value: i128) {
        let mut inputs = self.inputs.lock().unwrap();
        inputs.push_back(value);
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
        if pos < self.instructions.len() {
            self.instructions[pos]
        } else {
            *self.external_numbers.get(&pos).unwrap_or_else({ || &default_value })
        }
    }

    fn save(&mut self, pos: usize, value: i128) {
        if pos < self.instructions.len() {
            self.instructions[pos] = value;
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

                    if let Some(value) = self.input_queue.pop_front() {
                        self.save_number(mode1, position, value);
                    } else {
                        // TODO: Yuchen - maybe need to configure this as well
                        println!("Reading empty...");
                        self.save_number(mode1, position, -1);

                        let random_number: u64 = rand::random();
                        sleep(Duration::from_millis(random_number % 100));
                    }
                }
                OPERATION_OUTPUT_4 => {
                    let output_number = self.parse_number(mode1, self.relative_base);
                    self.index += 1;
                    return AtomicIntCodeResult::Output(output_number);
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

        AtomicIntCodeResult::Halted
    }
}

