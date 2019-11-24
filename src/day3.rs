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

pub fn part1(input: Vec<String>) {
    let rects: Vec<Rect> = input.iter().map(|x| parse(x)).collect();

    let width = rects.iter().map(|r| r.x + r.width).max().unwrap() as usize;
    let height = rects.iter().map(|r| r.y + r.height).max().unwrap() as usize;

    // Just need a 2D array, way too complicated: https://stackoverflow.com/a/36376568/1035008
    let mut grid_data = vec![0; width * height];
    let mut grid_base: Vec<_> = grid_data.as_mut_slice().chunks_mut(width).collect();
    let grid: &mut [&mut [_]] = grid_base.as_mut_slice();

    for r in &rects {
        for x in r.x..r.x + r.width {
            for y in r.y..r.y + r.height {
                // println!("{}, {}", x, y);
                grid[x as usize][y as usize] += 1;
            }
        }
    }

    let mut result = 0;
    for x in 0..width {
        for y in 0..height {
            result += (grid[x as usize][y as usize] > 1) as i32;
        }
    }

    for r in &rects {
        let mut has_overlap = false;
        for x in r.x..r.x + r.width {
            for y in r.y..r.y + r.height {
                if grid[x as usize][y as usize] > 1 {
                    has_overlap = true;
                    break;
                }
            }
        }

        if !has_overlap {
            println!("{:?}", r.id);
        }
    }
    println!("{}", result);
}
