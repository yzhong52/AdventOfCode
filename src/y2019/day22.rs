use crate::helpers::parser::Answer;
use crate::helpers::parser::Input;

pub fn shuffle1(data: Vec<String>, original_position: usize, deck_length: usize) -> usize {
    let mut deck: Vec<usize> = (0..deck_length).collect();
    for row in data {
        if row.starts_with("deal with increment ") {
            let parts: Vec<&str> = row.split(" ").collect();
            let increment: usize = parts.last().unwrap().parse().unwrap();
            let mut new_deck: Vec<usize> = vec![0; deck.len()];
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

    deck.iter().position(|x| *x == original_position).unwrap()
}

enum Shuffle {
    Cut(i32),
    DealWithIncrement(usize),
    DealNewDeck,
}

fn shuffle2(shuffles: Vec<Shuffle>, original_position: usize, deck_len: usize, repeat: usize) -> usize {
    let shuffles: Vec<&Shuffle> = shuffles.iter().rev().collect();

    let mut result: usize = original_position;

    for r in 0..repeat {
//        if r % (repeat / 10000000) == 0 {
        println!("Shuffle cards iteration {} ({}%)", r, (r * 100000 / repeat) as f32 / 1000.0);
//        }
        for row in &shuffles {
            match row {
                Shuffle::DealWithIncrement(increment) => {
                    let number_rounds = deck_len / *increment;
                    let remain = result % number_rounds;
                    let divider = result / number_rounds;

                    result = remain * (number_rounds + 1) + divider;
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
            result.push(Shuffle::Cut(number))
        }
    }
    result
}

pub fn part1(input: Input<Vec<String>>) -> Answer<usize> {
    let result = shuffle1(input.data, 10, 10007);
    Answer { question: input.question, result }
}

pub fn part2(input: Input<Vec<String>>) -> Answer<usize> {
    let parsed = parse(&input.data);
    // let result = shuffle(parsed, 2020, 119315717514047, 101741582076661);
    let result = shuffle2(parsed, 10, 10007, 1);
    Answer { question: input.question, result }
}
