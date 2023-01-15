use std::{
    fs,
    io::{stdout, Write},
    thread,
    time::Duration,
};

use itertools::Itertools;

// Coordinates for day 14
// +--> x
// |
// |
// y

pub fn day14() -> (String, String) {
    let content = fs::read_to_string("input/day14").unwrap();
    run(content, false)
}

fn print(buffer: &Vec<Vec<char>>, overlap: Option<(i32, i32)>) {
    let mut buffer2 = buffer.clone();

    if let Some((x, y)) = overlap {
        if 0 <= x && x < buffer2[0].len() as i32 && y >= 0 && y < buffer2.len() as i32 {
            buffer2[y as usize][x as usize] = 'O';
        }
    }

    println!(
        "{}\n",
        buffer2
            .iter()
            .map(|line| line.iter().collect::<String>())
            .join("\n")
    );

    stdout().flush().expect("Unable to flush stdout");
}

fn paint(buffer: &mut Vec<Vec<char>>, x: i32, y: i32, c: char) {
    buffer[y as usize][x as usize] = c;
}

fn get(buffer: &Vec<Vec<char>>, x: i32, y: i32) -> char {
    if 0 <= x && x < buffer[0].len() as i32 && 0 <= y && y < buffer.len() as i32 {
        buffer[y as usize][x as usize]
    } else {
        '.'
    }
}

fn simulate(buffer: &mut Vec<Vec<char>>, start_x: i32, start_y: i32, animate: bool) {
    if animate {
        print(&buffer, None);
        thread::sleep(Duration::from_millis(5000));
    }

    let mut should_continue = true;
    while should_continue {
        let mut cur_x = start_x;
        let mut cur_y = start_y;

        should_continue = false;
        while cur_y < buffer.len() as i32 && cur_x < buffer[0].len() as i32 && cur_x >= 0 {
            if get(buffer, cur_x, cur_y + 1) == '.' {
                cur_y += 1;
            } else if get(buffer, cur_x - 1, cur_y + 1) == '.' {
                cur_x -= 1;
                cur_y += 1;
            } else if get(buffer, cur_x + 1, cur_y + 1) == '.' {
                cur_x += 1;
                cur_y += 1;
            } else {
                paint(buffer, cur_x, cur_y, 'o');
                if animate {
                    print(&buffer, None);
                    thread::sleep(Duration::from_millis(20));
                }
                if cur_x == start_x && cur_y == start_y {
                    should_continue = false;
                } else {
                    should_continue = true;
                }
                break;
            }

            if animate {
                print(buffer, Some((cur_x, cur_y)));
                thread::sleep(Duration::from_millis(20));
            }
        }
    }
}

const SAND_ORIGIN_X: i32 = 500;
const SAND_ORIGIN_Y: i32 = 0;

fn solve(
    lines: &Vec<Vec<(i32, i32)>>,
    max_x: i32,
    min_x: i32,
    max_y: i32,
    min_y: i32,
    with_floor: bool,
    animate: bool,
) -> usize {
    let width_x = (max_x - min_x + 1) as usize;
    let width_y = (max_y - min_y + 1) as usize;
    let mut buffer = vec![vec!['.'; width_x]; width_y];

    let start_x = SAND_ORIGIN_X - min_x;
    let start_y = SAND_ORIGIN_Y - min_y;

    paint(&mut buffer, start_x, start_y, 'x');

    for line in lines {
        for i in 0..line.len() - 1 {
            let (x0, y0) = line[i];
            let (x1, y1) = line[i + 1];
            for x in i32::min(x0, x1)..=i32::max(x0, x1) {
                for y in i32::min(y0, y1)..=i32::max(y0, y1) {
                    paint(&mut buffer, x - min_x, y - min_y, '#');
                }
            }
        }
    }

    if with_floor {
        let last_row = buffer.len() - 1;
        for x in 0..buffer[0].len() {
            buffer[last_row][x] = '#';
        }
    }

    simulate(&mut buffer, start_x, start_y, animate);

    if !animate {
        println!("Boundary: ({}, {}) ({}, {})", min_x, min_y, max_x, max_y);
        print(&buffer, None);
    }

    buffer
        .iter()
        .map(|row| row.iter().filter(|ch| ch == &&'o').count())
        .sum()
}

pub fn run(content: String, animate: bool) -> (String, String) {
    let lines: Vec<Vec<(i32, i32)>> = content
        .trim()
        .split("\n")
        .map(|line| -> Vec<(i32, i32)> {
            line.trim()
                .split(" -> ")
                .into_iter()
                .map(|pair| -> (i32, i32) {
                    let pair = pair
                        .split(",")
                        .into_iter()
                        .map(|value| value.parse::<i32>().unwrap());
                    pair.collect_tuple().unwrap()
                })
                .collect()
        })
        .collect();

    let all_points = lines.iter().flatten().collect_vec();
    let all_x = all_points.iter().map(|(left, _)| left).collect_vec();
    let all_y = all_points.iter().map(|(_, right)| right).collect_vec();

    let max_x: i32 = i32::max(SAND_ORIGIN_X, **all_x.iter().max().unwrap());
    let min_x: i32 = i32::min(SAND_ORIGIN_X, **all_x.iter().min().unwrap());
    let max_y: i32 = i32::max(SAND_ORIGIN_Y, **all_y.iter().max().unwrap());
    let min_y: i32 = i32::min(SAND_ORIGIN_Y, **all_y.iter().min().unwrap());

    let max_y2: i32 = max_y + 2;
    let width_y2 = max_y2 - min_y + 1;
    let max_x2: i32 = i32::max(max_x, SAND_ORIGIN_X + width_y2 + 1);
    let min_x2: i32 = i32::min(min_x, SAND_ORIGIN_X - width_y2 - 1);

    let part1: usize = solve(&lines, max_x, min_x, max_y, min_y, false, animate);
    let part2 = solve(&lines, max_x2, min_x2, max_y2, min_y, true, animate);

    println!("day14 part1: {}", part1);
    println!("day14 part2: {}", part2);
    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day14_example_test() {
        let input = "
            498,4 -> 498,6 -> 496,6
            503,4 -> 502,4 -> 502,9 -> 494,9
            "
        .to_string();

        let (part1, part2) = run(input, false);
        assert_eq!(part1, "24");
        assert_eq!(part2, "93");
    }

    #[test]
    fn day14_test() {
        let (part1, part2) = day14();
        assert_eq!(part1, "644");
        assert_eq!(part2, "27324");
    }
}
