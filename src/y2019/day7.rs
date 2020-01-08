use super::super::helpers::parser::*;
use std::collections::VecDeque;
use crate::int_code_computers::basic_int_code_computer::{IntCodeComputer, IntCodeResult};
use crate::helpers::puzzle::Puzzle;


pub struct Day7 {}

impl Day7 {
    fn combinations(current: Vec<i32>) -> Vec<Vec<i32>> {
        if current.len() == 1 {
            return vec![current];
        } else {
            let mut result = vec![];

            for i in 0..current.len() {
                let mut remain = current.clone();
                let removed_phase = remain[i];
                remain.remove(i);
                let next = Day7::combinations(remain.clone());
                for mut n in next {
                    n.push(removed_phase);
                    result.push(n)
                }
            }
            result
        }
    }
}

impl Puzzle<Vec<i32>, i32> for Day7 {
    fn day(&self) -> i8 {
        7
    }

    fn parser(&self) -> fn(String) -> Vec<i32> {
        parse_numbers_by_comma
    }

    fn part1(&self, input: &Vec<i32>) -> i32 {
        let possible_phases = Day7::combinations(vec![0, 1, 2, 3, 4]);

        let mut result = 0;
        for phases in possible_phases {
            let mut phase_setting = 0;
            for phase in phases {
                phase_setting = IntCodeComputer::run_till_halt(&input, vec![phase, phase_setting]);
            }
            result = i32::max(result, phase_setting);
        }
        result
    }

    fn part2(&self, input: &Vec<i32>) -> i32 {
        let possible_phases = Day7::combinations(vec![5, 6, 7, 8, 9]);

        let mut result = 0;

        for phases in possible_phases {
            let mut signal = 0;
            let mut index = 0;

            let mut computers: Vec<IntCodeComputer> = phases.iter().map(|phase| {
                let input_queue: VecDeque<i32> = vec![*phase].into_iter().collect();
                IntCodeComputer {
                    index,
                    numbers: input.clone(),
                    input_queue,
                    relative_base: 0,
                }
            }).collect();

            loop {
                computers[index].input_queue.push_back(signal);

                match computers[index].run() {
                    IntCodeResult::Output(val) => {
                        signal = val
                    }
                    IntCodeResult::Halted => break,
                }

                index = (index + 1) % 5;
            }

            result = i32::max(result, signal);
        }
        result
    }
}
