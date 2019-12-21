use super::super::helpers::parser::*;
use crate::helpers::models::_Point;
use std::collections::HashMap;
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

fn max_key(data: &Vec<Vec<char>>) -> usize {
    let mut result = 0;
    for i in 0..data.len() {
        for j in 0..data[0].len() {
            if data[i][j] >= 'a' && data[i][j] <= 'z' {
                result = result.max(data[i][j] as usize - 'a' as usize + 1);
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

fn dfs(
    data: &Vec<Vec<char>>,
    keys: Vec<i32>,
    current_position: _Point<usize>,
    visited: &mut HashMap<Visited, i32>,
    depth: i32,
    result: &mut i32,
) {
    let visited_key = Visited { position: current_position.clone(), keys: keys.clone() };

    if keys.iter().all(|x| x > &0) {
        if depth < *result {
            println!("Update result: {}", depth);
            *result = depth;
            return;
        }
    }

    if depth >= *result {
        return;
    }

    if visited.contains_key(&visited_key) && *visited.get(&visited_key).unwrap() <= depth {
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
    let max_key = max_key(&input.data);
    let keys: Vec<i32> = vec![0; max_key];
    let mut result: i32 = std::i32::MAX;
    dfs(
        &input.data,
        keys,
        entrance,
        visited.borrow_mut(),
        0,
        &mut result,
    );
    Answer { question: input.question, result }
}


pub fn part2(input: Input<Vec<Vec<char>>>) -> Answer<i32> {
    Answer { question: input.question, result: 0 }
}
