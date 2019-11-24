use std::num::ParseIntError;
use std::collections::HashMap;

fn guardId(line: &str) -> Result<i32, std::num::ParseIntError> {
    // Guard #1381 begins shift
    let p1 = line.split("#").last().unwrap();
    let id = p1.split_whitespace().next().unwrap();
    return id.parse::<i32>();
}

pub fn part1(input: &Vec<String>) {
    let mut sorted = input.clone();
    sorted.sort();

    let map = HashMap<i32, [i32; 60] > ::new();
    for line in &sorted {
        match guardId(line) {
            Ok(i) => println!("{}", i),
            Err(_) => {
                println!("{}", line);
            }
        }
    }
}