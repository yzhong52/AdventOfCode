use crate::helpers::parser::{Input, Answer, parse_numbers_by_comma};
use crate::int_code_computers::super_int_code_computer::SuperIntCodeComputer;
use crate::int_code_computers::super_int_code_computer::SuperIntCodeResult;
use crate::helpers::puzzle::Puzzle;

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


pub struct Day21 {}

impl Puzzle<Vec<i128>, i128> for Day21 {
    fn day(&self) -> i8 {
        21
    }

    fn parser(&self) -> fn(String) -> Vec<i128> {
        parse_numbers_by_comma
    }

    fn part1(&self, input: &Vec<i128>) -> i128 {
        let mut droid = Springdroid::new(input.clone());
        let script = r#"
        NOT C T
        AND D T
        OR T J
        NOT A T
        OR T J
        WALK
        "#;
        let output = droid.run(script);
        output
    }


    // Inspired by:
    // * https://www.reddit.com/r/adventofcode/comments/edll5a/2019_day_21_solutions/fbinci1/?context=3
    //
    //  - "Part 1: jump if there's a hole in front of me and the landing spot is ground"
    //  - "Part 2: "jump if part 1 AND I can either jump or walk forward from the landing spot successfully"
    fn part2(&self, input: &Vec<i128>) -> i128 {
        let mut droid = Springdroid::new(input.clone());
        // Jump = !A
        // ......@..........
        // .....@.@.........
        // ....@...@........
        // #####.###########
        //      ABCDEFGH
        //
        // Jump = !B & !E = !(B | E)
        // .....@...@.......
        // ....@.@.@.@......
        // ...@...@...@.....
        // #####.?#.########
        //     ABCDEFGH
        //
        // Jump = !C & !E = !(C | E)
        // ....@...@.........
        // ...@.@.@.@........
        // ..@...@...@.......
        // #####.#..########
        //    ABCDEFGH
        //
        // Jump = !C & !I & !F = !(C | I | F)
        // ....@...@...@....
        // ...@.@.@.@.@.@...
        // ..@...@...@...@..
        // #####.##.##..####
        //    ABCDEFGHI
        //
        // Jump = !C & D & E & F
        // ....@....@.......
        // ...@.@..@.@......
        // .@@...@@...@.....
        // #####.###..#.####
        //    ABCDEFGHI
        //
        // Don't Jump = D & !E & !H
        // ......@...@......
        // .....@.@.@.@.....
        // ...@@...@...@....
        // #####.#.##.######
        //    ABCDEFGH

        let script = r#"
        OR A T
        AND B T
        AND C T
        NOT T T
        AND D T
        OR T J

        AND E T
        OR H T
        AND T J

        RUN
        "#;
        let output = droid.run(script);
        output
    }
}
