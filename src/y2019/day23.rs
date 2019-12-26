use crate::helpers::parser::Input;
use crate::helpers::parser::Answer;
use std::process::exit;
use std::sync::Arc;
use std::thread;
use crate::y2019::atomic_int_code_computer::AtomicIntCodeComputer;
use crate::y2019::atomic_int_code_computer::AtomicIntCodeResult;

const CONTROLLER_COUNT: usize = 50;

pub fn part1(input: Input<Vec<i128>>) -> Answer<usize> {
    let mut controllers = vec![];
    let mut handles = vec![];

    for i in 0..CONTROLLER_COUNT {
        let controller = Arc::new(AtomicIntCodeComputer::new(
            input.data.clone(),
            format!("Controller {}", i),
        ));
        controller.input(i as i128);
        controllers.push(Arc::new(controller));
    }

    for i in 0..CONTROLLER_COUNT {
        let controller = Arc::clone(&controllers[i]);
//        let all = Arc::clone(&controllers);

        let handle = thread::spawn(move || {
            println!("Running computer {}", i);
            controller.input(-1);
            let output = controller.execute(3);
            let (address, x, y) = (output[0], output[1], output[2]);
            println!("address {}, x: {}, y: {}", address, x, y);

//            let controller1 = all[address as usize];
//            controller1.input(-1);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    Answer { question: input.question, result: 0 }
}