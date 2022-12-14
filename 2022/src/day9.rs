use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

pub fn day9() -> (String, String) {
    let content = fs::read_to_string("input/day9").unwrap();
    run(content)
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

#[allow(dead_code)]
fn printstd(pos: &Vec<Pos>) {
    let max_x = pos.iter().map(|p| p.x).max().unwrap() as i32;
    let max_y = pos.iter().map(|p| p.y).max().unwrap() as i32;

    let min_x = pos.iter().map(|p| p.x).min().unwrap() as i32;
    let min_y = pos.iter().map(|p| p.y).min().unwrap() as i32;

    let width_x = (max_x - min_x + 1) as usize;
    let width_y = (max_y - min_y + 1) as usize;

    let mut buffer = vec![vec!['.'; width_y]; width_x];

    for (i, p) in pos.iter().enumerate() {
        let y = (p.y - min_y) as usize;
        let x = (p.x - min_x) as usize;
        if buffer[x][y] == '.' {
            if i == 0 {
                buffer[x][y] = 'H';
            } else {
                buffer[x][y] = ('0' as u8 + i as u8) as char;
            }
        }
    }

    let buffer = buffer
        .iter()
        .map(|row| row.iter().collect::<String>())
        .join("\n");
    println!("--\n{}", buffer);
}

fn move_tails(pos: &mut Vec<Pos>) {
    for i in 0..pos.len() - 1 {
        let dx = pos[i + 1].x - pos[i].x;
        let dy = pos[i + 1].y - pos[i].y;

        if dx.abs() <= 1 && dy.abs() <= 1 {
            // no need to move
            break;
        }

        if dx.abs() == dy.abs() {
            // move diagonally
            let norm_dx = dx / dx.abs();
            let norm_dy = dy / dy.abs();
            pos[i + 1].y = pos[i].y + norm_dy;
            pos[i + 1].x = pos[i].x + norm_dx;
        } else if dx.abs() > dy.abs() {
            let norm_dx = dx / dx.abs();
            pos[i + 1].x = pos[i].x + norm_dx;
            pos[i + 1].y = pos[i].y;
        } else {
            let norm_dy = dy / dy.abs();
            pos[i + 1].x = pos[i].x;
            pos[i + 1].y = pos[i].y + norm_dy;
        }
    }
}

fn run(content: String) -> (String, String) {
    let lines: Vec<&str> = content.trim().split("\n").map(|line| line.trim()).collect();

    // +----->
    // |    y
    // |
    // | x
    let mut pos: Vec<Pos> = vec![Pos { x: 0, y: 0 }; 10];

    let mut part1_visited = HashSet::from([Pos { x: 0, y: 0 }]);
    let mut part2_visited = HashSet::from([Pos { x: 0, y: 0 }]);
    for line in lines {
        let mut split = line.split(" ");
        let dir = split.next().unwrap();
        let steps = split.next().unwrap().parse::<u32>().unwrap();

        if dir == "R" {
            for _ in 0..steps {
                pos[0].y += 1;

                move_tails(&mut pos);

                part1_visited.insert(pos[1].clone());
                part2_visited.insert(pos.last().unwrap().clone());
            }
        } else if dir == "L" {
            for _ in 0..steps {
                pos[0].y -= 1;

                move_tails(&mut pos);
                part1_visited.insert(pos[1].clone());
                part2_visited.insert(pos.last().unwrap().clone());
            }
        } else if dir == "U" {
            for _ in 0..steps {
                pos[0].x -= 1;

                move_tails(&mut pos);
                part1_visited.insert(pos[1].clone());
                part2_visited.insert(pos.last().unwrap().clone());
            }
        } else if dir == "D" {
            for _ in 0..steps {
                pos[0].x += 1;

                move_tails(&mut pos);
                part1_visited.insert(pos[1].clone());
                part2_visited.insert(pos.last().unwrap().clone());
            }
        }
        // Uncomment to print states
        // printstd(&pos);
    }

    let part1 = part1_visited.len();
    let part2 = part2_visited.len();

    println!("day9 part1: {:?}", part1);
    println!("day9 part2: {:?}", part2);

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day9_test() {
        let (part1, part2) = day9();
        assert_eq!(part1, "6354");
        assert_eq!(part2, "2651");
    }

    #[test]
    fn day9_example_test() {
        let input = "
            R 4
            U 4
            L 3
            D 1
            R 4
            D 1
            L 5
            R 2"
        .to_string();

        let (part1, part2) = run(input);
        assert_eq!(part1, "13");
        assert_eq!(part2, "1");
    }
}
