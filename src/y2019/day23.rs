use crate::helpers::parser::Input;
use crate::helpers::parser::Answer;
use std::sync::{Arc, Mutex};
use std::thread;
use crate::y2019::atomic_int_code_computer::AtomicIntCodeComputer;
use std::thread::sleep;
use std::time::Duration;

const CONTROLLER_COUNT: usize = 50;

fn create_controllers(data: Vec<i128>) -> Vec<AtomicIntCodeComputer> {
    let mut controllers = vec![];

    for i in 0..CONTROLLER_COUNT {
        let controller = AtomicIntCodeComputer::new(
            data.clone(),
            format!("Controller {}", i),
        );
        // when each computer boots up, it will request its network address via a single input instruction
        controller.input_single(i as i128);
        controllers.push(controller);
    }

    controllers
}

pub fn part1(input: Input<Vec<i128>>) -> Answer<i128> {
    let mut result = Arc::new(Mutex::new(Option::None));
    let controllers: Arc<Vec<AtomicIntCodeComputer>> = Arc::new(create_controllers(input.data));

    let mut thread_handlers = vec![];
    for i in 0..CONTROLLER_COUNT {
        let controllers = Arc::clone(&controllers);
        let mut result = Arc::clone(&result);

        let handle = thread::spawn(move || {
            let controller = &controllers[i];
            loop {
                let output = controller.execute3();
                println!("Output {:?}", output);

                if let Some((address, x, y)) = output {
                    if address == 255 {
                        let mut result = result.lock().unwrap();
                        *result = Some(y);
                        break;
                    } else {
                        let receiver = &controllers[address as usize];
                        receiver.input_multiple(vec![x, y]);
                    }
                } else {
                    let result = result.lock().unwrap();
                    if result.is_some() {
                        break;
                    }
                }
            }

            println!("Shut down {}", &controller.name);
        });
        thread_handlers.push(handle);
    }

    for handle in thread_handlers {
        handle.join().unwrap();
    }

    let result = result.lock().unwrap();

    Answer { question: input.question, result: result.unwrap() }
}

pub fn part2(input: Input<Vec<i128>>) -> Answer<i128> {
    Answer { question: input.question, result: 1 }
}