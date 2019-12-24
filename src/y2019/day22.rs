use crate::helpers::parser::Answer;
use crate::helpers::parser::Input;

pub fn part1(input: Input<Vec<String>>) -> Answer<i32> {
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

    Answer { question: input.question, result: deck[2019] }
}

pub fn part2(input: Input<Vec<String>>) -> Answer<i32> {
    Answer { question: input.question, result: 0 }
}
