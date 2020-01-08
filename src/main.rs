use std::env;

mod helpers {
    pub mod models;
    pub mod parser;
    pub mod puzzle;
    pub mod utils;
}

mod int_code_computers {
    pub mod basic_int_code_computer;
    pub mod super_int_code_computer;
    pub mod atomic_int_code_computer;
}

mod y2019 {
    pub mod day1;
    pub mod day2;
    pub mod day3;
    pub mod day4;
    pub mod day5;
    pub mod day6;
    pub mod day7;
    pub mod day8;
    pub mod day9;
    pub mod day10;
    pub mod day11;
    pub mod day12;
    pub mod day13;
    pub mod day14;
    pub mod day15;
    pub mod day16;
    pub mod day17;
    pub mod day18;
    pub mod day19;
    pub mod day20;
    pub mod day21;
    pub mod day22;
    pub mod day23;
    pub mod day24;
    pub mod day25;
}

use helpers::parser::*;
use crate::helpers::puzzle::Puzzle;
use std::time::Instant;
use crate::y2019::day1::Day1;
use crate::y2019::day2::Day2;
use crate::y2019::day3::Day3;
use crate::y2019::day4::Day4;
use crate::y2019::day5::Day5;
use crate::y2019::day6::Day6;
use crate::y2019::day7::Day7;
use crate::y2019::day8::Day8;
use crate::y2019::day9::Day9;
use crate::y2019::day10::Day10;
use crate::y2019::day11::Day11;
use crate::y2019::day24::Day24;
use crate::y2019::day20::Day20;
use crate::y2019::day18::Day18;
use crate::y2019::day13::Day13;
use crate::y2019::day15::Day15;
use crate::y2019::day17::Day17;
use crate::y2019::day21::Day21;
use crate::y2019::day19::Day19;
use crate::y2019::day16::Day16;
use crate::y2019::day23::Day23;
use crate::y2019::day12::Day12;
use crate::y2019::day14::Day14;
use crate::y2019::day22::Day22;

fn run_completed() {}

fn main() {
    let start = Instant::now();
    for arg in env::args().skip(1) {
        match arg {
            s if s == "run_all".to_string() => run_completed(),
            _ => ()
        };
    }

    // [dyn Fn()](https://stackoverflow.com/questions/39083375/expected-closure-found-a-different-closure)
    let mut runnables: Vec<Box<dyn Fn()>> = Vec::new();
    runnables.push(Box::new(|| { Day1 {}.run() }));
    runnables.push(Box::new(|| { Day2 {}.run() }));
    runnables.push(Box::new(|| { Day3 {}.run() }));
    // Day 4: Secure Container
    runnables.push(Box::new(|| { Day4 {}.run() }));
    // Day 5: Sunny with a Chance of Asteroids
    runnables.push(Box::new(|| { Day5 {}.run() }));
    // Day 6: Universal Orbit Map
    runnables.push(Box::new(|| { Day6 {}.run() }));
    // Day 7: Amplification Circuit
    runnables.push(Box::new(|| { Day7 {}.run() }));
    // Day 8: Space Image Format
    runnables.push(Box::new(|| { Day8 {}.run() }));
    // Day 9: Sensor Boost
    runnables.push(Box::new(|| { Day9 {}.run() }));
    // Day 10: Monitoring Station
    runnables.push(Box::new(|| { Day10 {}.run() }));
    // Day 11: Space Police
    runnables.push(Box::new(|| { Day11 {}.run() }));
    // Day 12: The N-Body Problem
    runnables.push(Box::new(|| { Day12 {}.run() }));
    // Day 13: Care Package
    runnables.push(Box::new(|| { Day13 {}.run() }));
    // Day 14: Space Stoichiometry
    runnables.push(Box::new(|| { Day14 {}.run() }));
    // Day 15: Oxygen System
    runnables.push(Box::new(|| { Day15 {}.run() }));
    // Day 16: Flawed Frequency Transmission
    runnables.push(Box::new(|| { Day16 {}.run() }));
    // Day 17: Set and Forget
    runnables.push(Box::new(|| { Day17 {}.run() }));
    // Day 18: Many-Worlds Interpretation
    runnables.push(Box::new(|| { Day18 {}.run() }));
    // Day 19: Tractor Beam
    runnables.push(Box::new(|| { Day19 {}.run() }));
    // Day 20: Donut Maze
    runnables.push(Box::new(|| { Day20 {}.run() }));
    // Day 21: Springdroid Adventure
    runnables.push(Box::new(|| { Day21 {}.run() }));
    // Day 22: Slam Shuffle
    runnables.push(Box::new(|| { Day22 {}.run() }));
    // Day 23: Category Six
    runnables.push(Box::new(|| { Day23 {}.run() }));
    // Day 24: Planet of Discord
    runnables.push(Box::new(|| { Day24 {}.run() }));
    // Day 25
    runnables.push(Box::new(|| { Day24 {}.run() }));


    for puzzle in runnables {
        puzzle.as_ref()();
    }

    println!("Finish running: {:?}", start.elapsed());
}
