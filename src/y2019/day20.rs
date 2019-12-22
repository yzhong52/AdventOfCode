use super::super::helpers::parser::*;
use crate::helpers::models::_Point;
use std::collections::HashMap;

fn search_portal(maze: &Vec<Vec<char>>) -> HashMap<[char; 2], Vec<_Point<i32>>> {
    let mut portals: HashMap<[char; 2], Vec<_Point<i32>>> = HashMap::new();
    let max_x = maze.len();
    let max_y = maze[0].len();
    for i in 0..max_x {
        for j in 0..max_y {
            if maze[i][j].is_ascii_uppercase() {
                let p1 = _Point { x: i, y: j };

                let neighbours = [
                    _Point { x: i + 1, y: j },
                    _Point { x: i, y: j + 1 }
                ];

                let p2 = neighbours
                    .iter()
                    .filter(|pos| {
                        pos.is_valid(max_x, max_y) && maze[pos.x][pos.y].is_ascii_uppercase()
                    })
                    .nth(0);

                if let Some(p2) = p2 {
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


                    let key = [maze[p1.x][p1.y], maze[p2.x][p2.y]];
                    portals.entry(key).or_insert(vec![]).push(portal_pos.clone());
                }
            }
        }
    }
    portals
}

pub fn part1(input: Input<Vec<Vec<char>>>) -> Answer<usize> {
    let portals = search_portal(&input.data);

    for (pk, pv) in portals {
        println!("{:?}: {:?}", pk, pv);
    }
    Answer { question: input.question, result: 0 }
}

pub fn part2(input: Input<Vec<Vec<char>>>) -> Answer<i32> {
    Answer { question: input.question, result: 0 }
}
