use crate::helpers::parser::Answer;
use crate::helpers::parser::Input;
use std::thread::sleep;
use std::time::{Duration, Instant};

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

// Solution like this would take 1.9 TB of memory for part 2, though part 2 is asking for reverse shuffle
#[allow(dead_code)]
fn shuffle_part1_naive(shuffles: Vec<Shuffle>, original_position: usize, deck_length: usize) -> usize {
    let mut deck: Vec<usize> = (0..deck_length).collect();
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

    deck.iter().position(|x| *x == original_position).unwrap()
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

// This solution would take about 29 years for part 2
#[allow(dead_code)]
fn shuffle_part2_in_29_years(shuffles: Vec<Shuffle>, original_position: usize, deck_len: usize, repeat: usize) -> usize {
    let start = Instant::now();
    let mut result: usize = original_position;

    let mut reversed = shuffles;
    reversed.reverse();

    for i in 0..repeat {
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

// Inspired by: https://www.reddit.com/r/adventofcode/comments/eh1d6p/2019_day_22_part_2_tutorial/

fn shuffle_part2(shuffles: Vec<Shuffle>, original_position: usize, deck_len: usize, repeat: usize) -> usize {
    // After shuffling, the result = multiplier * result + constant
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
            Shuffle::Cut(number) => {
                constant = constant - number as i128;
            }
        }
        multiplier = multiplier % deck_len as i128;
        constant = constant % deck_len as i128;
    }
    println!("Finally: multiplier: {}, constant: {}", multiplier, constant);

    println!("Finally: multiplier: {}, constant: {}", multiplier, constant);
    let result = (original_position as i128 * multiplier + constant) % deck_len as i128;
    result as usize
}

pub fn part1(input: Input<Vec<String>>) -> Answer<usize> {
    let deck_len = 10007;
    let parsed = parse(&input.data, deck_len);
    let result = shuffle_part1(parsed, 2019, deck_len);
    Answer { question: input.question, result }
}

pub fn part2(input: Input<Vec<String>>) -> Answer<usize> {
    let deck_len = 10007;
    let parsed = parse(&input.data, deck_len);

    // let result = shuffle2(parsed, 2020, deck_len, 101741582076661);
    let result = shuffle_part1(parsed.clone(), 2019, deck_len);
    println!("29 years: {}", result);
    let result = shuffle_part2(parsed, 2019, deck_len, 1);
    println!("fast: {}", result);

    Answer { question: input.question, result }
}
