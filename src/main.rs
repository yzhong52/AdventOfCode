mod helpers {
    pub mod models;
    pub mod parser;
}

mod day1;
mod day2;
mod day4;
mod day5;
mod day6;
mod day7;

use helpers::parser::*;


fn main() {
    day1::part1(read_ints(Question { year: 2018, day: 1 })).save_part1();
    day1::part2(read_ints(Question { year: 2018, day: 1 })).save_part2();

    day2::part1(read_strings(Question { year: 2018, day: 2 })).save_part1();

    day4::part1(read_strings(Question { year: 2018, day: 4 })).save_part1();
    day4::part2(read_strings(Question { year: 2018, day: 4 })).save_part2();

    day5::part1(read_string(Question { year: 2018, day: 5 })).save_part1();
    day5::part2(read_string(Question { year: 2018, day: 5 })).save_part2();

    day6::part1(read_points(Question { year: 2018, day: 6 })).save_part1();
    day6::part2(read_points(Question { year: 2018, day: 6 })).save_part2();

    day7::part1(read_strings(Question { year: 2018, day: 7 })).save_part1();
    day7::part2(read_strings(Question { year: 2018, day: 7 })).save_part2();
}
