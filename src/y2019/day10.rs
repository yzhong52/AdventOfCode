use super::super::helpers::parser::*;
use std::collections::HashSet;

fn count_visible(data: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let mut set: HashSet<(i32, bool)> = HashSet::new();
    for (i, row) in data.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c == &'#' {
                let diff_x = x as i32 - i as i32;
                let diff_y = y as i32 - j as i32;

                match (diff_x, diff_y) {
                    (0, 0) => (),
                    (0, dy) if dy < 0 => {
                        set.insert((std::i32::MIN, false));
                    }
                    (0, dy) if dy > 0 => {
                        set.insert((std::i32::MAX, false));
                    }
                    (dx, dy) => {
                        let value: i32 = ((dy as f32 / dx as f32) * 10000.0) as i32;
                        set.insert((value, dx > 0));
                    }
                }
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
