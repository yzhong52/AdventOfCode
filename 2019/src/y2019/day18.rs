use super::super::helpers::parser::*;
use crate::helpers::models::_Point;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::ops::Range;
use crate::helpers::puzzle::Puzzle;

const ENTRANCE: char = '@';
const WALL: char = '#';
const EMPTY: char = '.';

fn find_entrance(data: &Vec<Vec<char>>) -> _Point<usize> {
    for i in 0..data.len() {
        for j in 0..data[0].len() {
            if data[i][j] == ENTRANCE {
                return _Point { x: i, y: j };
            }
        }
    }
    _Point::origin()
}

fn is_key(c: char) -> bool {
    c.is_ascii_lowercase()
}

fn is_door(c: char) -> bool {
    c.is_ascii_uppercase()
}

fn to_key(c: char) -> i32 {
    1 << (c as i32 - 'a' as i32)
}

fn open_door(keys: i32, door: char) -> bool {
    (keys & 1 << (door as i32 - 'A' as i32)) > 0
}

fn all_keys(data: &Vec<Vec<char>>) -> i32 {
    let mut result = 0;
    for row in data {
        for cell in row {
            if is_key(*cell) {
                result |= to_key(*cell);
            }
        }
    }
    result
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct VisitState {
    position: _Point<usize>,
    keys: i32,
}

struct Quadrant {
    x_range: Range<usize>,
    y_range: Range<usize>,
    entrance: _Point<usize>,
}

fn bfs(data: &Vec<Vec<char>>, all_keys: &i32, entrance: _Point<usize>) -> usize {
    let max_x = data.len();
    let max_y = data[0].len();

    let start = VisitState { position: entrance.clone(), keys: 0 };

    let mut visiting_queue: VecDeque<(VisitState, usize)> = VecDeque::new();
    visiting_queue.push_back((start.clone(), 0));

    let mut visited = HashSet::new();
    visited.insert(start);

    while !visiting_queue.is_empty() {
        let (visiting_state, distance) = visiting_queue.pop_front().unwrap();

        if visiting_state.keys == *all_keys {
            return distance;
        }

        let new_positions = visiting_state.position.neighbours4(max_x, max_y);
        for next_position in new_positions {
            match data[next_position.x][next_position.y] {
                // Oops, hit a wall, do nothing
                WALL => (),
                // Cannot open the door
                value if is_door(value) && !open_door(visiting_state.keys, value) => (),
                // Move forward
                value => {
                    let next_keys: i32;
                    if is_key(value) {
                        next_keys = visiting_state.keys | to_key(value)
                    } else {
                        next_keys = visiting_state.keys
                    }
                    let new_visited = VisitState { position: next_position, keys: next_keys };
                    if !visited.contains(&new_visited) {
                        visiting_queue.push_back((new_visited.clone(), distance + 1));
                        visited.insert(new_visited);
                    }
                }
            }
        }
    }

    unimplemented!("Unable to open all doors!")
}

pub struct Day18 {}

impl Puzzle<Vec<Vec<char>>, usize> for Day18 {
    fn day(&self) -> i8 {
        18
    }

    fn parser(&self) -> fn(String) -> Vec<Vec<char>> {
        read_grid
    }

    fn part1(&self, input: &Vec<Vec<char>>) -> usize {
        let entrance = find_entrance(input);
        let all_keys = all_keys(input);
        let result = bfs(
            input,
            &all_keys,
            entrance,
        );
        result
    }

    fn part2(&self, input: &Vec<Vec<char>>) -> usize {
        let entrance = find_entrance(input);

        let max_x = input.len();
        let max_y = input[0].len();
        let quadrants = [
            Quadrant {
                x_range: 0..entrance.x,
                y_range: 0..entrance.y,
                entrance: _Point { x: entrance.x - 1, y: entrance.y - 1 },
            },
            Quadrant {
                x_range: 0..entrance.x,
                y_range: entrance.y + 1..max_y,
                entrance: _Point { x: entrance.x - 1, y: 0 },
            },
            Quadrant {
                x_range: entrance.x + 1..max_x,
                y_range: 0..entrance.y,
                entrance: _Point { x: 0, y: entrance.y - 1 },
            },
            Quadrant {
                x_range: entrance.x + 1..max_x,
                y_range: entrance.y + 1..max_y,
                entrance: _Point { x: 0, y: 0 },
            }
        ];

        let mut final_result = 0;
        for quadrant in quadrants.iter() {
            let mut data = vec![];
            for i in quadrant.x_range.clone() {
                let mut row = vec![];
                for j in quadrant.y_range.clone() {
                    row.push(input[i][j]);
                }
                data.push(row);
            }
            data[quadrant.entrance.x][quadrant.entrance.y] = ENTRANCE;

            let all_keys = all_keys(&data);

            // For all doors that are in the current quadrant, if there isn't a key, let's just remove it.
            for row in &mut data {
                for cell in row {
                    if is_door(*cell) && !open_door(all_keys, *cell) {
                        *cell = EMPTY;
                    }
                }
            }

            let result = bfs(
                &data,
                &all_keys,
                quadrant.entrance.clone(),
            );

            final_result += result;
        }

        final_result
    }
}
