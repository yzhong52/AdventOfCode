use super::super::helpers::parser::*;
use super::day9::*;
use std::collections::{HashMap, VecDeque};
use crate::helpers::models::{BigPoint, Point};
use std::thread::sleep;
use std::time::Duration;
use std::process::exit;

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

struct ArcadeCabinet {
    computer: SuperIntCodeComputer,
    screen: Vec<Vec<char>>,
    score: i128,
}

impl ArcadeCabinet {
    fn insert_coin(&mut self) {
        self.computer.numbers[0] = 2;
    }

    fn start(&mut self) {
        let memory = self.computer.numbers.clone();

        let mut paddle_targets: VecDeque<i128> = VecDeque::from(vec![21, 27, 27, 19, 13, 7, 1, 7, 15, 21, 21, 21, 19, 19, 19, 19, 19]);

        loop {
            let mut paddle_pos: BigPoint = BigPoint::origin();
            let mut loop_count = 0;
            let mut current_paddle_targets = paddle_targets.clone();

            self.computer.numbers = memory.clone();
            self.computer.index = 0;
            self.computer.input_queue = VecDeque::new();
            self.computer.relative_base = 0;
            self.computer.external_numbers = HashMap::new();
            let mut ball_pos = BigPoint::origin();
            println!("Game Restarted!!!!");
            println!("Total Knowledge: ");
            for t in &paddle_targets {
                print!("{},", t);
            };
            println!();

            // Game run loop
            loop {
                let x = match self.computer.run() {
                    SuperIntCodeResult::Output(val) => val,
                    SuperIntCodeResult::Halted => {
                        println!("Game halted!");
                        println!("{}", self.score);
                        exit(0);
                    }
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

                    if y == paddle_pos.y - 1 as i128 && third == BALL_TILE {
                        if current_paddle_targets.is_empty() {
                            paddle_targets.push_back(x);
                            println!("Restart!!!! New knowledge: {}", x);
                            break;
                        } else if *current_paddle_targets.front().unwrap() == x {
                            current_paddle_targets.pop_front();
                        } else {
                            unimplemented!();
                        }
                    }

                    if self.computer.input_queue.is_empty() {
                        let action = match current_paddle_targets.front() {
                            Some(target) => {
                                if target - paddle_pos.x == 0 {
                                    0
                                } else {
                                    (target - paddle_pos.x) / (target - paddle_pos.x).abs()
                                }
                            }
                            None => 0
                        };
                        self.computer.input_queue.push_back(action);
                    }
                } else {
                    self.score = third;
                }

                loop_count += 1;

                if loop_count > 989 {
                    let mut screen_buffer: String = String::new();
                    for row in &self.screen {
                        for c in row {
                            screen_buffer.push(c.clone());
                        }
                        screen_buffer.push('\n');
                    }
                    println!("{}", screen_buffer);
                    println!("Current score: {}", self.score);

                    println!("Pending input: {:?}", self.computer.input_queue.front());
                    println!("Loop: {:?}", loop_count);
                    println!("Paddle position: {:?}!", paddle_pos);
                    println!("Ball score: {:?}", ball_pos);
                    println!("Knowledge: {:?}", current_paddle_targets);
                    println!("-->");

                    match third {
                        HORIZONTAL_PADDLE_TILE | BALL_TILE => sleep(Duration::from_millis(200)),
                        _ => ()
                    };
                }
            }
        }
    }
}

pub fn part2(input: Input<Vec<i128>>) -> Answer<String> {

    // Use part 1 to just get the size of the screen
    let map = play_game(&input.data);
    let max_x = map.keys().into_iter().map(|p| p.x).max().unwrap();
    let max_y = map.keys().into_iter().map(|p| p.y).max().unwrap();

    let screen: Vec<Vec<char>> = vec![vec!['?'; max_x as usize + 1]; max_y as usize + 1];

    let computer = SuperIntCodeComputer {
        numbers: input.data.clone(),
        index: 0,
        input_queue: VecDeque::new(),
        relative_base: 0,
        external_numbers: HashMap::new(),
    };

    let mut arcade = ArcadeCabinet { computer, screen, score: 0 };

    arcade.insert_coin();
    arcade.start();

    Answer { question: input.question, result: "1".to_string() }
}
