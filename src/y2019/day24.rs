use crate::helpers::parser::{Answer, Input};
use crate::helpers::utils::print_grid;
use std::collections::HashSet;
use crate::helpers::models::_Point;

const BUG: char = '#';
const NO_BUG: char = '.';
const UNKNOWN: char = '?';

fn evolution(count_bugs_nearby: usize, bug: char) -> char {
    if bug == BUG {
        if count_bugs_nearby != 1 {
            // A bug dies (becoming an empty space) unless there is exactly one bug
            // adjacent to it.
            NO_BUG
        } else {
            BUG
        }
    } else if bug == NO_BUG {
        if count_bugs_nearby == 1 || count_bugs_nearby == 2 {
            // An empty space becomes infested with a bug if exactly one or two bugs are
            // adjacent to it
            BUG
        } else {
            NO_BUG
        }
    } else {
        UNKNOWN
    }
}

pub fn part1(input: Input<Vec<Vec<char>>>) -> Answer<usize> {
    let mut current: Vec<Vec<char>> = input.data;

    let mut appeared: HashSet<Vec<Vec<char>>> = HashSet::new();
    let max_x = current.len();
    let max_y = current[0].len();
    while !appeared.contains(&current) {
        print_grid(&current);

        appeared.insert(current.clone());
        let mut next_state = vec![vec![UNKNOWN; max_y]; max_x];

        for x in 0..max_x as i32 {
            for y in 0..max_y as i32 {
                let point = _Point { x, y };
                let count_bugs_nearby = point.neighbours4(max_x as i32, max_y as i32)
                    .iter()
                    .filter(|p| current[p.x as usize][p.y as usize] == BUG)
                    .count();

                if current[x as usize][y as usize] == BUG {
                    if count_bugs_nearby != 1 {
                        // A bug dies (becoming an empty space) unless there is exactly one bug
                        // adjacent to it.
                        next_state[x as usize][y as usize] = NO_BUG
                    } else {
                        next_state[x as usize][y as usize] = BUG
                    }
                } else if current[x as usize][y as usize] == NO_BUG {
                    if count_bugs_nearby == 1 || count_bugs_nearby == 2 {
                        // An empty space becomes infested with a bug if exactly one or two bugs are
                        // adjacent to it
                        next_state[x as usize][y as usize] = BUG
                    } else {
                        next_state[x as usize][y as usize] = NO_BUG
                    }
                }
            }
        }

        current = next_state;
    }

    print_grid(&current);

    let mut result = 0;
    let mut biodiversity_rating = 1;
    for i in 0..max_x {
        for j in 0..max_y {
            if current[i][j] == BUG {
                result += biodiversity_rating;
            }
            biodiversity_rating <<= 1;
        }
    }
    Answer { question: input.question, result }
}

fn has_bug(data: &Vec<Vec<char>>) -> bool {
    for row in data {
        for cell in row {
            if *cell == BUG {
                return true;
            }
        }
    }
    false
}


fn print_state(state: &Vec<Vec<Vec<char>>>) {
    for layer in state {
        print_grid(layer)
    }
}

pub fn part2(input: Input<Vec<Vec<char>>>) -> Answer<usize> {
    let max_x = 5;
    let max_y = 5;

    let mut current = vec![input.data];

    for _ in 0..200 {
        // Pad two more layers on both side
        let mut previous_state: Vec<Vec<Vec<char>>> = vec![];
        previous_state.push(vec![vec![NO_BUG; max_y]; max_x]);
        previous_state.push(vec![vec![NO_BUG; max_y]; max_x]);
        previous_state.append(current.as_mut());
        previous_state.push(vec![vec![NO_BUG; max_y]; max_x]);
        previous_state.push(vec![vec![NO_BUG; max_y]; max_x]);

        let mut next_state = vec![vec![vec![UNKNOWN; max_y]; max_x]; previous_state.len()];

        for layer in 1..next_state.len() - 1 {
            for x in 0..max_x {
                for y in 0..max_y {
                    if (x, y) == (2, 2) {
                        // skip the middle grid
                        continue;
                    }

                    let mut neighbours = vec![];
                    // Top
                    if x == 0 {
                        neighbours.push((layer + 1, 1, 2))
                    } else if (x, y) == (3, 2) {
                        for y in 0..max_y {
                            neighbours.push((layer - 1, 4, y))
                        }
                    } else {
                        // same layer
                        neighbours.push((layer, x - 1, y))
                    }
                    // Bottom
                    if x == 4 {
                        neighbours.push((layer + 1, 3, 2))
                    } else if (x, y) == (1, 2) {
                        for y in 0..max_y {
                            neighbours.push((layer - 1, 0, y))
                        }
                    } else {
                        // same layer
                        neighbours.push((layer, x + 1, y))
                    }
                    // Left
                    if y == 0 {
                        // outer layer
                        neighbours.push((layer + 1, 2, 1));
                    } else if (x, y) == (2, 3) {
                        // inner layer
                        for x in 0..max_x {
                            neighbours.push((layer - 1, x, 4));
                        }
                    } else {
                        // same layer
                        neighbours.push((layer, x, y - 1));
                    }
                    // Right
                    if y == 4 {
                        // outer layer
                        neighbours.push((layer + 1, 2, 3));
                    } else if (x, y) == (2, 1) {
                        // inner layer
                        for x in 0..max_x {
                            neighbours.push((layer - 1, x, 0));
                        }
                    } else {
                        neighbours.push((layer, x, y + 1));
                    }

                    let count_bugs_nearby = neighbours
                        .iter()
                        .filter(|n| previous_state[n.0][n.1][n.2] == BUG)
                        .count();

                    next_state[layer][x][y] = evolution(count_bugs_nearby, previous_state[layer][x][y]);
                }
            }
        }

        let from = next_state.iter().position(has_bug).unwrap();
        let to = next_state.len() - 1 - next_state.iter().rev().position(has_bug).unwrap();

        current = next_state[from ..= to].to_vec();
    }

    print_state(&current);

    let result = current.iter()
        .flatten()
        .flatten()
        .filter(|p| **p == BUG)
        .count();

    Answer { question: input.question, result }
}
