use super::super::helpers::parser::*;

struct Rect {
    id: String,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

// #1 @ 108,350: 22x29
fn parse(line: &std::string::String) -> Rect {
    let parts = line.split_whitespace().collect::<Vec<&str>>();

    let origin = parts[2].split(',').collect::<Vec<&str>>();
    let x = origin[0].parse::<u32>().unwrap();

    // Just need to drop the last char, https://stackoverflow.com/a/37888550/1035008
    let mut y_value = origin[1].to_string();
    y_value.truncate(y_value.len() - 1);
    let y = y_value.parse::<u32>().unwrap();

    let size = parts[3].split('x').collect::<Vec<&str>>();
    let width = size[0].parse::<u32>().unwrap();
    let height = size[1].parse::<u32>().unwrap();

    return Rect { id: parts[0].to_string(), x, y, width, height };
}

struct Processed {
    rects: Vec<Rect>,
    width: usize,
    height: usize,
    grid: Vec<Vec<i32>>,
}

fn precess(input: &Input<Vec<String>>) -> Processed {
    let rects: Vec<Rect> = input.data.iter().map(|x| parse(x)).collect();

    let width = rects.iter().map(|r| r.x + r.width).max().unwrap() as usize;
    let height = rects.iter().map(|r| r.y + r.height).max().unwrap() as usize;

    // Just need a 2D array, way too complicated: https://stackoverflow.com/a/36376568/1035008
    let mut grid: Vec<Vec<i32>> = vec![vec![0; height]; width];

    for r in &rects {
        for x in r.x..r.x + r.width {
            for y in r.y..r.y + r.height {
                grid[x as usize][y as usize] += 1;
            }
        }
    }

    Processed { rects, width, height, grid }
}

pub fn part1(input: Input<Vec<String>>) -> Answer<i32> {
    let processed = precess(&input);

    let mut result = 0;
    for x in 0..processed.width {
        for y in 0..processed.height {
            result += (processed.grid[x as usize][y as usize] > 1) as i32;
        }
    }

    Answer { question: input.question, result }
}

pub fn part2(input: Input<Vec<String>>) -> Answer<String> {
    let processed = precess(&input);

    for r in &processed.rects {
        let mut has_overlap = false;
        for x in r.x..r.x + r.width {
            for y in r.y..r.y + r.height {
                if processed.grid[x as usize][y as usize] > 1 {
                    has_overlap = true;
                    break;
                }
            }
        }

        if !has_overlap {
            return Answer { question: input.question, result: r.id.clone() };
        }
    }
    Answer { question: input.question, result: "Not Found".to_string() }
}