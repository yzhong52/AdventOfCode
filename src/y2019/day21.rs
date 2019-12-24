use crate::helpers::parser::{Input, Answer};
use crate::y2019::day9::{SuperIntCodeComputer, SuperIntCodeResult};

struct Springdroid {
    computer: SuperIntCodeComputer,
}

impl Springdroid {
    fn new(instructions: Vec<i128>) -> Springdroid {
        Springdroid { computer: SuperIntCodeComputer::new(instructions) }
    }

    fn run(&mut self, script: &str) -> i128 {
        let s: String = script.to_string()
            .split('\n')
            .map(|x| x.trim().to_string())
            .filter(|x| x.len() > 0)
            .collect::<Vec<String>>()
            .join("\n");

        for c in s.chars() {
            self.computer.input(c as i128);
        }
        self.computer.input('\n' as i128); // That's why we need "EOF"

        let mut output = 0;
        loop {
            match self.computer.run() {
                SuperIntCodeResult::Output(value) => {
                    if value < 128 {
                        print!("{}", value as u8 as char);
                    } else {
                        output = value;
                    }
                }
                SuperIntCodeResult::Halted => break,
            }
        }
        output
    }
}


pub fn part1(input: Input<Vec<i128>>) -> Answer<i128> {
    let mut droid = Springdroid::new(input.data.clone());
    let script = r#"
        NOT C T
        AND D T
        OR T J
        NOT A T
        OR T J
        WALK
        "#;
    let output = droid.run(script);

    Answer { question: input.question, result: output }
}

pub fn part2(input: Input<Vec<i128>>) -> Answer<i128> {
    let mut droid = Springdroid::new(input.data.clone());
    // J = !A
    // ......@..........
    // .....@.@.........
    // ....@...@........
    // #####.###########
    //      ABCDEFGH
    //
    // J = !B & !E = !(B | E)
    // .....@...@.......
    // ....@.@.@.@......
    // ...@...@...@.....
    // #####.?#.########
    //     ABCDEFGH
    //
    // J = !C & !E = !(C | E)
    // ....@...@.........
    // ...@.@.@.@........
    // ..@...@...@.......
    // #####.#..########
    //    ABCDEFGH
    //
    // J = !C & !I & !F = !(C | I | F)
    // ....@...@...@....
    // ...@.@.@.@.@.@...
    // ..@...@...@...@..
    // #####.##.##..####
    //    ABCDEFGHI
    //
    // J = !C & D & E & F
    // ....@....@.......
    // ...@.@..@.@......
    // ..@...@@...@.....
    // #####.###..#.####
    //    ABCDEFGHI

    let script = r#"
        OR F T
        OR C T
        AND B T
        OR E T
        AND A T
        NOT T J

        NOT I T
        AND H T
        AND D T
        OR T J

        RUN
        "#;
    let output = droid.run(script);

    Answer { question: input.question, result: output }
}
