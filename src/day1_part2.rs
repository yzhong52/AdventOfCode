use std::collections::HashSet;
use Answer;

pub fn day1_part2(input: Vec<i32>) -> Answer<i32> {
    let mut total = 0;
    let mut seens: HashSet<i32> = HashSet::new();
    seens.insert(0);

    let mut found = false;
    let mut x = 0;

    while !found {
        let number = input[x % input.len()];
        x += 1;
        total += number;
        if seens.contains(&total) {
            println!("Already seen this frequency: {}", total);
            found = true;
        } else {
            println!("New frequency: {}", total);
            seens.insert(total);
        }
    }
    return Answer(total);
}
