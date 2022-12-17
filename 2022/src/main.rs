mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

mod day10;
mod day11;
mod day12;
mod day13;
mod day14;

use crate::day1::day1;
use crate::day2::day2;
use crate::day3::day3;
use crate::day4::day4;
use crate::day5::day5;
use crate::day6::day6;
use crate::day7::day7;
use crate::day8::day8;
use crate::day9::day9;

use crate::day10::day10;
use crate::day11::day11;
use crate::day12::day12;
use crate::day13::day13;
use crate::day14::day14;

use clap::Parser;
#[derive(clap::ValueEnum, Clone)]
enum Day {
    Day14,
    All,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_enum, default_value_t=Day::All)]
    day: Day,
}

fn main() {
    let args = Args::parse();
    match args.day {
        Day::All => {
            day1();
            day2();
            day3();
            day4();
            day5();
            day6();
            day7();
            day8();
            day9();
            day10();
            day11();
            day12();
            day13();
            day14();
        }
        Day::Day14 => {
            // Run a single day 14 with animation
            _ = day14::run(
                "
                498,4 -> 498,6 -> 496,6
                503,4 -> 502,4 -> 502,9 -> 494,9
                "
                .to_string(),
                true,
            )
        }
    }
}
