use super::super::helpers::parser::*;
use super::super_int_code_computer::*;
use std::collections::VecDeque;
use crate::y2019::day5::IntCodeComputer;

fn combinations(current: Vec<i128>) -> Vec<Vec<i128>> {
    if current.len() == 1 {
        return vec![current];
    } else {
        let mut result: Vec<Vec<i128>> = vec![];

        for i in 0..current.len() {
            let mut remain = current.clone();
            let removed_phase = remain[i];
            remain.remove(i);
            let next = combinations(remain.clone());
            for mut n in next {
                n.push(removed_phase);
                result.push(n)
            }
        }
        result
    }
}

pub fn part1(input: Input<Vec<i128>>) -> Answer<i128> {
    let possible_phases = combinations(vec![0, 1, 2, 3, 4]);

    let mut result: i128 = 0;
    for phases in possible_phases {
        let mut phase_setting = 0;
        for phase in phases {
            phase_setting = SuperIntCodeComputer::run_till_halt(&input.data, vec![phase, phase_setting]);
        }
        result = i128::max(result, phase_setting);
    }
    Answer { question: input.question, result }
}

pub fn part2(input: Input<Vec<i128>>) -> Answer<i128> {
    let possible_phases = combinations(vec![5, 6, 7, 8, 9]);

    let mut result = 0;

    for phases in possible_phases {
        let mut signal = 0;
        let mut index = 0;

        let mut computers: Vec<SuperIntCodeComputer> = phases.iter().map(|phase| {
            let input_queue: VecDeque<i128> = vec![*phase].into_iter().collect();
            let mut computer = SuperIntCodeComputer::new(input.data.clone());
            computer.input_queue = input_queue;
            computer
        }).collect();

        loop {
            computers[index].input_queue.push_back(signal);

            match computers[index].run() {
                SuperIntCodeResult::Output(val) => {
                    signal = val
                }
                SuperIntCodeResult::Halted => break,
            }

            index = (index + 1) % 5;
        }

        result = i128::max(result, signal);
    }
    Answer { question: input.question, result }
}