use super::super::helpers::parser::*;
use std::process::exit;

fn parse(s: &String) -> Vec<i8> {
    s.chars().into_iter().map(|x| x as i8 - '0' as i8).collect()
}

static BASE_PATTERN: [i8; 4] = [0, 1, 0, -1];

pub fn part1(input: Input<String>) -> Answer<String> {
    let mut digits = parse(&input.data);
    for _ in 0..100 {
        let mut next_digits: Vec<i8> = Vec::new();
        for row in 1..digits.len() + 1 {
            let mut total: i128 = 0;
            for (i, n) in digits.iter().enumerate() {
                let offset = (i + 1) / row;
                let patten = BASE_PATTERN[offset % BASE_PATTERN.len()];
                total += patten as i128 * *n as i128;
            }
            let last_digit: i8 = (&total.abs() % 10) as i8;
            next_digits.push(last_digit);
        }
        digits = next_digits;
    }

    let result: String = digits[0..8].iter().map(ToString::to_string).collect();
    return Answer { question: input.question, result };
}

const REPEATED_TIMES: usize = 100;

fn fft(x: Vec<i8>) -> Vec<i8> {
    for i in 0 .. x.len() {
        for j in 0 .. x.len() {
            let offset = (j + 1) / (i + 1);
            let patten = BASE_PATTERN[offset % BASE_PATTERN.len()];
            print!("{:2} ", patten)
        }
        println!()
    }
    println!();

    for i in 0 .. x.len()/2 {
        for j in x.len()/2 .. x.len() {
            let offset = (j + 1) / (i + 1);
            let patten = BASE_PATTERN[offset % BASE_PATTERN.len()];
            print!("{:2}    ", patten)
        }
        println!()
    }
    println!();

    for i in (0 .. x.len()).step_by(2) {
        for j in (0 .. x.len()).step_by(2) {
            let offset = (j + 1) / (i + 1);
            let patten = BASE_PATTERN[offset % BASE_PATTERN.len()];
            print!("{:2}    ", patten)
        }
        println!()
    }
    println!();

    vec![]
}

pub fn part2(input: Input<String>) -> Answer<String> {

    fft(vec![0; 8]);
    fft(vec![0; 16]);
    fft(vec![0; 32]);
    exit(0);

    let initial_digits = parse(&input.data);

    let total_length = REPEATED_TIMES * initial_digits.len();
    let mut digits: Vec<i8> = vec![0; total_length];
    for i in 0..REPEATED_TIMES {
        for j in 0..initial_digits.len() {
            digits[i * initial_digits.len() + j] = initial_digits[j];
        }
    }

    println!("{:?} - {}", digits[0..100].to_vec(), digits.len());

    for phase in 0..1 {
        let mut next_digits = vec![0; total_length];
        for row in 0..total_length {
            let mut total: i32 = 0;
            for column in 0..total_length {
                let offset = (column + 1) / (row + 1);
                let patten = BASE_PATTERN[offset % BASE_PATTERN.len()];
                total += patten as i32 * digits[column] as i32;
            }

            let last_digit: i8 = (&total.abs() % 10) as i8;
            next_digits.push(last_digit);

            if row % 2 == 0 {
                println!("{} - {}", row, total);
            }
        }
        digits = next_digits;
    }

    let result: String = digits[0..8].iter().map(ToString::to_string).collect();
    return Answer { question: input.question, result: "0".to_string() };
}
