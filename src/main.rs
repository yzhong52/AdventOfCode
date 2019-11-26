mod helpers {
    pub mod models;
    pub mod parser;
}

mod day1;
mod day2;
mod day4;
mod day5;
mod day6;

use helpers::parser::*;


fn main() {
    day1::part1(read_ints(Question { year: 2018, day: 1 })).save_as("part1");
    day1::part2(read_ints(Question { year: 2018, day: 1 })).save_as("part2");
    day2::day2(read_strings(Question { year: 2018, day: 2 })).save();
    day4::part1(read_strings(Question { year: 2018, day: 4 })).save_as("part1");

    day5::part1(read_string(Question { year: 2018, day: 5 })).save_as("part1");
    day5::part2(read_string(Question { year: 2018, day: 5 })).save_as("part2");

    day6::part1(read_points(Question { year: 2018, day: 6 })).save_as("part1");
}
