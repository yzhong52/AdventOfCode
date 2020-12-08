use std::ops::RangeInclusive;
use crate::helpers::puzzle::Puzzle;

pub struct Day4 {}

impl Day4 {
    fn day4_static_input(_filename: String) -> RangeInclusive<i32> {
        265275..=781584
    }
}

impl Puzzle<RangeInclusive<i32>, i32> for Day4 {
    fn day(&self) -> i8 {
        4
    }

    fn parser(&self) -> fn(String) -> RangeInclusive<i32> {
        Day4::day4_static_input
    }

    fn part1(&self, input: &RangeInclusive<i32>) -> i32 {
        fn meet_the_rule(number: i32) -> bool {
            let mut remain = number;
            let mut last_digit = 10;
            let mut same_digit = false;
            while remain > 0 {
                let digit = remain % 10;
                remain /= 10;

                if digit > last_digit {
                    return false;
                }
                same_digit = same_digit || (digit == last_digit);
                last_digit = digit;
            }
            return same_digit;
        }

        input.clone().map(meet_the_rule).map(|x| x as i32).sum()
    }

    fn part2(&self, input: &RangeInclusive<i32>) -> i32 {
        fn meet_the_rule_part2(number: i32) -> bool {
            let mut remain = number;
            let mut last_digit = 10;
            let mut same_digit_repeated_twice = false;
            let mut same_digit_count = 1;
            while remain > 0 {
                let digit = remain % 10;

                if digit > last_digit {
                    return false;
                }

                if digit == last_digit {
                    same_digit_count += 1;
                } else {
                    if same_digit_count == 2 {
                        same_digit_repeated_twice = true
                    }
                    same_digit_count = 1;
                }

                last_digit = digit;
                remain /= 10;
            }

            return same_digit_count == 2 || same_digit_repeated_twice;
        }
        input.clone().map(meet_the_rule_part2).map(|x| x as i32).sum()
    }
}
