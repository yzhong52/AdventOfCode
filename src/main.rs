use std::env;

mod helpers {
    pub mod models;
    pub mod parser;
    pub mod utils;
}

mod y2018 {
    pub mod day1;
    pub mod day2;
    pub mod day3;
    pub mod day4;
    pub mod day5;
    pub mod day6;
    pub mod day7;
    pub mod day8;
    pub mod day9;
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
}

use helpers::parser::*;

fn run_completed() {
    // ### 2018 ###

    y2018::day1::part1(read_numbers_by_line(Question::y2018(1))).save_part1();
    y2018::day1::part2(read_numbers_by_line(Question::y2018(1))).save_part2();

    y2018::day2::part1(read_strings(Question::y2018(2))).save_part1();
    y2018::day2::part2(read_strings(Question::y2018(2))).save_part2();

    y2018::day3::part1(read_strings(Question::y2018(3))).save_part1();
    y2018::day3::part2(read_strings(Question::y2018(3))).save_part2();

    y2018::day4::part1(read_strings(Question::y2018(4))).save_part1();
    y2018::day4::part2(read_strings(Question::y2018(4))).save_part2();

    y2018::day5::part1(read_single_string(Question::y2018(5))).save_part1();
    y2018::day5::part2(read_single_string(Question::y2018(5))).save_part2();

    y2018::day6::part1(read_points(Question::y2018(6))).save_part1();
    y2018::day6::part2(read_points(Question::y2018(6))).save_part2();

    y2018::day7::part1(read_strings(Question::y2018(7))).save_part1();
    y2018::day7::part2(read_strings(Question::y2018(7))).save_part2();

    y2018::day8::part1(read_numbers_space(Question::y2018(8))).save_part1();
    y2018::day8::part2(read_numbers_space(Question::y2018(8))).save_part2();

    y2018::day9::part1(Input { question: Question::y2018(9), data: (428, 70825) }).save_part1(); // TODO: Yuchen -
    y2018::day9::part2(Input { question: Question::y2018(9), data: (428, 7082500) }).save_part2(); // TODO: Yuchen -

    // ### 2019 ###

    y2019::day1::part1(read_numbers_by_line(Question::y2019(1))).save_part1();
    y2019::day1::part2(read_numbers_by_line(Question::y2019(1))).save_part2();

    y2019::day2::part1(read_numbers_by_comma(Question::y2019(2))).save_part1();
    y2019::day2::part2(read_numbers_by_comma(Question::y2019(2))).save_part2();

    y2019::day3::part1(&read_strings(Question::y2019(3))).save_part1();
    y2019::day3::part2(&read_strings(Question::y2019(3))).save_part2();

    y2019::day4::part1(Input { question: Question::y2019(4), data: 265275..=781584 }).save_part1();
    y2019::day4::part2(Input { question: Question::y2019(4), data: 265275..=781584 }).save_part2();

    y2019::day5::part1(read_numbers_by_comma(Question::y2019(5))).save_part1();
    y2019::day5::part2(read_numbers_by_comma(Question::y2019(5))).save_part2();

    y2019::day6::part1(read_strings(Question::y2019(6))).save_part1();
    y2019::day6::part2(read_strings(Question::y2019(6))).save_part2();

    y2019::day7::part1(read_numbers_by_comma(Question::y2019(7))).save_part1();
    y2019::day7::part2(read_numbers_by_comma(Question::y2019(7))).save_part2();

    y2019::day8::part1(read_single_string(Question::y2019(8))).save_part1();
    y2019::day8::part2(read_single_string(Question::y2019(8))).save_part2();

    // Day 9: Sensor Boost
    y2019::day9::part1(read_numbers_by_comma(Question::y2019(9))).save_part1();
    y2019::day9::part2(read_numbers_by_comma(Question::y2019(9))).save_part2();

    // Day 10: Monitoring Station
    y2019::day10::part1(read_grid(Question::y2019(10))).save_part1();
    y2019::day10::part2(read_grid(Question::y2019(10))).save_part2();

    // Day 11: Space Police
    y2019::day11::part1(read_numbers_by_comma(Question::y2019(11))).save_part1();
    y2019::day11::part2(read_numbers_by_comma(Question::y2019(11))).save_part2();

    // Day 12: The N-Body Problem
    y2019::day12::part1(read_numbers_by_line(Question::y2019(12))).save_part1();
    y2019::day12::part2(read_numbers_by_line(Question::y2019(12))).save_part2();

    // Day 13: Space Police
    y2019::day13::part1(read_numbers_by_comma(Question::y2019(13))).save_part1();
    y2019::day13::part2(read_numbers_by_comma(Question::y2019(13))).save_part2();

    // Day 14: Space Stoichiometry
    y2019::day14::part1(read_strings(Question::y2019(14))).save_part1();
    y2019::day14::part2(read_strings(Question::y2019(14))).save_part2();

    // Day 15: Oxygen System
    y2019::day15::part1(read_numbers_by_comma(Question::y2019(15))).save_part1();
    y2019::day15::part2(read_numbers_by_comma(Question::y2019(15))).save_part2();

    // Day 16: Flawed Frequency Transmission
    y2019::day16::part1(read_single_string(Question::y2019(16))).save_part1();
    // TODO: Yuchen - part 2
    y2019::day16::part2(read_single_string(Question::y2019(16))).save_part2();

    // Day 17: Set and Forget
    y2019::day17::part1(read_numbers_by_comma(Question::y2019(17))).save_part1();
    y2019::day17::part2(read_numbers_by_comma(Question::y2019(17))).save_part2();

    // Day 18: Many-Worlds Interpretation
    y2019::day18::part1(read_grid(Question::y2019(18))).save_part1();
    y2019::day18::part2(read_grid(Question::y2019(18))).save_part2();

    // Day 19: Tractor Beam
    y2019::day19::part1(read_numbers_by_comma(Question::y2019(19))).save_part1();
    y2019::day19::part2(read_numbers_by_comma(Question::y2019(19))).save_part2();

    // Day 20: Donut Maze
    y2019::day20::part1(read_grid(Question::y2019(20))).save_part1();
    y2019::day20::part2(read_grid(Question::y2019(20))).save_part2();

    // Day 21: Springdroid Adventure
    y2019::day21::part1(read_numbers_by_comma(Question::y2019(21))).save_part1();

    // Day 22: Slam Shuffle
    y2019::day22::part1(read_strings(Question::y2019(22))).save_part1();
}

fn main() {
    for arg in env::args() {
        match arg {
            s if s == "run_all".to_string() => run_completed(),
            _ => ()
        };
    }

    // TODO: Yuchen -
    // y2019::day21::part2(read_numbers_by_comma(Question::y2019(21))).save_part2();
    // TODO: Yuchen -
    // y2019::day22::part2(read_strings(Question::y2019(22))).save_part2();
}
