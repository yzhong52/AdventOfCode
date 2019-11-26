use super::helpers::parser::*;
use super::helpers::models::*;

pub fn part1(input: Input<Vec<Point>>) -> Answer<usize> {
    let points = input.data;
    println!("Points {:?}", points);

    let init = '-';
    let invalid = '.';

    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();

    let mut array: Vec<Vec<char>> = vec![vec![init; (max_y + 1) as usize]; (max_x + 1) as usize];

    for (index, point) in points.iter().enumerate() {
        array[point.x as usize][point.y as usize] = ((index) as u8 + 'a' as u8) as char
    }

    println!("{:?}", array);

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

                if next.x >= 0 && next.x <= max_x && next.y >= 0 && next.y <= max_y {
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


        for i in 0..=max_y as usize {
            for j in 0..=max_x as usize {
                print!("{} ", array[j][i]);
            }
            println!();
        }
        println!("--..---------");
    }

    for i in 0..=max_y as usize {
        for j in 0..=max_x as usize {
            print!("{} ", array[j][i]);
        }
        println!();
    }

    Answer { question: input.question, result: 0 }
}
