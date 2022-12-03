use std::fs;

const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSORS: u32 = 3;

const LOSE: u32 = 1; // i.e. 'X'
const DRAW: u32 = 2; // i.e. 'Y'
const WIN: u32 = 3; // i.e. 'Z'

// Winning on the right side
const WIN_COMBINATIONS: [(&u32, &u32); 3] =
    [(&SCISSORS, &ROCK), (&ROCK, &PAPER), (&PAPER, &SCISSORS)];

fn compute_score(options: &Vec<(u32, u32)>) -> u32 {
    let scores: Vec<u32> = options
        .iter()
        .map(|(left, right)| {
            if left == right {
                // draw
                3 + right
            } else if WIN_COMBINATIONS.contains(&(left, right)) {
                // win
                6 + right
            } else {
                // lose
                0 + right
            }
        })
        .collect();

    scores.iter().sum::<u32>()
}

pub fn day2() -> (u32, u32) {
    let content = fs::read_to_string("input/day2").unwrap();
    day2_run(content)
}

fn day2_run(content: String) -> (u32, u32) {
    let rows = content.split("\n");

    let pairs: Vec<_> = rows
        .map(|row| {
            let elements = row.split(" ").collect::<Vec<_>>();
            match &elements[..] {
                [left, right, ..] => Some((
                    left.chars().nth(0).unwrap() as u32 - 'A' as u32 + 1,
                    right.chars().nth(0).unwrap() as u32 - 'X' as u32 + 1,
                )),
                _ => None,
            }
        })
        .flatten()
        .collect();

    let part1 = compute_score(&pairs);

    let part2_pairs: Vec<_> = pairs
        .iter()
        .map(|(left, right)| match right {
            &WIN => {
                let option = WIN_COMBINATIONS
                    .iter()
                    .find(|(loser_option, _)| loser_option == &left)
                    .unwrap();
                (*left, *option.1)
            }
            &DRAW => (*left, *left),
            &LOSE => {
                let option = WIN_COMBINATIONS
                    .iter()
                    .find(|(_, winner_option)| winner_option == &left)
                    .unwrap();
                (*left, *option.0)
            }
            _ => {
                panic!("Not valid input")
            }
        })
        .collect();

    let part2 = compute_score(&part2_pairs);

    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2_test() {
        let (part1, part2) = day2();
        assert_eq!(part1, 10595);
        assert_eq!(part2, 9541);
    }

    #[test]
    fn day2_example_test() {
        let (part1, part2) = day2_run(
            r#"A Y
B X
C Z"#
                .to_string(),
        );
        assert_eq!(part1, 15);
        assert_eq!(part2, 12);
    }
}
