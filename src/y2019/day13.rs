use super::super::helpers::parser::*;
use std::collections::HashMap;
use crate::helpers::models::BigPoint;
use std::thread::sleep;
use std::time::Duration;
use crate::int_code_computers::super_int_code_computer::SuperIntCodeComputer;
use crate::int_code_computers::super_int_code_computer::SuperIntCodeResult;
use crate::helpers::puzzle::Puzzle;

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

    let mut robot = SuperIntCodeComputer::new(input.clone());

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


struct ArcadeCabinet {
    computer: SuperIntCodeComputer,
    screen: Vec<Vec<char>>,
    score: i128,
}

impl ArcadeCabinet {
    fn insert_coin(&mut self) {
        self.computer.instructions[0] = 2;
    }

    fn play(&mut self, render: bool) -> i128 {
        let mut paddle_pos: BigPoint = BigPoint::origin();
        let mut loop_count = 0;
        let mut ball_pos = BigPoint::origin();

        // Game run loop
        loop {
            let x = match self.computer.run() {
                SuperIntCodeResult::Output(val) => val,
                SuperIntCodeResult::Halted => break,
            };

            let y = match self.computer.run() {
                SuperIntCodeResult::Output(val) => val,
                SuperIntCodeResult::Halted => unimplemented!(),
            };

            let third = match self.computer.run() {
                SuperIntCodeResult::Output(val) => val,
                SuperIntCodeResult::Halted => unimplemented!(),
            };

            if x >= 0 {
                self.screen[y as usize][x as usize] = match third {
                    EMPTY_TILE => ' ',
                    WALL_TILE => '#',
                    BLOCK_TILE => 'x',
                    HORIZONTAL_PADDLE_TILE => '=',
                    BALL_TILE => 'O',
                    _ => unimplemented!()
                };

                match third {
                    HORIZONTAL_PADDLE_TILE => {
                        paddle_pos.x = x;
                        paddle_pos.y = y;
                    }
                    BALL_TILE => {
                        ball_pos.x = x;
                        ball_pos.y = y;
                    }
                    _ => ()
                }

                while self.computer.input_queue.len() > 0 {
                    self.computer.input_queue.pop_front();
                }
                let position_difference = ball_pos.x - paddle_pos.x;
                if position_difference == 0 {
                    self.computer.input_queue.push_back(0);
                } else {
                    self.computer.input_queue.push_back(position_difference / position_difference.abs());
                }
            } else {
                self.score = third;
            }

            loop_count += 1;

            if render {
                let mut screen_buffer: String = String::new();
                for row in &self.screen {
                    screen_buffer.push(' ');
                    for c in row {
                        screen_buffer.push(c.clone());
                    }
                    screen_buffer.push('\n');
                }
                println!("\n{}\n", screen_buffer);
                println!(" Current score: {}", self.score);

                println!(" Pending input: {:?}", self.computer.input_queue);
                println!(" Run loop: {:?}", loop_count);
                println!(" Paddle position: {:?}", paddle_pos);
                println!(" Ball position: {:?}", ball_pos);
                println!(" -->");

                match third {
                    HORIZONTAL_PADDLE_TILE | BALL_TILE => sleep(Duration::from_millis(30)),
                    _ => ()
                };
            }
        }
        return self.score;
    }
}

pub struct Day13 {}

impl Puzzle<Vec<i128>, i128> for Day13 {
    fn day(&self) -> i8 {
        13
    }

    fn parser(&self) -> fn(String) -> Vec<i128> {
        parse_numbers_by_comma
    }

    fn part1(&self, input: &Vec<i128>) -> i128 {
        let map = play_game(input);
        let result = map.values().filter(|x| **x == BLOCK_TILE).count();
        result as i128
    }

    fn part2(&self, input: &Vec<i128>) -> i128 {

        // Use part 1 to just get the size of the screen
        let map = play_game(input);
        let max_x = map.keys().into_iter().map(|p| p.x).max().unwrap();
        let max_y = map.keys().into_iter().map(|p| p.y).max().unwrap();

        let screen: Vec<Vec<char>> = vec![vec!['?'; max_x as usize + 1]; max_y as usize + 1];

        let computer = SuperIntCodeComputer::new(input.clone());

        let mut arcade = ArcadeCabinet { computer, screen, score: 0 };

        arcade.insert_coin();

        let final_score = arcade.play(false);

        final_score
    }
}
