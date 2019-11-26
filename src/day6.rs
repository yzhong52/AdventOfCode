use super::helpers::parser::*;
use super::helpers::models::*;

pub fn part1(input: Input<Vec<Point>>) -> Answer<usize> {
    let points = input.data;
    println!("Points {:?}", points);
    Answer { question: input.question, result: 0 }
}