use super::super::helpers::parser::*;
use crate::helpers::models::_Point;
use std::collections::{HashMap, HashSet};
use std::borrow::BorrowMut;

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

fn all_keys(data: &Vec<Vec<char>>) -> HashSet<char> {
    let mut result = HashSet::new();
    for i in 0..data.len() {
        for j in 0..data[0].len() {
            if data[i][j] >= 'a' && data[i][j] <= 'z' {
                result.insert(data[i][j]);
            }
        }
    }
    result
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Visited {
    position: _Point<usize>,
    keys: Vec<i32>,
}

fn all_keys_found(
    keys: &Vec<i32>,
    all_keys: &HashSet<char>) -> bool {
    for key in all_keys {
        if keys[*key as u8 as usize - 'a' as u8 as usize] == 0 {
            return false;
        }
    }
    return true;
}

fn dfs(
    data: &Vec<Vec<char>>,
    keys: Vec<i32>,
    all_keys: &HashSet<char>,
    current_position: _Point<usize>,
    visited: &mut HashMap<Visited, i32>,
    depth: i32,
    result: &mut i32,
) {
    let visited_key = Visited { position: current_position.clone(), keys: keys.clone() };

    if all_keys_found(&keys, all_keys) {
        if depth < *result {
            println!("Update result: {}", depth);
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
            value if 'a' <= value && value <= 'z' => {
                // Pick up the key
                let mut new_keys = keys.clone();
                new_keys[value as usize - 'a' as usize] = 1;
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
            value if 'A' <= value && value <= 'Z' => {
                if keys[value as usize - 'A' as usize] > 0 {
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
    let keys: Vec<i32> = vec![0; 26];
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

pub fn part2(input: Input<Vec<Vec<char>>>) -> Answer<i32> {
    let entrance = find_entrance(&input.data);

    let max_x = &input.data.len();
    let max_y = &input.data[0].len();
    let quadrant = [
        (
            0..entrance.x,
            0..entrance.y,
            _Point { x: entrance.x - 1, y: entrance.y - 1 }
        ),
        (
            0..entrance.x,
            entrance.y + 1..*max_y,
            _Point { x: entrance.x - 1, y: 0 }
        ),
        (
            entrance.x + 1..*max_x,
            0..entrance.y,
            _Point { x: 0, y: entrance.y - 1 }
        ),
        (
            entrance.x + 1..*max_x,
            entrance.y + 1..*max_y,
            _Point { x: 0, y: 0 }
        ),
    ];

    let mut final_result = 0;
    for (x_range, y_range, entrance) in quadrant.iter() {
        let mut data = vec![];

        for i in x_range.clone() {
            let mut row = vec![];
            for j in y_range.clone() {
                row.push(input.data[i][j]);
            }
            data.push(row);
        }

        data[entrance.x][entrance.y] = ENTRANCE;

        let all_keys = all_keys(&data);

        // For all doors that are in the current quadrant, if there isn't a key, let's just remove it.
        for i in 0..data.len() {
            for j in 0..data[0].len() {
                if data[i][j] >= 'A' && data[i][j] <= 'Z' {
                    let key = (data[i][j] as u8 - 'A' as u8 + 'a' as u8) as char;
                    if !all_keys.contains(&key) {
                        data[i][j] = EMPTY;
                    }
                }
            }
        }

        let mut visited: HashMap<Visited, i32> = HashMap::new();
        let keys: Vec<i32> = vec![0; 26];
        let mut result: i32 = std::i32::MAX;
        dfs(
            &data,
            keys,
            &all_keys,
            entrance.clone(),
            visited.borrow_mut(),
            0,
            &mut result,
        );

        final_result += result;
    }

    Answer { question: input.question, result: final_result }
}
