use crate::helpers::parser::Input;
use crate::helpers::parser::Answer;
use std::process::exit;
use std::sync::Arc;
use std::thread;
use crate::y2019::day23_computer::{AtomicIntCodeComputer, AtomicIntCodeResult};

pub fn part1(input: Input<Vec<i128>>) -> Answer<usize> {
    let mut controllers = vec![];
    let mut handles = vec![];

    for i in 0..50 {
        let controller = AtomicIntCodeComputer::new(input.data.clone());
        controller.input(i as i128);
        controllers.push(Arc::new(controller));
    }

    for i in 0..50 {
        let mut controller = Arc::clone(&controllers[i]);

        let handle = thread::spawn(move || {
            println!("Running computer {}", i);
            controller.input(-1);
            match controller.run() {
                AtomicIntCodeResult::Output(value) => {
                    println!("Output from {}: {}", i, value)
                }
                AtomicIntCodeResult::Halted => exit(0),
            }
            controller.input(-1);
            match controller.run() {
                AtomicIntCodeResult::Output(value) => {
                    println!("Output from {}: {}", i, value)
                }
                AtomicIntCodeResult::Halted => exit(0),
            }
            controller.input(-1);
            match controller.run() {
                AtomicIntCodeResult::Output(value) => {
                    println!("Output from {}: {}", i, value)
                }
                AtomicIntCodeResult::Halted => exit(0),
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    Answer { question: input.question, result: 0 }
}