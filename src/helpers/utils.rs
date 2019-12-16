use std::collections::{HashMap, VecDeque};
use super::super::helpers::parser::*;
use super::super::helpers::models::*;

extern crate num;

use num::Integer;

pub fn get_rectangle<T, ValueType>(map: &HashMap<_Point<T>, ValueType>) -> _Rect<_Point<T>>
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

    _Rect { lower, upper }
}


fn get_screen_buffer(map: HashMap<BigPoint, char>) -> Vec<Vec<char>> {
    let rect: _Rect<_Point<i128>> = get_rectangle(&map);
    vec![vec![' '; (rect.upper.y - rect.lower.y + 1) as usize]; (rect.upper.x - rect.lower.x) as usize]
}
