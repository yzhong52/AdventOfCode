use super::super::helpers::parser::*;

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

fn visualization(size: usize) {
    for i in 0..size {
        for j in 0..size {
            let offset = (j + 1) / (i + 1);
            let patten = BASE_PATTERN[offset % BASE_PATTERN.len()];
            print!("{:2} ", patten)
        }
        println!()
    }
}

// Inspired by:
// * https://www.reddit.com/r/adventofcode/comments/eh8s3d/aoc_2019_wording_issues_and_improvements/
//
// Oh, oh, I missed this line, the "initial input signal" instead of the final signal:
//
// "The first seven digits of your initial input signal also represent the message offset. "
pub fn part2(input: Input<String>) -> Answer<String> {
    visualization(32);

    const REPEATED_TIMES: usize = 10000;

    let message_offset = 5977709;

    let initial_digits = parse(&input.data);

    let total_length = REPEATED_TIMES * initial_digits.len();

    let remaining_length = total_length - message_offset;
    println!("remaining_length length: {}", remaining_length);

    let mut truncated_digits = vec![0; remaining_length];
    for i in 0..remaining_length {
        truncated_digits[i] = initial_digits[(i + message_offset) % initial_digits.len()];
    }

    for phase in 0..100 {
        let mut next_phase_result = truncated_digits.clone();

        next_phase_result[remaining_length - 1] = truncated_digits[remaining_length - 1];
        for i in 1..remaining_length {
            next_phase_result[remaining_length - i - 1] += next_phase_result[remaining_length - i];
            next_phase_result[remaining_length - i - 1] %= 10;
        }

        truncated_digits = next_phase_result;

        println!("Phase {}: {:?}", phase, truncated_digits[0..8].to_vec());
    }

    let result: String = truncated_digits[0..8].iter().map(ToString::to_string).collect();
    return Answer { question: input.question, result };
}
