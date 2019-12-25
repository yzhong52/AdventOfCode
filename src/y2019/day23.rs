use crate::helpers::parser::{Input, Answer};
use crate::y2019::day9::SuperIntCodeComputer;
use crate::y2019::day9::SuperIntCodeResult;
use std::process::exit;
use std::time::Duration;

use std::thread;

pub fn part1(input: Input<Vec<i128>>) -> Answer<usize> {
    let mut controllers = vec![];

    for i in 0..50 {
        thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        let mut controller = SuperIntCodeComputer::new(input.data.clone());
        controller.input(i as i128);
        controllers.push(controller);
    }

    loop {
        for i in 0..50 {
            let controller = &mut controllers[i];
            println!("Running computer {}", i);
            controller.input(-1);
            match controller.run() {
                SuperIntCodeResult::Output(value) => {
                    println!("Output from {}: {}", i, value)
                }
                SuperIntCodeResult::Halted => exit(0),
            }
            controller.input(-1);
            match controller.run() {
                SuperIntCodeResult::Output(value) => {
                    println!("Output from {}: {}", i, value)
                }
                SuperIntCodeResult::Halted => exit(0),
            }
            controller.input(-1);

            match controller.run() {
                SuperIntCodeResult::Output(value) => {
                    println!("Output from {}: {}", i, value)
                }
                SuperIntCodeResult::Halted => exit(0),
            }
        }
        break;
    }

    Answer { question: input.question, result: 0 }
}