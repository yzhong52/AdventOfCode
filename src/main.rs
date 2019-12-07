use std::env;

mod helpers {
    pub mod models;
    pub mod parser;
}

mod y2018 {
    pub mod day1;
    pub mod day2;
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
}

use helpers::parser::*;

fn run_completed() {
    y2018::day1::part1(read_ints(Question::y2018(1))).save_part1();
    y2018::day1::part2(read_ints(Question::y2018(1))).save_part2();

    y2018::day2::part1(read_strings(Question::y2018(2))).save_part1();
    // TODO: Yuchen - day 2 part 2

    y2018::day4::part1(read_strings(Question::y2018(4))).save_part1();
    y2018::day4::part2(read_strings(Question::y2018(4))).save_part2();

    y2018::day5::part1(read_string(Question::y2018(5))).save_part1();
    y2018::day5::part2(read_string(Question::y2018(5))).save_part2();

    y2018::day6::part1(read_points(Question::y2018(6))).save_part1();
    y2018::day6::part2(read_points(Question::y2018(6))).save_part2();

    y2018::day7::part1(read_strings(Question::y2018(7))).save_part1();
    y2018::day7::part2(read_strings(Question::y2018(7))).save_part2();

    y2018::day8::part1(read_ints_by_space(Question::y2018(8))).save_part1();
    y2018::day8::part2(read_ints_by_space(Question::y2018(8))).save_part2();

    let y2018_day9_input = Input {
        question: Question::y2018(9),
        data: y2018::day9::Day9 { players: 9, last_marble: 25 },
    };
    y2018::day9::part1(y2018_day9_input).save_part1();
    // TODO: day 9

    y2019::day1::part1(read_ints_by_comma(Question::y2019(1))).save_part1();
    y2019::day1::part2(read_ints_by_comma(Question::y2019(1))).save_part2();

    y2019::day2::part1(read_ints_by_comma(Question::y2019(2))).save_part1();
    y2019::day2::part2(read_ints_by_comma(Question::y2019(2))).save_part2();

    y2019::day3::part1(&read_strings(Question::y2019(3))).save_part1();
    y2019::day3::part2(&read_strings(Question::y2019(3))).save_part2();

    y2019::day4::part1(Input { question: Question::y2019(4), data: 265275..=781584 }).save_part1();
    y2019::day4::part2(Input { question: Question::y2019(4), data: 265275..=781584 }).save_part2();
}

fn main() {
    match env::args().next() {
        Some(arg) if arg == "run_all".to_string() => run_completed(),
        _ => ()
    };
    y2019::day5::part1(read_ints_by(Question::y2019(5), ',')).save_part1();
    y2019::day5::part2(read_ints_by(Question::y2019(5), ',')).save_part2();
}
