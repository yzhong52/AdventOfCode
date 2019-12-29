use crate::helpers::parser::Answer;
use crate::helpers::parser::Input;
use std::iter::Rev;
use std::slice::Iter;
use crate::y2019::day22::Shuffle::DealNewDeck;

pub fn part1(input: Input<Vec<String>>) -> Answer<usize> {
    let mut deck: Vec<i32> = (0..10007).collect();
    for row in input.data {
        if row.starts_with("deal with increment ") {
            let parts: Vec<&str> = row.split(" ").collect();
            let increment: usize = parts.last().unwrap().parse().unwrap();
            let mut new_deck: Vec<i32> = vec![0; deck.len()];
            let mut j = 0;
            for i in 0..deck.len() {
                new_deck[j] = deck[i];
                j = (j + increment) % deck.len();
            }
            deck = new_deck;
        } else if row.starts_with("deal into new stack") {
            deck.reverse()
        } else if row.starts_with("cut") {
            let parts: Vec<&str> = row.split(" ").collect();
            let mut number: i32 = parts.last().unwrap().parse().unwrap();
            if number > 0 {
                number = number % deck.len() as i32;
            } else if number < 0 {
                number = deck.len() as i32 - (-number) % deck.len() as i32;
            }
            let mut left = deck[0..number as usize].to_vec();
            let right = deck[number as usize..deck.len()].to_vec();
            deck = right;
            deck.append(left.as_mut());
        }
    }

    let result = deck.iter().position(|x| *x == 2019).unwrap();
    Answer { question: input.question, result }
}

enum Shuffle {
    Cut(i32),
    DealWithIncrement(usize),
    DealNewDeck,
}

fn shuffle(shuffles: Vec<Shuffle>, target_position: usize, deck_len: usize, repeat: usize) -> usize {
    let shuffles: Vec<&Shuffle> = shuffles.iter().rev().collect();

    let mut result: usize = target_position;

    for r in 0..repeat {
//        if r % (repeat / 10000000) == 0 {
        println!("Shuffle cards iteration {} ({}%)", r, (r * 100000 / repeat) as f32 / 1000.0);
//        }
        for row in &shuffles {
            match row {
                Shuffle::DealWithIncrement(increment) => {
                    let number_rounds = deck_len / *increment;
                    let remain = result % *increment;
                    let divider = result / *increment;

                    result = remain * number_rounds + divider;
                }
                Shuffle::DealNewDeck => {
                    result = deck_len - 1 - result;
                }
                Shuffle::Cut(number) => {
                    let positive_number: usize;
                    if *number >= 0 {
                        positive_number = *number as usize % deck_len;
                    } else {
                        positive_number = deck_len - (-number) as usize % deck_len
                    };

                    if result >= positive_number {
                        result -= positive_number;
                    } else {
                        result = deck_len - (positive_number - result);
                    }
                }
            }
        }
    }
    result
}

fn parse(shuffles: &Vec<String>) -> Vec<Shuffle> {
    let mut result = vec![];
    let deck_len = shuffles.len();
    for row in shuffles {
        if row.starts_with("deal with increment ") {
            let parts: Vec<&str> = row.split(" ").collect();
            let increment: usize = parts.last().unwrap().parse().unwrap();
            result.push(Shuffle::DealWithIncrement(increment));
        } else if row.starts_with("deal into new stack") {
            result.push(Shuffle::DealNewDeck);
        } else if row.starts_with("cut") {
            let parts: Vec<&str> = row.split(" ").collect();
            let mut number: i32 = parts.last().unwrap().parse().unwrap();
            result.push(Shuffle::Cut(number))
        }
    }
    result
}

pub fn part2(input: Input<Vec<String>>) -> Answer<usize> {
    let parsed = parse(&input.data);
    // let result = shuffle(parsed, 2020, 119315717514047, 101741582076661);
    let result = shuffle(parsed, 2019, 10007, 1);
    Answer { question: input.question, result }
}
