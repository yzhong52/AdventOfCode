use std::collections::HashMap;
use super::super::helpers::models::*;

extern crate num;

use num::Integer;

pub fn get_rectangle<T, ValueType>(map: &HashMap<_Point<T>, ValueType>) -> Rect<_Point<T>>
    where T: Integer, T: Copy
{
    let mut lower = map.keys().nth(0).unwrap().clone();
    let mut upper = map.keys().nth(0).unwrap().clone();

    for p in map.keys() {
        lower.x = p.x.min(lower.x);
        upper.x = p.x.max(upper.x);

        lower.y = p.y.min(lower.y);
        upper.y = p.y.max(upper.y);
    }

    Rect { lower, upper }
}

// ^ y
// |
// |
// |         x
// + - - - - >
pub(crate) fn get_screen_buffer(map: &HashMap<BigPoint, char>, fill: char, min_with: usize, min_height: usize) -> Vec<Vec<char>> {
    let rect: Rect<_Point<i128>> = get_rectangle(&map);

    let width = (rect.upper.x - rect.lower.x + 1) as usize;
    let height = (rect.upper.y - rect.lower.y + 1) as usize;

    let mut buffer = vec![vec![fill; width.max(min_with)]; height.max(min_height)];

    for (p, c) in map {
        buffer[(rect.upper.y - p.y) as usize][(p.x - rect.lower.x) as usize] = *c;
    }

    buffer
}

pub(crate) fn print(buffer: &Vec<Vec<char>>) {
    let mut result = String::new();
    result.push('\n');
    for row in buffer {
        for c in row {
            result.push(' ');
            result.push(*c);
            result.push(' ');
        }
        result.push('\n');
    }
    println!("{}", result);
}