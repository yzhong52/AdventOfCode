use crate::helpers::parser::Answer;
use crate::helpers::parser::Input;
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::collections::HashSet;
use std::process::exit;

// This solution would take 1.9 TB of memory for part 2
fn shuffle1(shuffles: Vec<Shuffle>, original_position: usize, deck_length: usize) -> usize {
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

    println!("At position {}: {}", original_position, deck[original_position]);
    println!("{:?}", deck);
    deck.iter().position(|x| *x == original_position).unwrap()
}

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

// This solution would take about 13 years for part 2
fn shuffle2(shuffles: Vec<Shuffle>, original_position: usize, deck_len: usize, repeat: usize) -> usize {
    let mut result: usize = original_position;
    println!("Initial result {}", result);
    for i in 0..repeat {
        for row in &shuffles {
            match row {
                Shuffle::DealWithIncrement(increment) => {
                    result = (result * *increment) % deck_len;
                }
                Shuffle::DealNewDeck => {
                    result = deck_len - 1 - result;
                }
                Shuffle::Cut(number) => {
                    result = (deck_len + result - *number) % deck_len;
                }
            }
        }
        println!("{}: {}", i, result);
        sleep(Duration::from_millis(300));
    }
    result
}

// Would take 29 years.
fn shuffle3(shuffles: Vec<Shuffle>, original_position: usize, deck_len: usize, repeat: usize) -> usize {
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
        if i % 1000000 == 0 {
            let ratio = i as f32 / repeat as f32;
            println!("{}", ratio);
            println!("Time passed: {:?}", start.elapsed());
            println!("Total: {:?}", start.elapsed() * (1.0 / ratio) as u32);
        }
    }
    result
}

pub fn part1(input: Input<Vec<String>>) -> Answer<usize> {
    let deck_len = 10007;
    let parsed = parse(&input.data, deck_len);

    let result = shuffle1(parsed.clone(), 2019, 10007);
    let result2 = shuffle2(parsed, 2019, deck_len, 1);

    println!("Validate {} vs {}", result, result2);
    Answer { question: input.question, result }
}

pub fn part2(input: Input<Vec<String>>) -> Answer<usize> {
    let deck_len = 119315717514047;
    let parsed = parse(&input.data, deck_len);

    // let result = shuffle2(parsed, 2020, deck_len, 101741582076661);
    let result = shuffle3(parsed, 2020, deck_len, 101741582076661);

    Answer { question: input.question, result }
}
