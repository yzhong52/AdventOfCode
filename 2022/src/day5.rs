use itertools::{Either, Itertools};
use std::{fs, iter};

pub fn day5() -> (String, String) {
    let content = fs::read_to_string("input/day5").unwrap();
    day5_run(content)
}

#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    count: usize,
}

fn day5_run(content: String) -> (String, String) {
    let (board, moves): (Vec<&str>, Vec<&str>) = content
        .split("\n")
        .filter(|row| row.len() > 0)
        .partition_map(|row| {
            if row.starts_with("move") {
                Either::Right(row)
            } else {
                Either::Left(row)
            }
        });

    let board_width = board
        .last()
        .unwrap()
        .split(" ")
        .filter(|x| x.len() > 0)
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let mut stacks: Vec<Vec<char>> = iter::repeat(vec![]).take(board_width).collect();

    for i in (0..board.len() - 1).rev() {
        let row = board[i];
        for (j, ch) in row.chars().enumerate() {
            if ch == ' ' {
                continue;
            }
            let bottom_row_char = board[board.len() - 1].as_bytes()[j] as char;
            let parse_result = bottom_row_char.to_digit(10);
            match parse_result {
                None => (),
                Some(column) => {
                    // -1 to make it 0-indexed
                    stacks[column as usize - 1].push(ch)
                }
            };
        }
    }

    let moves: Vec<_> = moves
        .iter()
        .map(|row| {
            let numbers: Vec<usize> = row
                .split(" ")
                .filter_map(|word| -> Option<usize> {
                    if let Ok(value) = word.parse::<usize>() {
                        Some(value)
                    } else {
                        None
                    }
                })
                .collect();
            Move {
                from: numbers[1] - 1, // make 0-indexed
                to: numbers[2] - 1,   // make 0-indexed
                count: numbers[0],
            }
        })
        .collect();

    let mut stacks_part1 = stacks.clone();
    for move1 in &moves {
        for _ in 0..move1.count {
            let value = stacks_part1[move1.from].pop().unwrap();
            stacks_part1[move1.to].push(value)
        }
    }

    let mut stacks_part2 = stacks.clone();
    for move2 in &moves {
        for i in 0..move2.count {
            let stack = &stacks_part2[move2.from];
            let value = stack[stack.len() - move2.count + i];
            stacks_part2[move2.to].push(value)
        }

        for _ in 0..move2.count {
            stacks_part2[move2.from].pop();
        }
    }

    let part1 = stacks_part1
        .iter()
        .map(|stack| stack[stack.len() - 1])
        .join("");
    let part2 = stacks_part2
        .iter()
        .map(|stack| stack[stack.len() - 1])
        .join("");

    println!("day5 part1: {:?}", part1);
    println!("day5 part2: {:?}", part2);

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day5_test() {
        let (part1, part2) = day5();
        assert_eq!(part1, "FJSRQCFTN");
        assert_eq!(part2, "CJVLJQPHS");
    }

    #[test]
    fn day5_example_test() {
        let input = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
            .to_string();

        let (part1, part2) = day5_run(input);
        assert_eq!(part1, "CMZ");
        assert_eq!(part2, "MCD");
    }
}
