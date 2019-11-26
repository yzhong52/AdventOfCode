//https://adventofcode.com/2018/day/4

use std::num::ParseIntError;
use std::collections::HashMap;
use super::helpers::Input;
use super::helpers::Answer;

fn cut(line: &str, from: char, to: char) -> String {
    let p1 = line.split(from).last().unwrap();
    return p1.split(to).next().unwrap().to_string();
}

fn parse_guard_id(line: &str) -> Result<i32, ParseIntError> {
    // Guard #1381 begins shift
    return cut(line, '#', ' ').parse::<i32>();
}

fn parse_minute(line: &str) -> i32 {
    // [1518-11-04 00:36] falls asleep
    let target = cut(line, ':', ']');
    return target.parse::<i32>().unwrap();
}

pub fn helper(input: &Input<Vec<String>>) -> HashMap<i32, [i32; 60]> {
    let mut sorted = input.data.clone();
    sorted.sort();

    // We use 60 here because:
    //
    // > Because all asleep/awake times are during the midnight hour (00:00 - 00:59), only the minute
    // > portion (00 - 59) is relevant for those events.
    let mut map: HashMap<i32, [i32; 60]> = HashMap::new();
    let mut current_index: i32 = 0;
    let mut current_asleep: i32 = 0;

    for line in &sorted {
        match parse_guard_id(line) {
            Ok(i) => {
                current_index = i;
            }
            Err(_) => {
                if line.ends_with("asleep") {
                    current_asleep = parse_minute(line);
                } else if line.ends_with("up") {
                    let wakes_up = parse_minute(line);

                    let mut row: [i32; 60] = *map.get(&current_index).unwrap_or(&[0; 60]);
                    for i in current_asleep..wakes_up {
                        row[i as usize] += 1
                    }
                    map.insert(current_index, row);
                }
            }
        }
    }

    return map;
}

pub fn part1(input: Input<Vec<String>>) -> Answer<i32> {
    let map = helper(&input);
    let mut global_maximum_total_sleep = 0;
    let mut result: i32 = 0;

    for (guard_id, sleep_counts) in map.iter() {
        let mut local_total_sleep = 0;
        let mut local_maximum_sleep_count = 0;
        let mut local_maximum_sleep_time = 0;

        for (time, count) in sleep_counts.iter().enumerate() {
            if local_maximum_sleep_count < *count {
                local_maximum_sleep_count = *count;
                local_maximum_sleep_time = time;
            }
            local_total_sleep += *count;
        }

        if local_total_sleep > global_maximum_total_sleep {
            global_maximum_total_sleep = local_total_sleep;
            result = *guard_id * local_maximum_sleep_time as i32;
        }
    }

    return Answer { question: input.question, result: result };
}


pub fn part2(input: Input<Vec<String>>) -> Answer<i32> {
    let map = helper(&input);

    let mut result: i32 = 0;

    let mut maximum_sleep = 0;

    for (guard_id, sleep_counts) in map.iter() {
        for (time, count) in sleep_counts.iter().enumerate() {
            if maximum_sleep < *count {
                maximum_sleep = *count;
                result = *guard_id * time as i32;
            }
        }
    }

    return Answer { question: input.question, result };
}
