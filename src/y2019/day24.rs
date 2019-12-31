use crate::helpers::parser::{Answer, Input};
use crate::helpers::utils::print_grid;
use std::collections::HashSet;
use crate::helpers::models::_Point;

const BUG: char = '#';
const NO_BUG: char = '.';
const UNKNOWN: char = '?';

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

pub fn part2(input: Input<Vec<Vec<char>>>) -> Answer<usize> {
    let mut initial: Vec<Vec<char>> = input.data;

    let max_x = initial.len();
    let max_y = initial[0].len();

    let mut current = vec![initial];

    for _ in 0..200 {
        // Pad two more layers on both side
        let mut previous_state = vec![vec![vec![]]];
        previous_state.push(vec![vec![NO_BUG; max_y]; max_x]);
        previous_state.push(vec![vec![NO_BUG; max_y]; max_x]);
        previous_state.append(current.as_mut());
        previous_state.push(vec![vec![NO_BUG; max_y]; max_x]);
        previous_state.push(vec![vec![NO_BUG; max_y]; max_x]);

        let mut next_state = vec![vec![vec![UNKNOWN; max_y]; max_x]; current.len() + 4];

        for layer in 1..next_state.len() - 1 {
            for x in 0..max_x as i32 {
                for y in 0..max_y as i32 {
                    break
                }
            }
        }
    }

    let mut result = 0;

    Answer { question: input.question, result: 0 }
}
