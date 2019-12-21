use super::super::helpers::parser::*;
use super::super::helpers::models::*;
use std::collections::HashSet;

#[allow(dead_code)]
fn print(array: &Vec<Vec<i32>>) {
    let size_x = array.len();
    let size_y = array[0].len();
    for y in 0..size_y {
        for x in 0..size_x {
            print!("{:2} ", array[x][y]);
        }
        println!();
    }
    println!("-------------");
}

pub fn part1(input: Input<Vec<Point>>) -> Answer<usize> {
    let points = input.data;
    // println!("Points {:?}", points);

    let init: i32 = -1;
    let invalid: i32 = -2;

    let max_x = points.iter().map(|p| p.x).max().unwrap() as usize;
    let max_y = points.iter().map(|p| p.y).max().unwrap() as usize;

    let mut array: Vec<Vec<i32>> = vec![vec![init; max_y + 1]; max_x + 1];

    for (index, point) in points.iter().enumerate() {
        array[point.x as usize][point.y as usize] = index as i32
    }

    let total_points_count = points.len();
    let mut layer = points;
    let dirs: [[i32; 2]; 4] = [
        [0, 1],
        [1, 0],
        [-1, 0],
        [0, -1]
    ];

    while layer.len() > 0 {
        let mut next_layer: Vec<Point> = vec![];

        for p in layer {
            let source_value = array[p.x as usize][p.y as usize];

            if source_value == invalid {
                continue;
            }

            // expand in 4 directions
            for dir in dirs.iter() {
                let next = Point { x: p.x as i32 + dir[0], y: p.y as i32 + dir[1] };

                if next.is_valid(max_x as i32, max_y as i32) {
                    let current_value = array[next.x as usize][next.y as usize];

                    if current_value == init {
                        array[next.x as usize][next.y as usize] = array[p.x as usize][p.y as usize];
                        next_layer.push(next);
                    } else if current_value != source_value && next_layer.contains(&next) {
                        array[next.x as usize][next.y as usize] = invalid
                    }
                }
            }
        }

        layer = next_layer;

        // print(&array);
    }

    // print(&array);

    let mut infinity: HashSet<i32> = HashSet::new();

    let size_x = array.len();
    let size_y = array[0].len();
    for y in 0..size_y {
        infinity.insert(array[0][y]);
        infinity.insert(array[max_x][y]);
    }
    for x in 0..size_x {
        infinity.insert(array[x][0]);
        infinity.insert(array[x][max_y]);
    }
    // println!("{:?}", infinity);

    let mut counts = vec![0; total_points_count];
    for y in 0..size_y {
        for x in 0..size_x {
            if !infinity.contains(&array[x][y]) {
                counts[array[x][y] as usize] += 1;
            }
        }
    }
    // println!("{:?}", counts);

    Answer { question: input.question, result: *counts.iter().max().unwrap() }
}

pub fn part2(input: Input<Vec<Point>>) -> Answer<usize> {
    let points = &input.data;

    let max_x = points.iter().map(|p| p.x).max().unwrap() as usize;
    let max_y = points.iter().map(|p| p.y).max().unwrap() as usize;

    let mut safe_count = 0;

    for x in 0..=max_x {
        for y in 0..=max_y {
            let mut total = 0;
            for p in points {
                total += i32::abs(p.x - x as i32);
                total += i32::abs(p.y - y as i32);
            }


            if total < 10000 {
                safe_count += 1;
            }
        }
    }

    Answer { question: input.question, result: safe_count }
}
