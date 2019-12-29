use crate::helpers::parser::{Answer, Input};
use crate::helpers::utils::print_grid;
use std::collections::HashSet;
use crate::helpers::models::_Point;
use std::thread::sleep;
use std::time::Duration;

const BUG: char = '#';
const NO_BUG: char = '.';

pub fn part1(input: Input<Vec<Vec<char>>>) -> Answer<usize> {
    let mut current: Vec<Vec<char>> = input.data;

    let mut appeared: HashSet<Vec<Vec<char>>> = HashSet::new();
    let max_x = current.len();
    let max_y = current[0].len();
    while !appeared.contains(&current) {
        print_grid(&current);

        appeared.insert(current.clone());
        let mut next_state = vec![vec!['?'; max_y]; max_x];

        for x in 0..max_x as i32 {
            for y in 0..max_y as i32 {
                let point = _Point { x, y };
                let count_bugs_nearby = point.neighbours4(max_x as i32, max_y as i32)
                    .iter()
                    .filter(|p| current[p.x as usize][p.y as usize] == BUG)
                    .count();

                if current[x as usize][y as usize] == BUG {
                    if count_bugs_nearby == 1 {
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

                print!(" {} ", count_bugs_nearby);
            }
            println!();
        }

        current = next_state;

        sleep(Duration::from_secs(1));
    }


    Answer { question: input.question, result: 0 }
}

pub fn part2(input: Input<Vec<Vec<char>>>) -> Answer<usize> {
    Answer { question: input.question, result: 0 }
}