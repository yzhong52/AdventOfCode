use super::super::helpers::parser::*;
use crate::helpers::models::_Point;
use std::collections::{HashMap, VecDeque};

const NOT_VISITED: i32 = -1;
const ENTRY_PORTAL_NAME: [char; 2] = ['A', 'A'];
const EXIT_PORTAL_NAME: [char; 2] = ['Z', 'Z'];

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

fn index_portals(named_portals: &HashMap<[char; 2], Vec<_Point<usize>>>) -> HashMap<_Point<usize>, _Point<usize>> {
    let mut portals = HashMap::new();
    for pair in named_portals.values() {
        if pair.len() == 2 {
            portals.insert(pair[0].clone(), pair[1].clone());
            portals.insert(pair[1].clone(), pair[0].clone());
        }
    }
    portals
}

pub fn part1(input: Input<Vec<Vec<char>>>) -> Answer<i32> {
    let maze = &input.data;
    let named_portals: HashMap<[char; 2], Vec<_Point<usize>>> = search_portal(maze);
    let portals = index_portals(&named_portals);

    let max_x = maze.len();
    let max_y = maze[0].len();

    let mut distance = vec![vec![NOT_VISITED; max_y]; max_x];

    let mut queue: VecDeque<_Point<usize>> = VecDeque::new();
    let start = named_portals.get(&ENTRY_PORTAL_NAME).unwrap().first().unwrap();
    let end = named_portals.get(&EXIT_PORTAL_NAME).unwrap().first().unwrap();
    queue.push_back(start.clone());
    distance[start.x][start.y] = 0;

    let mut total_distance = 0;
    while let Some(current) = queue.pop_front() {
        if current == *end {
            total_distance = distance[current.x][current.y];
            break;
        }

        let neighbours = current.neighbours4(max_x, max_y);
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

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
struct RecursivePosition {
    position: _Point<usize>,
    level: usize,
}

pub fn part2(input: Input<Vec<Vec<char>>>) -> Answer<i32> {
    let maze = &input.data;
    let named_portals = search_portal(maze);
    let portals = index_portals(&named_portals);

    let max_x = maze.len();
    let max_y = maze[0].len();

    let mut visited: HashMap<RecursivePosition, i32> = HashMap::new();

    let mut queue: VecDeque<RecursivePosition> = VecDeque::new();
    let start_position = RecursivePosition {
        position: named_portals.get(&ENTRY_PORTAL_NAME).unwrap().first().unwrap().clone(),
        level: 0,
    };

    let end_position = RecursivePosition {
        position: named_portals.get(&EXIT_PORTAL_NAME).unwrap().first().unwrap().clone(),
        level: 0,
    };

    visited.insert(start_position.clone(), 0);
    queue.push_back(start_position);

    let mut total_distance = 0;
    while let Some(current) = queue.pop_front() {
        if current == end_position {
            total_distance = *visited.get(&current).unwrap();
            break;
        }

        let neighbours = current.position.neighbours4(max_x, max_y);
        for neighbour_position in neighbours {
            let recursive_neighbour_position = RecursivePosition {
                position: neighbour_position.clone(),
                level: current.level,
            };
            if maze[neighbour_position.x][neighbour_position.y] == '.' &&
                !visited.contains_key(&recursive_neighbour_position)
            {
                visited.insert(recursive_neighbour_position.clone(), *visited.get(&current).unwrap() + 1);
                queue.push_back(recursive_neighbour_position)
            }
        }

        if let Some(warp_position) = portals.get(&current.position) {
            if current.position.x == 2 || current.position.x == max_x - 3 ||
                current.position.y == 2 || current.position.y == max_y - 3 {
                if current.level > 0 {
                    let warp_recursive_position = RecursivePosition {
                        position: warp_position.clone(),
                        level: current.level - 1,
                    };
                    if !visited.contains_key(&warp_recursive_position) {
                        visited.insert(warp_recursive_position.clone(), *visited.get(&current).unwrap() + 1);
                        queue.push_back(warp_recursive_position)
                    }
                }
            } else {
                let warp_recursive_position = RecursivePosition {
                    position: warp_position.clone(),
                    level: current.level + 1,
                };
                if !visited.contains_key(&warp_recursive_position) {
                    visited.insert(warp_recursive_position.clone(), *visited.get(&current).unwrap() + 1);
                    queue.push_back(warp_recursive_position)
                }
            }
        }
    }

    Answer { question: input.question, result: total_distance }
}
