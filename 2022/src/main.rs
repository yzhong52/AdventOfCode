use std::env;
use std::fs;

fn day1() {
    let content = fs::read_to_string("day1").unwrap();

    let rows: Vec<_> = content.split("\n").collect::<Vec<_>>();

    let grouped_numbers: Vec<Vec<_>> = rows.iter().fold(vec![vec![]], |mut acc, x| {
        if x.len() == 0 {
            acc.push(vec![]);
        } else {
            let value: i32 = x.parse().unwrap();
            acc.last_mut().unwrap().push(value);
        }
        acc
    });

    let mut nums = grouped_numbers
        .into_iter()
        .map(|group| group.iter().sum::<i32>())
        .collect::<Vec<i32>>();

    nums.sort();

    let max3: i32 = nums[nums.len() - 3..].iter().sum();

    println!("Part 1: {:?}", nums.last().unwrap());
    println!("Part 2: {:?}", max3);
}

fn main() {
    day1()
}
