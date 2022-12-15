use std::collections::VecDeque;
use std::{fs, vec};

pub fn day12() -> (String, String) {
    let content = fs::read_to_string("input/day12").unwrap();
    run(content)
}

const SOURCE: char = 'S';
const DESTINATION: char = 'E';

fn run(content: String) -> (String, String) {
    let mut grid: Vec<Vec<char>> = content
        .trim()
        .split("\n")
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect();

    let mut src_x: usize = 0;
    let mut src_y: usize = 0;
    let mut dst_x: usize = 0;
    let mut dst_y: usize = 0;
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y] == SOURCE {
                src_x = x as usize;
                src_y = y as usize;
            } else if grid[x][y] == DESTINATION {
                dst_x = x as usize;
                dst_y = y as usize;
            }
        }
    }

    grid[src_x][src_y] = 'a';
    grid[dst_x][dst_y] = 'z';

    const NOT_VISITED: i32 = -1;
    let mut visited_steps = vec![vec![NOT_VISITED; grid[0].len()]; grid.len()];
    let mut visiting = VecDeque::from([(src_x, src_y)]);

    visited_steps[src_x as usize][src_y as usize] = 0;

    while visiting.len() > 0 {
        let (current_x, current_y) = visiting.pop_front().unwrap();

        for (dx, dy) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let x2 = dx + current_x as i32;
            let y2 = dy + current_y as i32;
            if !(0 <= x2 && x2 < grid.len() as i32 && 0 <= y2 && y2 < grid[0].len() as i32) {
                // out of grid
                continue;
            }

            if visited_steps[x2 as usize][y2 as usize] != NOT_VISITED {
                // has already visited
                continue;
            }

            if grid[x2 as usize][y2 as usize] as u32
                <= grid[current_x as usize][current_y as usize] as u32 + 1
            {
                // height only increased by 1
                visiting.push_back((x2 as usize, y2 as usize));
                visited_steps[x2 as usize][y2 as usize] =
                    visited_steps[current_x as usize][current_y as usize] + 1;
            }
        }
    }

    // destination
    let part1 = visited_steps[dst_x as usize][dst_y as usize];

    let mut visited_steps = vec![vec![NOT_VISITED; grid[0].len()]; grid.len()];
    let mut visiting = VecDeque::from([(dst_x, dst_y)]);

    visited_steps[dst_x as usize][dst_y as usize] = 0;
    let mut part2 = 0;
    while visiting.len() > 0 && part2 == 0 {
        let (current_x, current_y) = visiting.pop_front().unwrap();

        for (dx, dy) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let x2 = dx + current_x as i32;
            let y2 = dy + current_y as i32;
            if !(0 <= x2 && x2 < grid.len() as i32 && 0 <= y2 && y2 < grid[0].len() as i32) {
                // out of grid
                continue;
            }

            if visited_steps[x2 as usize][y2 as usize] != NOT_VISITED {
                // has already visited
                continue;
            }

            if grid[x2 as usize][y2 as usize] as u32
                >= grid[current_x as usize][current_y as usize] as u32 - 1
            {
                // height only increased by 1
                visiting.push_back((x2 as usize, y2 as usize));
                visited_steps[x2 as usize][y2 as usize] =
                    visited_steps[current_x as usize][current_y as usize] + 1;

                if grid[x2 as usize][y2 as usize] == 'a' {
                    // Found it
                    part2 = visited_steps[x2 as usize][y2 as usize];
                    break;
                }
            }
        }
    }

    println!("day12 part1: {}", part1);
    println!("day12 part2: {}", part2);

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day12_test() {
        let (part1, part2) = day12();
        assert_eq!(part1, "462");
        assert_eq!(part2, "451");
    }

    #[test]
    fn day12_example_test() {
        let input = "
            Sabqponm
            abcryxxl
            accszExk
            acctuvwj
            abdefghi"
            .to_string();

        let (part1, part2) = run(input);
        assert_eq!(part1, "31");
        assert_eq!(part2, "29");
    }
}
