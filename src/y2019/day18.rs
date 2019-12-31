use super::super::helpers::parser::*;
use crate::helpers::models::_Point;
use std::collections::HashMap;
use std::borrow::BorrowMut;
use std::ops::Range;

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

fn key_for(door: char) -> i32 {
    assert!(is_door(door));
    1 << (door as i32 - 'A' as i32 + 'a' as i32)
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

#[derive(Eq, PartialEq, Hash, Debug)]
struct Visited {
    position: _Point<usize>,
    keys: i32,
}

fn all_keys_found(keys: &i32, all_keys: &i32) -> bool {
    keys == all_keys
}

fn dfs(
    data: &Vec<Vec<char>>,
    keys: i32,
    all_keys: &i32,
    current_position: _Point<usize>,
    visited: &mut HashMap<Visited, i32>,
    depth: i32,
    result: &mut i32,
) {
    let visited_key = Visited { position: current_position.clone(), keys: keys };

    if all_keys_found(&keys, all_keys) {
        if depth < *result {
            println!("Found a better path with {} steps.", depth);
            *result = depth;
        }
        return;
    } else if depth >= *result {
        return;
    } else if visited.contains_key(&visited_key) && *visited.get(&visited_key).unwrap() <= depth {
        return;
    }
    visited.insert(visited_key, depth);

    let max_x = data.len();
    let max_y = data[0].len();

    let neighbours4 = current_position.neighbours4(max_x, max_y);

    for new_position in neighbours4 {
        match data[new_position.x][new_position.y] {
            EMPTY | ENTRANCE => {
                dfs(
                    data,
                    keys.clone(),
                    all_keys,
                    new_position,
                    visited,
                    depth + 1,
                    result,
                );
            }
            value if is_key(value) => {
                // Pick up the key
                let new_keys = keys | to_key(value);
                dfs(
                    data,
                    new_keys,
                    all_keys,
                    new_position,
                    visited,
                    depth + 1,
                    result,
                );
            }
            value if is_door(value) => {
                if open_door(keys, value) {
                    dfs(
                        data,
                        keys.clone(),
                        all_keys,
                        new_position,
                        visited,
                        depth + 1,
                        result,
                    );
                }
            }
            WALL => (), // Oops, hit the wall
            value => {
                unimplemented!("Unknown land: {}", value)
            }
        }
    }
}

pub fn part1(input: Input<Vec<Vec<char>>>) -> Answer<i32> {
    let entrance = find_entrance(&input.data);
    let mut visited: HashMap<Visited, i32> = HashMap::new();
    let all_keys = all_keys(&input.data);
    let keys = 0;
    let mut result: i32 = std::i32::MAX;
    dfs(
        &input.data,
        keys,
        &all_keys,
        entrance,
        visited.borrow_mut(),
        0,
        &mut result,
    );
    Answer { question: input.question, result }
}

struct Quadrant {
    x_range: Range<usize>,
    y_range: Range<usize>,
    entrance: _Point<usize>,
}

pub fn part2(input: Input<Vec<Vec<char>>>) -> Answer<i32> {
    let entrance = find_entrance(&input.data);

    let max_x = &input.data.len();
    let max_y = &input.data[0].len();
    let quadrants = [
        Quadrant {
            x_range: 0..entrance.x,
            y_range: 0..entrance.y,
            entrance: _Point { x: entrance.x - 1, y: entrance.y - 1 },
        },
        Quadrant {
            x_range: 0..entrance.x,
            y_range: entrance.y + 1..*max_y,
            entrance: _Point { x: entrance.x - 1, y: 0 },
        },
        Quadrant {
            x_range: entrance.x + 1..*max_x,
            y_range: 0..entrance.y,
            entrance: _Point { x: 0, y: entrance.y - 1 },
        },
        Quadrant {
            x_range: entrance.x + 1..*max_x,
            y_range: entrance.y + 1..*max_y,
            entrance: _Point { x: 0, y: 0 },
        }
    ];

    let mut final_result = 0;
    for quadrant in quadrants.iter() {
        let mut data = vec![];
        for i in quadrant.x_range.clone() {
            let mut row = vec![];
            for j in quadrant.y_range.clone() {
                row.push(input.data[i][j]);
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

        let mut visited: HashMap<Visited, i32> = HashMap::new();
        let keys = 0;
        let mut result: i32 = std::i32::MAX;
        dfs(
            &data,
            keys,
            &all_keys,
            quadrant.entrance.clone(),
            visited.borrow_mut(),
            0,
            &mut result,
        );

        final_result += result;
    }

    Answer { question: input.question, result: final_result }
}
