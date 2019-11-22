mod day1_part2;
mod helpers;

use helpers::Question;
use helpers::read_ints;

fn main() {
    let y2018_d1 = Question { year: 2018, day: 1 };
    day1_part2::day1_part2(read_ints(y2018_d1)).save();
}
