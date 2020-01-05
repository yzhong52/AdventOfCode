use crate::helpers::parser::{Answer, Input, extract_between_plus};
use crate::int_code_computers::atomic_int_code_computer::AtomicIntCodeComputer;
use crate::int_code_computers::atomic_int_code_computer::AtomicIntCodeResult;
use std::collections::VecDeque;

pub fn part1(input: Input<Vec<i128>>) -> Answer<String> {
    let computer = AtomicIntCodeComputer::new(
        input.data,
        "droid".to_string());

    let mut items_state: u32 = 0;
    let mut previous_items_state: u32 = 255;

    let all_items = [
        "jam",
        "loom",
        "mug",
        "spool of cat6",
        "prime number",
        "food ration",
        "fuel cell",
        "manifold",
    ];
    let mut commands_to_security_checkpoint = VecDeque::from(vec![
        "east",
        "take food ration",
        "south",
        "take prime number",
        "north",
        "east",
        "take manifold",
        "east",
        "north",
        "north",
        "take fuel cell",
        "south",
        "east",
        "take spool of cat6",
        "west",
        "south",
        "east",
        "take jam",
        "west",
        "west",
        "west",
        "west",
        "north",
        "north",
        "north",
        "east",
        "east",
        "take loom",
        "west",
        "west",
        "south",
        "west",
        "take mug",
        "east",
        "south",
        "west",
        "north",
        "west",
        "north"
    ]);

    let mut output_buffer: String = String::new();
    loop {
        match computer.run() {
            AtomicIntCodeResult::Output(value) => {
                output_buffer.push(value as u8 as char);
            }
            AtomicIntCodeResult::WaitingInput => {
                print!("{}", output_buffer);
                output_buffer.clear();

                let line: String;
                if !commands_to_security_checkpoint.is_empty() {
                    let mut buffer = commands_to_security_checkpoint.pop_front().unwrap().to_string();
                    buffer.push('\n');
                    line = buffer;
                } else {
                    let mut buffer = String::new();

                    for i in 0..all_items.len() {
                        let now = items_state & (1 << i as u32);
                        let before = previous_items_state & (1 << i as u32);

                        if now == before {
                            // do nothing for this item
                        } else if now > 0 {
                            buffer += format!("take {}\n", all_items[i]).as_str();
                        } else if before > 0 {
                            buffer += format!("drop {}\n", all_items[i]).as_str();
                        }
                    }
                    previous_items_state = items_state;
                    items_state += 1;

                    line = buffer + "inv\nnorth\n";
                }

                print!("{}", line);

                let commands = line.chars().into_iter().map(|c| c as i128).collect();
                computer.input_multiple(commands);
            }
            AtomicIntCodeResult::Halted => {
                print!("{}", output_buffer);
                break;
            }
        }
    }

    let result = extract_between_plus(
        output_buffer.as_str(),
        " You should be able to get in by typing ",
        " on the keypad at the main airlock.");
    Answer { question: input.question, result }
}
