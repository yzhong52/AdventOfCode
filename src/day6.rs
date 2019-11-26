use super::helpers::parser::*;
use super::helpers::models::*;

pub fn part1(input: Input<Vec<Point>>) -> Answer<usize> {
    let points = input.data;
    println!("Points {:?}", points);

    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();

    let mut array: Vec<Vec<i32>> = vec![vec![0; (max_y + 1) as usize]; (max_x + 1) as usize];

    for (index, point) in points.iter().enumerate() {
        array[point.x as usize][point.y as usize] = (index + 1) as i32
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
        for p in layer {
            if array[p.x as usize][p.y as usize] == -1 {
                continue
            }

            // expand in 4 directions
            for dir in dirs.iter() {
                let next = Point { x: p.x as i32 + dir[0], y: p.y as i32 + dir[1] };

                if next.x >= 0 && next.x <= max_x && next.y >= 0 && next.y <= max_y {
                    let current_value = array[next.x as usize][next.y as usize];

                    if current_value == 0 {
                        array[next.x as usize][next.y as usize] = array[p.x as usize][p.y as usize]
                    } else {
                        array[next.x as usize][next.y as usize] = -1
                    }
                }
            }
        }

        layer = vec![];
    }

    for row in &array {
        for c in row {
            print!("{} ", c);
        }
        println!();
    }

    Answer { question: input.question, result: 0 }
}