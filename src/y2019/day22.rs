use crate::helpers::parser::Answer;
use crate::helpers::parser::Input;

pub fn part1(input: Input<Vec<String>>) -> Answer<i32> {
    let deck: Vec<i32> = (0..10006).collect();
    for row in input.data {
        println!("{}", row);
    }
    Answer { question: input.question, result: 0 }
}

pub fn part2(input: Input<Vec<String>>) -> Answer<i32> {
    Answer { question: input.question, result: 0 }
}
