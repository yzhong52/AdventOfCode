use crate::helpers::parser::Question;

pub trait Puzzle<Input, Output> {
    fn day() -> i8;
    fn parser() -> fn(Question) -> Input;
    fn part1(input: &Input) -> Output;
    fn part2(input: &Input) -> Output;
}
