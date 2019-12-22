use super::super::helpers::parser::*;
use crate::helpers::models::_Point;

pub trait Letter {
    fn is_uppercase(&self) -> bool;
}

impl Letter for char {
    fn is_uppercase(&self) -> bool {
        return *self >= 'A' && *self <= 'Z';
    }
}

fn search_portal(map: &Vec<Vec<char>>) {
    let max_x = map.len();
    let max_y = map[0].len();
    for i in 0..max_x {
        for j in 0..max_y {
            if map[i][j].is_ascii_uppercase() {
                let point = _Point { x: i, y: j };
                let neighbours = point.neighbours4(max_x, max_y);
                let other = neighbours
                    .iter()
                    .filter(|pos| map[pos.x][pos.y].is_ascii_uppercase())
                    .nth(0)
                    .unwrap();

                let side1 = point.clone() + (point.clone() - other.clone());
                let side2 = other.clone() + (other.clone() - point.clone());
                let both_sides = [side1, side2];
                let portal_pos = both_sides.iter()
                    .filter(|x| x.is_valid(max_x, max_y) && map[x.x][x.y] == '.')
                    .nth(0)
                    .unwrap();

                println!("{}{}: {:?}", map[point.x][point.y], map[other.x][other.y], portal_pos);
            }
        }
    }
}

pub fn part1(input: Input<Vec<Vec<char>>>) -> Answer<usize> {
    search_portal(&input.data);
    Answer { question: input.question, result: 0 }
}

pub fn part2(input: Input<Vec<Vec<char>>>) -> Answer<i32> {
    Answer { question: input.question, result: 0 }
}
