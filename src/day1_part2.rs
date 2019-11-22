use std::collections::HashSet;
use super::helpers::Input;
use super::helpers::Answer;

pub fn day1_part2(input: Input<Vec<i32>>) -> Answer<i32> {
    let mut total = 0;
    let mut seen: HashSet<i32> = HashSet::new();
    seen.insert(0);

    let mut found = false;
    let mut x = 0;

    while !found {
        let number = input.data[x % input.data.len()];
        x += 1;
        total += number;
        if seen.contains(&total) {
            // println!("Already seen this frequency: {}", total);
            found = true;
        } else {
            // println!("New frequency: {}", total);
            seen.insert(total);
        }
    }
    return Answer { question: input.question, result: total };
}
