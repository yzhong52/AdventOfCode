use super::super::helpers::parser::*;
use crate::helpers::models::_Point;
use std::collections::{HashMap, VecDeque};

const NOT_VISITED: i32 = -1;

fn search_portal(maze: &Vec<Vec<char>>) -> HashMap<[char; 2], Vec<_Point<usize>>> {
    let mut portals: HashMap<[char; 2], Vec<_Point<usize>>> = HashMap::new();
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
                    portals.entry(key).or_insert(vec![]).push(_Point {
                        x: portal_pos.x as usize,
                        y: portal_pos.y as usize,
                    });
                }
            }
        }
    }
    portals
}

pub fn part1(input: Input<Vec<Vec<char>>>) -> Answer<i32> {
    let maze = &input.data;
    let named_portals = search_portal(maze);

    let mut portals = HashMap::new();
    for pair in named_portals.values() {
        if pair.len() == 2 {
            portals.insert(pair[0].clone(), pair[1].clone());
            portals.insert(pair[1].clone(), pair[0].clone());
        }
    }

    for (pk, pv) in &named_portals {
        println!("{:?}: {:?}", pk, pv);
    }

    let max_x = maze.len();
    let max_y = maze[0].len();

    let mut distance = vec![vec![NOT_VISITED; max_y]; max_x];

    let mut queue: VecDeque<_Point<usize>> = VecDeque::new();
    let start = named_portals.get(&['A', 'A']).unwrap().first().unwrap();
    let end = named_portals.get(&['Z', 'Z']).unwrap().first().unwrap();
    queue.push_back(start.clone());
    distance[start.x][start.y] = 0;

    let mut total_distance = 0;
    while let Some(current) = queue.pop_front() {
        println!("Visiting {:?} -> {:?}", current, distance[current.x][current.y]);
        if current == *end {
            total_distance = distance[current.x][current.y];
            break
        }

        let neighbours  = current.neighbours4(max_x, max_y);
        for n in neighbours {
            if maze[n.x][n.y] == '.' && distance[n.x][n.y] == NOT_VISITED {
                distance[n.x][n.y] = distance[current.x][current.y] + 1;
                queue.push_back(n.clone())
            }
        }

        if let Some(warp_position) = portals.get(&current) {
            if distance[warp_position.x][warp_position.y] == NOT_VISITED {
                distance[warp_position.x][warp_position.y] = distance[current.x][current.y] + 1;
                queue.push_back(warp_position.clone())
            }
        }
    }

    Answer { question: input.question, result: total_distance }
}

pub fn part2(input: Input<Vec<Vec<char>>>) -> Answer<i32> {
    Answer { question: input.question, result: 0 }
}
