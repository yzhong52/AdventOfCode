use super::super::helpers::parser::*;
use super::day5::*;
use std::collections::VecDeque;

fn combinations(current: Vec<i32>) -> Vec<Vec<i32>> {
    if current.len() == 1 {
        return vec![current];
    } else {
        let mut result: Vec<Vec<i32>> = vec![];

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

pub fn part1(input: Input<Vec<i32>>) -> Answer<i32> {
    let possible_phases = combinations(vec![0, 1, 2, 3, 4]);

    let mut result = 0;
    for phases in possible_phases {
        let mut phase_setting = 0;
        for phase in phases {
            phase_setting = run_till_halt(&input.data, vec![phase, phase_setting]);
        }
        result = i32::max(result, phase_setting);
    }
    Answer { question: input.question, result }
}

pub fn part2(input: Input<Vec<i32>>) -> Answer<i32> {
    let possible_phases = combinations(vec![5, 6, 7, 8, 9]);

    let mut result = 0;

    for phases in possible_phases {
        let mut signal = 0;
        let mut index = 0;

        let mut computers: Vec<IntCodeComputer> = phases.iter().map(|phase| {
            let input_queue = vec![*phase].into_iter().collect();
            IntCodeComputer { numbers: input.data.clone(), index: 0, input_queue }
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
    Answer { question: input.question, result }
}
