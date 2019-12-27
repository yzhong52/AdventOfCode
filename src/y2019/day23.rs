use crate::helpers::parser::Input;
use crate::helpers::parser::Answer;
use std::sync::{Arc, Mutex};
use std::thread;
use crate::y2019::atomic_int_code_computer::AtomicIntCodeComputer;

const CONTROLLER_COUNT: usize = 50;

fn create_controllers(data: Vec<i128>) -> Vec<AtomicIntCodeComputer> {
    let mut controllers = vec![];

    for i in 0..CONTROLLER_COUNT {
        let controller = AtomicIntCodeComputer::new(
            data.clone(),
            format!("C{:02}", i),
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
        let result = Arc::clone(&result);

        let handle = thread::spawn(move || {
            let controller = &controllers[i];
            while result.lock().unwrap().is_none() {
                let output = controller.execute3();

                if let Some((address, x, y)) = output {
                    if address == 255 {
                        let mut result = result.lock().unwrap();
                        *result = Some(y);
                    } else {
                        let receiver = &controllers[address as usize];
                        receiver.input_multiple(vec![x, y]);
                    }
                }
            }

            println!("[{}] Shut down", &controller.name);
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
    let nat_package: Arc<Mutex<Option<(i128, i128)>>> = Arc::new(Mutex::new(Option::None));
    let last_nat_package: Arc<Mutex<Option<(i128, i128)>>> = Arc::new(Mutex::new(Option::None));
    let controllers: Arc<Vec<AtomicIntCodeComputer>> = Arc::new(create_controllers(input.data));
    let idles: Arc<Mutex<Vec<bool>>> = Arc::new(Mutex::new(vec![false; CONTROLLER_COUNT]));

    let mut thread_handlers = vec![];
    for i in 0..CONTROLLER_COUNT {
        let controllers = Arc::clone(&controllers);
        let mut nat_package = Arc::clone(&nat_package);
        let mut last_nat_package = Arc::clone(&last_nat_package);
        let mut idles = Arc::clone(&idles);

        let handle = thread::spawn(move || {
            let controller = &controllers[i];
            loop {
                let output = controller.execute3();

                if let Some((address, x, y)) = output {
                    if address == 255 {
                        let mut nat_package = nat_package.lock().unwrap();
                        *nat_package = Some((x, y));
                    } else {
                        let receiver = &controllers[address as usize];
                        receiver.input_multiple(vec![x, y]);
                        idles.lock().unwrap()[address as usize] = false;
                    }
                } else {
                    let mut idles = idles.lock().unwrap();

                    idles[i] = true;

                    let mut nat_package = nat_package.lock().unwrap();
                    let mut last_nat_package = last_nat_package.lock().unwrap();


                    let all_idled = idles.iter().all(|x| *x);

                    if all_idled && nat_package.is_some() {
                        println!("Resume controllers with {:?}", nat_package);

                        match (*last_nat_package, *nat_package) {
                            (Some(last), Some(current)) if last.1 == current.1 => {
                                break;
                            }
                            _ => ()
                        }

                        *last_nat_package = *nat_package;
                        let receiver = &controllers[0];
                        receiver.input_multiple(vec![nat_package.unwrap().0, nat_package.unwrap().1]);
                        idles[0] = false;
                        *nat_package = None;
                    }
                }
            }

            println!("[{}] Shut down", &controller.name);
        });
        thread_handlers.push(handle);
    }

    for handle in thread_handlers {
        handle.join().unwrap();
    }

    let result = nat_package.lock().unwrap();

    Answer { question: input.question, result: result.unwrap().1 }
}