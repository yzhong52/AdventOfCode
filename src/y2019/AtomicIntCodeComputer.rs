use std::collections::{VecDeque, HashMap};
use std::thread::sleep;
use std::time::Duration;
use std::sync::Mutex;

const POSITION_MODE: i128 = 0;
const IMMEDIATE_MODE: i128 = 1;
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
    pub index: Mutex<usize>,
    pub instructions: Mutex<Vec<i128>>,
    pub inputs: Mutex<VecDeque<i128>>,
    pub relative_base: Mutex<usize>,
    pub external_storage: Mutex<HashMap<usize, i128>>,
}

impl AtomicIntCodeComputer {
    pub fn new(instructions: Vec<i128>) -> AtomicIntCodeComputer {
        AtomicIntCodeComputer {
            instructions: Mutex::new(instructions),
            index: Mutex::new(0),
            inputs: Mutex::new(VecDeque::new()),
            relative_base: Mutex::new(0),
            external_storage: Mutex::new(HashMap::new()),
        }
    }

    pub fn input(&self, value: i128) {
        let mut inputs = self.inputs.lock().unwrap();
        inputs.push_back(value);
    }

    fn parse_number(&self, index: usize, mode: i128, relative_base: usize) -> i128 {
        match mode {
            POSITION_MODE => self.read(self.read(index) as usize),
            IMMEDIATE_MODE => self.read(index),
            RELATIVE_MODE => {
                // self.read(self.index) can be negative, therefore we need to convert it to int first
                let position = self.read(index) + relative_base as i128;
                self.read(position as usize)
            }
            i => unimplemented!("{}", i),
        }
    }

    fn save_number(&self, mode: i128, position: i128, relative_base: usize, value: i128) {
        match mode {
            POSITION_MODE => self.save(position as usize, value),
            RELATIVE_MODE => {
                let relative_position = (position + relative_base as i128) as usize;
                self.save(relative_position, value)
            }
            i => unimplemented!("{}", i),
        }
    }

    fn read(&self, pos: usize) -> i128 {
        let default_value = 0 as i128;

        let instructions = self.instructions.lock().unwrap();
        let external_storage = self.external_storage.lock().unwrap();

        if pos < instructions.len() {
            instructions[pos]
        } else {
            *external_storage.get(&pos).unwrap_or_else({ || &default_value })
        }
    }

    fn save(&self, pos: usize, value: i128) {
        let mut instructions = self.instructions.lock().unwrap();
        let mut external_storage = self.external_storage.lock().unwrap();
        if pos < instructions.len() {
            instructions[pos] = value;
        } else {
            external_storage.insert(pos, value);
        }
    }

    pub fn run(&self) -> AtomicIntCodeResult {
        let mut index = self.index.lock().unwrap();
        let mut relative_base = self.relative_base.lock().unwrap();

        while self.read(*index as usize) != 99 {
            let current_instruction = self.read(*index as usize);
            let operation_code = current_instruction % 100;
            let mode1 = current_instruction / 100 % 10;
            let mode2 = current_instruction / 1000 % 10;
            let mode3 = current_instruction / 10000 % 10;
            *index += 1;

            match operation_code {
                OPERATION_ADDITION_1 | OPERATION_MULTIPLICATION_2 | OPERATION_LESS_THAN_7 | OPERATION_EQUAL_8 => {
                    let parameter1 = self.parse_number(*index, mode1, *relative_base);
                    *index += 1;
                    let parameter2 = self.parse_number(*index, mode2, *relative_base);
                    *index += 1;

                    let value = match operation_code {
                        OPERATION_ADDITION_1 => parameter1 + parameter2,
                        OPERATION_MULTIPLICATION_2 => parameter1 * parameter2,
                        OPERATION_LESS_THAN_7 => (parameter1 < parameter2) as i128,
                        OPERATION_EQUAL_8 => (parameter1 == parameter2) as i128,
                        i => unimplemented!("{}", i),
                    };

                    let parameter3 = self.read(*index);
                    *index += 1;

                    self.save_number(mode3, parameter3, *relative_base as usize, value);
                }
                OPERATION_INPUT_3 => {
                    let position = self.read(*index);
                    *index += 1;

                    let mut inputs = self.inputs.lock().unwrap();
                    if let Some(value) = inputs.pop_front() {
                        self.save_number(mode1, position, *relative_base as usize, value);
                    } else {
                        // TODO: Yuchen - maybe need to configure this as well
                        println!("Reading empty...");
                        self.save_number(mode1, position, *relative_base as usize, -1);

                        // Since we don't have semaphore in Rust, let's just sleep and switch thread for now.
                        let random_number: u64 = rand::random();
                        sleep(Duration::from_millis(random_number % 100 + 300));
                    }
                }
                OPERATION_OUTPUT_4 => {
                    let output_number = self.parse_number(*index, mode1, *relative_base);
                    *index += 1;
                    return AtomicIntCodeResult::Output(output_number);
                }
                OPERATION_JUMP_IF_TRUE_5 | OPERATION_JUMP_IF_FALSE_6 => {
                    let parameter1 = self.parse_number(*index, mode1, *relative_base);
                    *index += 1;
                    let parameter2 = self.parse_number(*index, mode2, *relative_base);
                    *index += 1;

                    *index = match operation_code {
                        OPERATION_JUMP_IF_TRUE_5 if parameter1 != 0 => parameter2 as usize,
                        OPERATION_JUMP_IF_FALSE_6 if parameter1 == 0 => parameter2 as usize,
                        _ => *index,
                    };
                }
                OPERATION_RELATIVE_BASE_OFFSET_9 => {
                    let parameter1 = self.parse_number(*index, mode1, *relative_base);
                    *index += 1;
                    *relative_base = (*relative_base as i128 + parameter1) as usize;
                }
                i => unimplemented!("{}", i),
            };
        }

        AtomicIntCodeResult::Halted
    }
}

