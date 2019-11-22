use super::helpers::Input;
use super::helpers::Answer;
use std::collections::HashMap;

pub fn day2(input: Input<Vec<String>>) -> Answer<i32> {
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
