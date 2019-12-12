use super::super::helpers::parser::*;
use std::collections::HashSet;

fn count_visible(data: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let mut set: HashSet<(bool, bool, i32)> = HashSet::new();
    for (i, row) in data.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c == &'#' && !(i == x && j == y) {
                let diff_x = x as i32 - i as i32;
                let diff_y = y as i32 - j as i32;

                let value: i32;
                if diff_x != 0 {
                    value = ((diff_y as f32 / diff_x as f32) * 10000.0) as i32;
                } else {
                    // value = 0;
                    value = ((diff_y as f32 / diff_x as f32) * 10000.0) as i32;
//                    println!("{} {} {}", diff_y as f32, diff_x as f32, diff_y as f32 / diff_x as f32); // value = ((diff_y as f32 / diff_x as f32) * 10000.0) as i32;
                }

                set.insert((diff_x > 0, diff_y > 0, value));
            }
        }
    }
    set.len()
}


pub fn part1(input: Input<Vec<Vec<char>>>) -> Answer<usize> {
    let mut result = 0;

    for (i, row) in input.data.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c == &'#' {
                let count = count_visible(&input.data, i, j);
                result = usize::max(result, count);
            }
        }
    }
    Answer { question: input.question, result }
}
