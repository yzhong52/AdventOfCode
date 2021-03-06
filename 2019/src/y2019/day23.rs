use crate::helpers::parser::read_numbers_by_comma;
use std::sync::{Arc, Mutex};
use std::thread;
use crate::int_code_computers::atomic_int_code_computer::AtomicIntCodeComputer;
use crate::helpers::puzzle::Puzzle;

const CONTROLLER_COUNT: usize = 50;

fn create_controllers(data: &Vec<i128>) -> Vec<AtomicIntCodeComputer> {
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

pub struct Day23 {}

impl Puzzle<Vec<i128>, i128> for Day23 {
    fn day(&self) -> i8 {
        23
    }

    fn parser(&self) -> fn(String) -> Vec<i128> {
        read_numbers_by_comma
    }

    fn part1(&self, input: &Vec<i128>) -> i128 {
        let result = Arc::new(Mutex::new(Option::None));
        let controllers: Arc<Vec<AtomicIntCodeComputer>> = Arc::new(create_controllers(input));

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
        result.unwrap()
    }

    fn part2(&self, input: &Vec<i128>) -> i128 {
        let nat_package: Arc<Mutex<Option<(i128, i128)>>> = Arc::new(Mutex::new(Option::None));
        let last_nat_package: Arc<Mutex<Option<(i128, i128)>>> = Arc::new(Mutex::new(Option::None));
        let controllers: Arc<Vec<AtomicIntCodeComputer>> = Arc::new(create_controllers(input));
        let idles: Arc<Mutex<Vec<bool>>> = Arc::new(Mutex::new(vec![false; CONTROLLER_COUNT]));

        let mut thread_handlers = vec![];
        for i in 0..CONTROLLER_COUNT {
            let controllers = Arc::clone(&controllers);
            let nat_package = Arc::clone(&nat_package);
            let last_nat_package = Arc::clone(&last_nat_package);
            let idles = Arc::clone(&idles);

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
                            println!("Resume controller 0 with {:?} from C{:02}", nat_package.unwrap(), i);

                            match (*last_nat_package, *nat_package) {
                                (Some(last), Some(current)) if last.1 == current.1 => {
                                    break;
                                }
                                _ => {
                                    let (x, y) = nat_package.unwrap();
                                    *last_nat_package = *nat_package;
                                    *nat_package = None;

                                    idles[0] = false;
                                    controllers[0].input_multiple(vec![x, y]);
                                }
                            }
                        }
                    }
                }
            });
            thread_handlers.push(handle);
        }

        for handle in thread_handlers {
            handle.join().unwrap();
        }

        let result = nat_package.lock().unwrap();

        result.unwrap().1
    }
}
