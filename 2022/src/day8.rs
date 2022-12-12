use itertools::Itertools;
use std::fs;

pub fn day8() -> (String, String) {
    let content = fs::read_to_string("input/day8").unwrap();
    run(content)
}

fn run(content: String) -> (String, String) {
    let lines: Vec<&str> = content.trim().split("\n").collect();
    let grid: Vec<Vec<i32>> = lines
        .iter()
        .map(|line| {
            line.trim()
                .chars()
                .into_iter()
                .map(|char| char as i32 - '0' as i32)
                .collect()
        })
        .collect();

    let mut counters: Vec<Vec<u32>> = vec![vec![0; grid[0].len()]; grid.len()];

    for i in 0..grid.len() {
        let mut height1 = -1;
        let mut height2 = -1;
        for j in 0..grid[i].len() {
            if grid[i][j] > height1 {
                height1 = grid[i][j];
                counters[i][j] = 1;
            }
            let k = grid[i].len() - 1 - j;
            if grid[i][k] > height2 {
                height2 = grid[i][k];
                counters[i][k] = 1;
            }
        }
    }
    for j in 0..grid[0].len() {
        let mut height1 = -1;
        let mut height2 = -1;
        for i in 0..grid.len() {
            if grid[i][j] > height1 {
                height1 = grid[i][j];
                counters[i][j] = 1;
            }
            let k = grid.len() - 1 - i;
            if grid[k][j] > height2 {
                height2 = grid[k][j];
                counters[k][j] = 1;
            }
        }
    }

    let part1: u32 = counters.iter().map(|row| row.iter().sum::<u32>()).sum();

    let mut left: Vec<Vec<usize>> = vec![vec![0; grid[0].len()]; grid.len()];
    let mut right: Vec<Vec<usize>> = vec![vec![0; grid[0].len()]; grid.len()];
    let mut up: Vec<Vec<usize>> = vec![vec![0; grid[0].len()]; grid.len()];
    let mut down: Vec<Vec<usize>> = vec![vec![0; grid[0].len()]; grid.len()];

    for i in 0..grid.len() {
        // left
        for j in 0..grid[i].len() {
            for k in j + 1..grid[i].len() {
                left[i][j] = k - j;
                if grid[i][k] >= grid[i][j] {
                    break;
                }
            }
        }
        // right
        for j in (0..grid[i].len()).rev() {
            for k in (0..j).rev() {
                right[i][j] = j - k;
                if grid[i][k] >= grid[i][j] {
                    break;
                }
            }
        }
    }
    for j in 0..grid[0].len() {
        // down
        for i in 0..grid.len() {
            for i2 in i + 1..grid.len() {
                down[i][j] = i2 - i;
                if grid[i2][j] >= grid[i][j] {
                    break;
                }
            }
        }
        // up
        for i in (0..grid.len()).rev() {
            for i1 in (0..i).rev() {
                up[i][j] = i - i1;
                if grid[i1][j] >= grid[i][j] {
                    break;
                }
            }
        }
    }

    let mut part2 = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            print!("{} ", left[i][j] * right[i][j] * up[i][j] * down[i][j]);
            part2 = usize::max(part2, left[i][j] * right[i][j] * up[i][j] * down[i][j])
        }
        println!("");
    }

    println!(
        "grid\n{}",
        grid.iter()
            .map(|row| row.iter().map(|num| num.to_string()).join(""))
            .join("\n")
    );

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day8_test() {
        let (part1, part2) = day8();
        assert_eq!(part1, "1818");
        assert_eq!(part2, "368368");
    }

    #[test]
    fn day8_example_test() {
        let input = "
            30373
            25512
            65332
            33549
            35390"
            .to_string();

        let (part1, part2) = run(input);
        assert_eq!(part1, "21");
        assert_eq!(part2, "8");
    }
}
