use super::helpers::Input;
use super::helpers::Answer;

fn diff(c1: &char, c2: &char) -> i32 {
    return i32::abs(*c1 as i32 - *c2 as i32);
}

pub fn part1(input: Input<String>) -> Answer<usize> {
    let mut stack: Vec<char> = Vec::new();

    for x in input.data.chars() {
        match stack.last() {
            Some(c) => {
                if diff(&x, c) == diff(&'A', &'a') {
                    println!("Has some value {} - {} match", c, x);
                    stack.pop();
                } else {
                    println!("Has some value {} pushed", c);
                    stack.push(x);
                }
            }
            None => {
                println!("It is empty");
                stack.push(x);
            }
        }
    }

    return Answer { question: input.question, result: stack.len() };
}