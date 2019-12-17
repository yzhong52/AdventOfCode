use super::super::helpers::parser::*;

fn parse(s: &String) -> Vec<i8> {
    s.chars().into_iter().map(|x| x as i8 - '0' as i8).collect()
}

pub fn part1(input: Input<String>) -> Answer<String> {
    let mut digits = parse(&input.data);
    let base_pattern: Vec<i8> = vec![0, 1, 0, -1];
    for phase in 0..100 {
        let mut next_digits: Vec<i8> = Vec::new();
        for row in 1..digits.len() + 1 {
            let mut total: i128 = 0;
            for (i, n) in digits.iter().enumerate() {
                let offset = (i + 1) / row;
                let patten = base_pattern[offset % base_pattern.len()];
                total += patten as i128 * *n as i128;
            }
            let last_digit: i8 = (&total.abs() % 10) as i8;
            next_digits.push(last_digit);
        }
        println!("{}: {:?}", phase + 1, &next_digits[0..8]);

        digits = next_digits;
    }

    let result: String = digits[0..8].iter().map(ToString::to_string).collect();
    return Answer { question: input.question, result };
}

pub fn part2(input: Input<String>) -> Answer<String> {
    return Answer { question: input.question, result: "..".to_string() };
}
