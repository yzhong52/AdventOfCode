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
}

mod y2019 {
    pub mod day1;
}

use helpers::parser::*;


fn main() {
    y2018::day1::part1(read_ints(Question { year: 2018, day: 1 })).save_part1();
    y2018::day1::part2(read_ints(Question { year: 2018, day: 1 })).save_part2();

    y2018::day2::part1(read_strings(Question { year: 2018, day: 2 })).save_part1();

    y2018::day4::part1(read_strings(Question { year: 2018, day: 4 })).save_part1();
    y2018::day4::part2(read_strings(Question { year: 2018, day: 4 })).save_part2();

    y2018::day5::part1(read_string(Question { year: 2018, day: 5 })).save_part1();
    y2018::day5::part2(read_string(Question { year: 2018, day: 5 })).save_part2();

    y2018::day6::part1(read_points(Question { year: 2018, day: 6 })).save_part1();
    y2018::day6::part2(read_points(Question { year: 2018, day: 6 })).save_part2();

    y2018::day7::part1(read_strings(Question { year: 2018, day: 7 })).save_part1();
    y2018::day7::part2(read_strings(Question { year: 2018, day: 7 })).save_part2();

    y2019::day1::part1(read_ints(Question { year: 2019, day: 1 })).save_part1();
    y2019::day1::part2(read_ints(Question { year: 2019, day: 1 })).save_part2();
}
