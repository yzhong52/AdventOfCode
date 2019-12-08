use super::super::helpers::parser::*;
use std::collections::{HashMap, HashSet};

pub fn part1(input: Input<Vec<String>>) -> Answer<i32> {
    let mut two_count = 0;
    let mut three_count = 0;

    for st in input.data {
        let mut map: HashMap<char, i32> = HashMap::new();

        for ch in st.chars() {
            let count = map.get(&ch).unwrap_or(&0) + 1;
            map.insert(ch, count);
        }

        let has_two = map.values().any(|x| *x == 2);
        let has_three = map.values().any(|x| *x == 3);

        two_count += has_two as i32;
        three_count += has_three as i32;
    }
    return Answer { question: input.question, result: two_count * three_count };
}

pub fn part2(input: Input<Vec<String>>) -> Answer<String> {
    let mut fuzzy_words: HashSet<String> = HashSet::new();
    for st in input.data {
        for i in 0..st.len() {
            // let mut fuzzy_word = [st[0..i], st[i..st.len()]].concat();
            let left: String = st[0..i].to_string();
            let right: String = st[i + 1..].to_string();
            let fuzzy_word = format!("{}?{}", left, right);
            if fuzzy_words.contains(&fuzzy_word) {
                return Answer { question: input.question, result: fuzzy_word.replace('?', "") };
            } else {
                fuzzy_words.insert(fuzzy_word);
            }
        }
    }
    return Answer { question: input.question, result: "NOT FOUND".to_string() };
}