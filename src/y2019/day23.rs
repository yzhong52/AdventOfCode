use crate::helpers::parser::Input;
use crate::helpers::parser::Answer;
use std::sync::Arc;
use std::thread;
use crate::y2019::atomic_int_code_computer::AtomicIntCodeComputer;

const CONTROLLER_COUNT: usize = 50;

pub fn part1(input: Input<Vec<i128>>) -> Answer<usize> {
    let mut controllers = vec![];

    for i in 0..CONTROLLER_COUNT {
        let controller = AtomicIntCodeComputer::new(
            input.data.clone(),
            format!("Controller {}", i),
        );
        // when each computer boots up, it will request its network address via a single input instruction
        controller.input_single(i as i128);
        controllers.push(controller);
    }

    let references = Arc::new(controllers);

    let mut thread_handlers = vec![];
    for i in 0..CONTROLLER_COUNT {
        let references = Arc::clone(&references);

        let handle = thread::spawn(move || {
            let controller = &references[i];
            println!("Starting {}", controller.name);

            loop {
                println!("Run loop for {}", controller.name);

                let output = controller.execute(3);
                let (address, x, y) = (output[0], output[1], output[2]);
                println!("Send package from {}, to {}, x: {}, y: {}", controller.name, address, x, y);

                let receiver = &references[address as usize];
                receiver.input_multiple(vec![x, y]);
            }
        });

        thread_handlers.push(handle);
    }

    for handle in thread_handlers {
        handle.join().unwrap();
    }
    Answer { question: input.question, result: 0 }
}