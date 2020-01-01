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

//const REPEATED_TIMES: usize = 10000;

pub fn part2(input: Input<String>) -> Answer<String> {
    let initial_digits = parse(&input.data);

//    // Repeat this `REPEATED_TIMES` times
//    let mut digits = vec![vec![]; REPEATED_TIMES];
//    for i in 0 .. REPEATED_TIMES {
//        digits[i] = initial_digits.clone();
//    }
//
//    for phase in 0..100 {
//        let mut next_digits = vec![Vec::new(); REPEATED_TIMES];
//        for r1 in 0..REPEATED_TIMES {
//            for r2 in 0..digits[r1].len() {
//                let row = r1 * digits[r1].len() + r2 + 1;
//
//                let mut total: i128 = 0;
//                for c1 in 0..REPEATED_TIMES {
//                    for (c2, n) in digits[c1].iter().enumerate() {
//                        let column = c1 * digits[c1].len() + c2 + 1;
//                        let offset = column / row;
//                        let patten = BASE_PATTERN[offset % BASE_PATTERN.len()];
//                        total += patten as i128 * *n as i128;
//                    }
//                }
//
//                let last_digit: i8 = (&total.abs() % 10) as i8;
//                next_digits[r1].push(last_digit);
//            }
//        }
//        digits = next_digits;
//    }
//
//    let result: String = digits[0][0..8].iter().map(ToString::to_string).collect();
    return Answer { question: input.question, result: "0".to_string() };
}
