use super::super::helpers::parser::*;
use crate::helpers::models::_Point;

pub trait Letter {
    fn is_uppercase(&self) -> bool;
}

impl Letter for char {
    fn is_uppercase(&self) -> bool {
        return *self >= 'A' && *self <= 'Z';
    }
}

fn search_portal(maze: &Vec<Vec<char>>) {
    let max_x = maze.len();
    let max_y = maze[0].len();
    for i in 0..max_x {
        for j in 0..max_y {
            if maze[i][j].is_ascii_uppercase() {
                let p1 = _Point { x: i, y: j };
                let neighbours = p1.neighbours4(max_x, max_y);
                let p2 = neighbours
                    .iter()
                    .filter(|pos| maze[pos.x][pos.y].is_ascii_uppercase())
                    .nth(0)
                    .unwrap();

                let side1 = _Point {
                    x: p1.x as i32 + p1.x as i32 - p2.x as i32,
                    y: p1.y as i32 + p1.y as i32 - p2.y as i32,
                };

                let side2 = _Point {
                    x: p2.x as i32 + p2.x as i32 - p1.x as i32,
                    y: p2.y as i32 + p2.y as i32 - p1.y as i32,
                };

                let both_sides = [side1, side2];

                let portal_pos = both_sides.iter()
                    .filter(|p| {
                        p.is_valid(max_x as i32, max_y as i32) && maze[p.x as usize][p.y as usize] == '.'
                    })
                    .nth(0)
                    .unwrap();

                println!("{}{}: {:?}", maze[p1.x][p1.y], maze[p2.x][p2.y], portal_pos);
            }
        }
    }
}

pub fn part1(input: Input<Vec<Vec<char>>>) -> Answer<usize> {
    search_portal(&input.data);
    Answer { question: input.question, result: 0 }
}

pub fn part2(input: Input<Vec<Vec<char>>>) -> Answer<i32> {
    Answer { question: input.question, result: 0 }
}
