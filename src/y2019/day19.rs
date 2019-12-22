use super::super::helpers::parser::*;
use super::day9::*;

fn test_location(x: i128, y: i128, data: &Vec<i128>) -> i128 {
    let mut drone = SuperIntCodeComputer::new(data.clone());

    drone.input(x);
    drone.input(y);

    match drone.run() {
        SuperIntCodeResult::Output(value) => value,
        SuperIntCodeResult::Halted => unimplemented!()
    }
}

pub fn part1(input: Input<Vec<i128>>) -> Answer<i128> {
    let mut final_result = 0;
    for i in 0..50 {
        for j in 0..50 {
            final_result += test_location(i, j, &input.data);
        }
    }
    Answer { question: input.question, result: final_result }
}

pub fn part2(input: Input<Vec<i128>>) -> Answer<i128> {
    let mut x = 11;
    let mut y = 14;

    loop {
        let mut x_size = 0;
        let mut y_size = 0;

        while test_location(x + x_size, y, &input.data) == 1 {
            x_size += 1;
        }

        while test_location(x, y + y_size, &input.data) == 1 {
            y_size += 1;
        }

        if x_size >= 100 && y_size >= 100 {
            break;
        }

        if x_size > y_size {
            x += 1;
        } else {
            y += 1;
        }
    }
    Answer { question: input.question, result: 10000 * x + y }
}
