use crate::helpers::parser::{Input, Answer};
use crate::y2019::day9::SuperIntCodeComputer;
use crate::y2019::day9::SuperIntCodeResult;
use std::process::exit;
use std::time::Duration;
use std::rc::Rc;
use std::sync::{atomic, Arc};
use std::thread;
use crate::y2019::day23_computer::AtomicIntCodeComputer;

pub fn part1(input: Input<Vec<i128>>) -> Answer<usize> {
    let mut controllers = vec![];
    for i in 0..50 {
        let mut controller = AtomicIntCodeComputer::new(input.data.clone());
        controller.input(i as i128);
        controllers.push(Arc::new(controller));
    }

    for i in 0..50 {
        let mut controller = Arc::clone(&controllers[i]);

        thread::spawn(move || {
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
        });
    }

    Answer { question: input.question, result: 0 }
}