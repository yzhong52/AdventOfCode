use itertools::Itertools;
use std::fs;
const CHAMBER_WALL: [char; 9] = ['|', '.', '.', '.', '.', '.', '.', '.', '|'];

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
    for i in 0..bottom.len().min(15) {
        let row = bottom[bottom.len() - 1 - i];
        let row_str = row.iter().join("");
        buffer.push(row_str);
    }
    println!("{}", buffer.join("\n"));
}

fn simulate(patterns: &Vec<char>, rock_count: usize, debug: bool) -> usize {
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

    // "vertical chamber is exactly seven units wide"
    let mut bottom: Vec<[char; 9]> = vec![['+', '-', '-', '-', '-', '-', '-', '-', '+']];
    let mut pattern_id = 0;
    for _rock_id in 0..rock_count {
        let rock = &rocks[_rock_id % rocks.len()];
        println!("Rock {}", _rock_id);

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
        if debug {
            print(&bottom);
        }

        loop {
            let pattern = patterns[pattern_id % patterns.len()];
            pattern_id += 1;
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
                if debug {
                    print(&bottom);
                }
                // remove empty chamber wall
                while bottom.last().unwrap() == &CHAMBER_WALL {
                    bottom.pop();
                }
                break;
            }
        }
    }
    bottom.len() - 1
}

fn run(content: String, debug: bool) -> (String, String) {
    let patterns: Vec<char> = content.trim().chars().collect();
    let part1 = simulate(&patterns, 2022, debug);
    // let part2 = simulate(&patterns, 1000000000000);
    let part2 = simulate(&patterns, 1000000, false);
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
        assert_eq!(part1, "3068");
        assert_eq!(part2, "1514285714288");
    }

    #[test]
    fn day17_test() {
        let (part1, part2) = day17();
        assert_eq!(part1, "x");
        assert_eq!(part2, "x");
    }
}
