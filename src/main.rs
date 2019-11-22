mod day1;
mod day2;
mod helpers;

use helpers::Question;
use helpers::read_ints;
use helpers::Input;
use crate::helpers::read_strings;

fn main() {
    day1::part1(read_ints(Question { year: 2018, day: 1 })).save_as("part1");
    day1::part2(read_ints(Question { year: 2018, day: 1 })).save_as("part2");
    day2::day2(read_strings(Question { year: 2018, day: 2 })).save();

    let data: Vec<String> = [
        "abcdef",
        "bababc",
        "abbcde",
        "abcccd",
        "aabcdd",
        "abcdee",
        "ababab"
    ].to_vec().iter().map(|x| x.to_string()).collect();

    let question = Question { year: 0, day: 0 };
    let input: Input<Vec<String>> = Input { question, data };
    day2::day2(input);
}
