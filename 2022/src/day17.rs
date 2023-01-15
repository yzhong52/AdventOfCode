use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use std::{thread, time::Duration};

// "vertical chamber is exactly seven units wide"
const CHAMBER_WALL: [char; 9] = ['|', '.', '.', '.', '.', '.', '.', '.', '|'];
const INITIAL_BOTTOM: [char; 9] = ['+', '-', '-', '-', '-', '-', '-', '-', '+'];

fn has_overlap(rock: &Vec<Vec<char>>, bottom: &Vec<[char; 9]>, x: usize, y: usize) -> bool {
    for (i, row) in rock.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            let x2 = x + j;
            let y2 = y + rock.len() - 1 - i;

            if ch == &'#' && bottom[y2][x2] != '.' {
                return true;
            }
        }
    }
    false
}

fn add_to_bottom(rock: &Vec<Vec<char>>, bottom: &mut Vec<[char; 9]>, x: usize, y: usize) {
    for (i, row) in rock.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            let x2 = x + j;
            let y2 = y + rock.len() - 1 - i;

            if ch == &'#' {
                bottom[y2][x2] = '#'
            }
        }
    }
}

fn print(bottom: &Vec<[char; 9]>) {
    let mut buffer = vec![];
    for i in 0..bottom.len().min(1000) {
        let row = bottom[bottom.len() - 1 - i];
        let row_str = row.iter().join("");
        buffer.push(row_str);
    }
    println!("{}", buffer.join("\n"));
    thread::sleep(Duration::from_millis(100));
}

fn simulate(
    rocks: &[Vec<Vec<char>>; 5],
    patterns: &Vec<char>,
    total_falling_rock_count: usize,
    debug: bool,
) -> usize {
    let mut bottom: Vec<[char; 9]> = vec![INITIAL_BOTTOM];
    let mut pattern_id = 0;

    let mut cache = HashMap::new();

    let mut height_skipped: usize = 0;
    let mut step_idx = 0;
    let mut multiplier_calculated = false;
    while step_idx < total_falling_rock_count {
        let rock_idx = step_idx % rocks.len();
        let rock = &rocks[rock_idx];

        // Just a magic number, it is unlikely to repeat more than this number and start becoming
        // different.
        const PATTERN_TEST_HEIGHT: usize = 30;
        if !multiplier_calculated && bottom.len() > PATTERN_TEST_HEIGHT {
            let bottom_pattern: Vec<_> = bottom[bottom.len() - PATTERN_TEST_HEIGHT..]
                .iter()
                .collect();
            let bottom_pattern: String = bottom_pattern
                .iter()
                .map(|row| row.iter().join(""))
                .join("\n");
            let key = (bottom_pattern.clone(), rock_idx, pattern_id);
            if cache.contains_key(&key) {
                if debug {
                    print(&bottom);
                }
                let (last_step, last_height) = cache.get(&key).unwrap();
                println!("Last time this is seen, we were at step: {last_step}");
                println!("We are now at step: {step_idx}");
                let height_difference = bottom.len() - last_height;
                let step_difference = step_idx - last_step;
                println!(
                    "height difference: {height_difference}, step difference: {step_difference}"
                );

                let multiplier = (total_falling_rock_count - step_idx) / step_difference;
                println!("multiplier {multiplier}");
                step_idx += multiplier * step_difference;
                height_skipped = multiplier * height_difference;
                multiplier_calculated = true;
            }
            cache.insert(key, (step_idx, bottom.len()));
        }

        // "Each rock appears so that its left edge is two units away from the
        // left wall and its bottom edge is three units above the highest rock
        // in the room (or the floor, if there isn't one)"
        let mut x = 3;
        let mut y = bottom.len() + 3;

        // Add some chamber wall so that it is easier to check to make sure the
        // moving of the rock won't fall out
        for _ in 0..3 + rock.len() {
            bottom.push(CHAMBER_WALL);
        }

        loop {
            let pattern = patterns[pattern_id];
            pattern_id = (pattern_id + 1) % patterns.len();
            if pattern == '<' && !has_overlap(&rock, &bottom, x - 1, y) {
                // move to the left
                x -= 1;
            } else if pattern == '>' && !has_overlap(&rock, &bottom, x + 1, y) {
                // move to the right
                x += 1;
            }
            if !has_overlap(&rock, &bottom, x, y - 1) {
                // move down
                y -= 1;
            } else {
                // cannot move down anymore
                add_to_bottom(&rock, &mut bottom, x, y);
                // remove empty chamber wall
                while bottom.last().unwrap() == &CHAMBER_WALL {
                    bottom.pop();
                }
                break;
            }
        }
        step_idx += 1;
    }
    bottom.len() - 1 + height_skipped
}

fn run(content: String, debug: bool) -> (String, String) {
    let patterns: Vec<char> = content.trim().chars().collect();

    let rocks: [Vec<Vec<char>>; 5] = [
        vec!["####"],
        vec![
            ".#.", //
            "###", //
            ".#.", //
        ],
        vec![
            "..#", //
            "..#", //
            "###", //
        ],
        vec![
            "#", //
            "#", //
            "#", //
            "#", //
        ],
        vec![
            "##", //
            "##", //
        ],
    ]
    .map(|rock| -> Vec<Vec<char>> {
        rock.into_iter()
            .map(|row| -> Vec<char> { row.chars().collect::<Vec<_>>() })
            .collect_vec()
    });

    let part1 = simulate(&rocks, &patterns, 2022, debug);

    // There is no way to simulate all 1000000000000 moves. But at some point, the
    // pattern is going to repeat.
    let part2 = simulate(&rocks, &patterns, 1000000000000, debug);

    println!("day17 part1: {}", part1);
    println!("day17 part2: {}", part2);
    (part1.to_string(), part2.to_string())
}

pub fn day17() -> (String, String) {
    let content = fs::read_to_string("input/day17").unwrap();
    run(content, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day17_example_test() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>".to_string();

        let (part1, part2) = run(input, true);
        assert_eq!(part2, "1514285714288");
        assert_eq!(part1, "3068");
    }

    #[test]
    fn day17_test() {
        let (part1, part2) = day17();
        assert_eq!(part1, "3159");
        assert_eq!(part2, "1566272189352");
    }
}
