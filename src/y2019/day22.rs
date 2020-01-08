use crate::helpers::parser::{Answer, read_strings, parse_strings};
use crate::helpers::parser::Input;
use modinverse::modinverse;
use crate::helpers::puzzle::Puzzle;

#[derive(Copy, Clone)]
enum Shuffle {
    Cut(usize),
    DealWithIncrement(usize),
    DealNewDeck,
}

fn parse(shuffles: &Vec<String>, deck_len: usize) -> Vec<Shuffle> {
    let mut result = vec![];
    for row in shuffles {
        if row.starts_with("deal with increment ") {
            let parts: Vec<&str> = row.split(" ").collect();
            let increment: usize = parts.last().unwrap().parse().unwrap();
            result.push(Shuffle::DealWithIncrement(increment));
        } else if row.starts_with("deal into new stack") {
            result.push(Shuffle::DealNewDeck);
        } else if row.starts_with("cut") {
            let parts: Vec<&str> = row.split(" ").collect();
            let number: i32 = parts.last().unwrap().parse().unwrap();
            if number >= 0 {
                result.push(Shuffle::Cut(number as usize % deck_len))
            } else {
                result.push(Shuffle::Cut(deck_len - (-number) as usize % deck_len))
            };
        }
    }
    result
}

// For part 1:
//
//     deck.iter().position(|x| *x == position).unwrap()
//
// For part 2 (would take 1.9 TB of memory and infinite time using this solution):
//
//    deck[position]
#[allow(dead_code)]
fn shuffle_part1_naive(shuffles: Vec<Shuffle>, deck_size: usize) -> Vec<usize> {
    let mut deck: Vec<usize> = (0..deck_size).collect();
    for row in &shuffles {
        match row {
            Shuffle::DealWithIncrement(increment) => {
                let mut new_deck: Vec<usize> = vec![0; deck.len()];
                let mut j = 0;
                for i in 0..deck.len() {
                    new_deck[j] = deck[i];
                    j = (j + *increment) % deck.len();
                }
                deck = new_deck;
            }
            Shuffle::DealNewDeck => {
                deck.reverse()
            }
            Shuffle::Cut(number) => {
                let mut left = deck[0..*number].to_vec();
                let right = deck[*number..deck.len()].to_vec();
                deck = right;
                deck.append(left.as_mut());
            }
        }
    }

    deck
}

fn shuffle_part1(shuffles: Vec<Shuffle>, original_position: usize, deck_len: usize) -> usize {
    let mut result: usize = original_position;
    for row in shuffles {
        match row {
            Shuffle::DealWithIncrement(increment) => {
                result = (result * increment) % deck_len;
            }
            Shuffle::DealNewDeck => {
                result = deck_len - 1 - result;
            }
            Shuffle::Cut(number) => {
                result = (deck_len + result - number) % deck_len;
            }
        }
    }
    result
}

// This solution would still take about 29 years for part 2
#[allow(dead_code)]
fn shuffle_part2_in_29_years(shuffles: Vec<Shuffle>, original_position: usize, deck_len: usize, repeat: usize) -> usize {
    let mut result: usize = original_position;

    let mut reversed = shuffles;
    reversed.reverse();

    for _ in 0..repeat {
        for row in &reversed {
            match row {
                Shuffle::DealWithIncrement(increment) => {
                    let mut i = 0;
                    while (result + deck_len * i) % *increment != 0 {
                        i += 1;
                    }
                    result = (result + deck_len * i) / *increment
                }
                Shuffle::DealNewDeck => {
                    result = deck_len - 1 - result;
                }
                Shuffle::Cut(number) => {
                    result = (result + *number) % deck_len;
                }
            }
        }
    }
    result
}

fn repeat(multiplier: i128, constant: i128, count: usize, modulo: i128) -> (i128, i128) {
    if count == 1 {
        (multiplier % modulo, constant % modulo)
    } else {
        if count % 2 == 0 {
            let (m2, c2) = repeat(multiplier, constant, count / 2, modulo);
            let final_m = (m2 * m2) % modulo;
            let final_c = (m2 * c2 + c2) % modulo;
            (final_m, final_c)
        } else {
            let (m1, c1) = repeat(multiplier, constant, count - 1, modulo);
            let final_m = (multiplier * m1) % modulo;
            let final_c = (multiplier * c1 + constant) % modulo;
            (final_m, final_c)
        }
    }
}

// Inspired by:
// * https://www.reddit.com/r/adventofcode/comments/eh1d6p/2019_day_22_part_2_tutorial/
// * https://codeforces.com/blog/entry/72593
fn shuffle_part2(shuffles: Vec<Shuffle>, original_position: usize, deck_size: usize, times: usize) -> i128 {
    let deck_size = deck_size as i128;

    let mut reversed = shuffles.clone();
    reversed.reverse();

    // After shuffling, result = multiplier * result + constant
    // Refer to `shuffle_part1` also for the computation of `multiplier` and `constant`
    let mut multiplier: i128 = 1;
    let mut constant: i128 = 0;
    for row in shuffles {
        match row {
            Shuffle::DealWithIncrement(increment) => {
                multiplier *= increment as i128;
                constant *= increment as i128;
            }
            Shuffle::DealNewDeck => {
                multiplier = -multiplier;
                constant = -1 - constant;
            }
            Shuffle::Cut(count) => {
                constant = constant - count as i128;
            }
        }
        multiplier = multiplier % deck_size as i128;
        constant = constant % deck_size as i128;
    }

    // "When did you become and expert in modular arithmetic? "
    // https://www.reddit.com/r/adventofcode/comments/eeb40v/day_22_part_2/
    let inverse_multiplier = modinverse(multiplier, deck_size as i128).unwrap();
    let inverse_constant = (-inverse_multiplier * constant) % deck_size as i128;

    let (repeated_inverse_multiplier, repeated_inverse_constant) = repeat(
        inverse_multiplier,
        inverse_constant,
        times,
        deck_size as i128,
    );

    (original_position as i128 * repeated_inverse_multiplier + repeated_inverse_constant) % deck_size
}

pub struct Day22 {}

impl Puzzle<Vec<String>, usize> for Day22 {
    fn day(&self) -> i8 {
        22
    }

    fn parser(&self) -> fn(String) -> Vec<String> {
        parse_strings
    }

    fn part1(&self, input: &Vec<String>) -> usize {
        let deck_deck_size = 10007;
        let parsed = parse(input, deck_deck_size);
        shuffle_part1(parsed, 2019, deck_deck_size)
    }

    fn part2(&self, input: &Vec<String>) -> usize {
        let deck_size = 119315717514047;
        let parsed = parse(input, deck_size);
        let result = shuffle_part2(
            parsed,
            2020,
            deck_size,
            101741582076661,
        );

        result as usize
    }
}
