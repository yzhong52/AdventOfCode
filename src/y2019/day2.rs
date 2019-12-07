use super::super::helpers::parser::*;

fn compute(input: &Vec<i32>, noun: i32, verb: i32) -> i32 {
    let mut numbers = input.clone();
    let mut index: usize = 0;
    numbers[1] = noun;
    numbers[2] = verb;

    while numbers[index] != 99 {
        let operator = numbers[index];
        let number1 = numbers[numbers[index as usize + 1] as usize];
        let number2 = numbers[numbers[index + 2] as usize];
        let result_index = numbers[index + 3] as usize;
        numbers[result_index] = match operator {
            1 => number1 + number2,
            2 => number1 * number2,
            _ => unimplemented!(),
        };
        index += 4;
    }

    numbers[0]
}

pub fn part1(input: Input<Vec<i32>>) -> Answer<i32> {
    Answer { question: input.question, result: compute(&input.data, 12, 2) }
}

pub fn part2(input: Input<Vec<i32>>) -> Answer<i32> {
    for noun in 0..100 {
        for verb in 0..100 {
            if compute(&input.data, noun, verb) == 19690720 {
                return Answer { question: input.question, result: 100 * noun + verb };
            }
        }
    }
    Answer { question: input.question, result: 9999 }
}
