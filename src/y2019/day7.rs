use super::super::helpers::parser::*;
use super::day5::*;

fn combinations(current: Vec<i32>) -> Vec<Vec<i32>> {
    if current.len() == 1 {
        return vec![current; 1];
    } else {
        let mut result: Vec<Vec<i32>> = vec![];

        for i in 0..=current.len() - 1 {
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
            phase_setting = compute(&input.data, vec![phase, phase_setting]);
        }
        result = i32::max(result, phase_setting);
    }
    Answer { question: input.question, result }
}

pub fn part2(input: Input<Vec<i32>>) -> Answer<i32> {
    Answer { question: input.question, result: 0 }
}
