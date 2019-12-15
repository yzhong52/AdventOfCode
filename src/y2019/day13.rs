use super::super::helpers::parser::*;
use super::day9::*;
use std::collections::{HashMap, VecDeque};
use crate::helpers::models::BigPoint;
use std::thread::sleep;
use std::time::Duration;

// 0 is an empty tile. No game object appears in this tile.
const EMPTY_TILE: i128 = 0;
// 1 is a wall tile. Walls are indestructible barriers.
const WALL_TILE: i128 = 1;
// 2 is a block tile. Blocks can be broken by the ball.
const BLOCK_TILE: i128 = 2;
// 3 is a horizontal paddle tile. The paddle is indestructible.
const HORIZONTAL_PADDLE_TILE: i128 = 3;
// 4 is a ball tile. The ball moves diagonally and bounces off objects.
const BALL_TILE: i128 = 4;


fn play_game(input: &Vec<i128>) -> HashMap<BigPoint, i128> {
    let mut map: HashMap<BigPoint, i128> = HashMap::new();

    let mut robot = SuperIntCodeComputer {
        numbers: input.clone(),
        index: 0,
        input_queue: VecDeque::new(),
        relative_base: 0,
        external_numbers: HashMap::new(),
    };

    loop {
        let x = match robot.run() {
            SuperIntCodeResult::Output(val) => val,
            SuperIntCodeResult::Halted => break,
        };

        let y = match robot.run() {
            SuperIntCodeResult::Output(val) => val,
            SuperIntCodeResult::Halted => unimplemented!(),
        };

        let tile_id = match robot.run() {
            SuperIntCodeResult::Output(val) => val,
            SuperIntCodeResult::Halted => unimplemented!(),
        };

        map.insert(BigPoint { x, y }, tile_id);
    }

    map
}

pub fn part1(input: Input<Vec<i128>>) -> Answer<usize> {
    let map = play_game(&input.data);
    let r = map.values().filter(|x| **x == BLOCK_TILE).count();
    Answer { question: input.question, result: r }
}

pub fn part2(input: Input<Vec<i128>>) -> Answer<String> {

    // Use part 1 to just get the size of the scren
    let map = play_game(&input.data);
    let max_x = map.keys().into_iter().map(|p| p.x).max().unwrap();
    let max_y = map.keys().into_iter().map(|p| p.y).max().unwrap();

    let mut screen = vec![vec![' '; max_x as usize + 1]; max_y as usize + 1];

    let mut robot = SuperIntCodeComputer {
        numbers: input.data.clone(),
        index: 0,
        input_queue: VecDeque::new(),
        relative_base: 0,
        external_numbers: HashMap::new(),
    };
    loop {
        let x = match robot.run() {
            SuperIntCodeResult::Output(val) => val,
            SuperIntCodeResult::Halted => break,
        };

        let y = match robot.run() {
            SuperIntCodeResult::Output(val) => val,
            SuperIntCodeResult::Halted => unimplemented!(),
        };

        let tile_id = match robot.run() {
            SuperIntCodeResult::Output(val) => val,
            SuperIntCodeResult::Halted => unimplemented!(),
        };

        screen[y as usize][x as usize] = match tile_id {
            EMPTY_TILE => ' ',
            WALL_TILE => '#',
            BLOCK_TILE => 'x',
            HORIZONTAL_PADDLE_TILE => '=',
            BALL_TILE => 'O',
            _ => unimplemented!()
        };

        let mut output: String = String::new();
        for row in &screen {
            for c in row {
                output.push(c.clone());
            }
            output.push('\n');
        }
        println!("{}", output);
        println!();
        println!("-->");

        sleep(Duration::from_millis(200));
    }


    Answer { question: input.question, result: "1".to_string() }
}
