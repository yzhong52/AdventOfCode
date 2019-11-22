mod day1;
mod day2;
mod helpers;

use helpers::Question;
use helpers::read_ints;
use crate::helpers::read_strs;

fn main() {
    day1::part1(read_ints(Question { year: 2018, day: 1 })).save_as("part1");
    day1::part2(read_ints(Question { year: 2018, day: 1 })).save_as("part2");

    day2::day2(read_strs(Question { year: 2018, day: 2 })).save();
}
