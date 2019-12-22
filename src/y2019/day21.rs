use crate::helpers::parser::{Input, Answer};
use crate::y2019::day9::{SuperIntCodeComputer, SuperIntCodeResult};

pub fn part1(input: Input<Vec<i128>>) -> Answer<i128> {
    let mut springdroid = SuperIntCodeComputer::new(input.data.clone());

    // 73
    let springscript = [
        "NOT C T",
        "AND D T",
        "OR T J",
        "NOT A T",
        "OR T J",
        "WALK",
    ];

    for script in springscript.iter() {
        for c in script.chars() {
            springdroid.input(c as u8 as i128);
        }
        springdroid.input('\n' as u8 as i128);
    }

    loop {
        match springdroid.run() {
            SuperIntCodeResult::Output(value) => {
                if value < 128 {
                    print!("{}'", value as u8 as char);
                } else {
                    print!("{}'", value);
                }
            },
            SuperIntCodeResult::Halted => break,
        }
    }


    Answer { question: input.question, result: 0 }
}

pub fn part2(input: Input<Vec<i128>>) -> Answer<i128> {
    Answer { question: input.question, result: 0 }
}
