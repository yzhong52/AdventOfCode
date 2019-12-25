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

    const NOT_SET: i32 = -1;
    const INVALID: i32 = -2;

    let max_x = points.iter().map(|p| p.x).max().unwrap() as usize;
    let max_y = points.iter().map(|p| p.y).max().unwrap() as usize;

    let mut closest: Vec<Vec<i32>> = vec![vec![NOT_SET; max_y + 1]; max_x + 1];

    for (index, point) in points.iter().enumerate() {
        closest[point.x as usize][point.y as usize] = index as i32
    }

    let total_points_count = points.len();
    let mut current_layer: Vec<Point> = points;

    while current_layer.len() > 0 {
        let mut next_layer: Vec<Point> = vec![];

        for current_point in current_layer {
            let source_value = closest[current_point.x as usize][current_point.y as usize];

            if source_value == INVALID {
                continue;
            }

            // Expand in 4 directions
            for next in current_point.neighbours4(max_x as i32, max_y as i32) {
                let current_value = closest[next.x as usize][next.y as usize];

                if current_value == NOT_SET {
                    closest[next.x as usize][next.y as usize] = closest[current_point.x as usize][current_point.y as usize];
                    next_layer.push(next);
                } else if current_value != source_value && next_layer.contains(&next) {
                    closest[next.x as usize][next.y as usize] = INVALID
                }
            }
        }

        current_layer = next_layer;
    }

    let mut infinity: HashSet<i32> = HashSet::new();

    for y in 0..max_y {
        infinity.insert(closest[0][y]);
        infinity.insert(closest[max_x][y]);
    }
    for x in 0..max_x {
        infinity.insert(closest[x][0]);
        infinity.insert(closest[x][max_y]);
    }

    let mut counts = vec![0; total_points_count];
    for y in 0..max_y {
        for x in 0..max_x {
            if !infinity.contains(&closest[x][y]) {
                counts[closest[x][y] as usize] += 1;
            }
        }
    }

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
