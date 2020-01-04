use super::super::helpers::parser::*;
use std::collections::{HashSet, BinaryHeap, HashMap};
use crate::helpers::models::Point;

fn count_visible(data: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let mut set: HashSet<(i32, bool)> = HashSet::new();
    for (i, row) in data.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c == &'#' {
                let diff_x = x as i32 - i as i32;
                let diff_y = y as i32 - j as i32;

                match (diff_x, diff_y) {
                    (0, 0) => (),
                    (0, dy) if dy < 0 => {
                        set.insert((std::i32::MIN, false));
                    }
                    (0, dy) if dy > 0 => {
                        set.insert((std::i32::MAX, false));
                    }
                    (dx, dy) => {
                        let value: i32 = ((dy as f32 / dx as f32) * 10000.0) as i32;
                        set.insert((value, dx > 0));
                    }
                }
            }
        }
    }
    set.len()
}

fn get_target(data: &Vec<Vec<char>>) -> (usize, (usize, usize)) {
    let mut result = 0;
    let mut target = (0, 0);
    for (i, row) in data.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c == &'#' {
                let count = count_visible(&data, i, j);
                if result < count {
                    result = count;
                    target = (i, j);
                }
            }
        }
    }
    (result, target)
}


pub fn part1(input: Input<Vec<Vec<char>>>) -> Answer<usize> {
    let (result, _) = get_target(&input.data);
    Answer { question: input.question, result }
}

// Sort by distance to the origin
impl std::cmp::Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let this = self.x * self.x + self.y * self.y;
        let that = other.x * other.x + other.y * other.y;
        that.cmp(&this)
    }
}

impl std::cmp::PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

pub fn part2(input: Input<Vec<Vec<char>>>) -> Answer<i32> {
    let (_, (x,  y)) = get_target(&input.data);

    let mut map: HashMap<(bool, i32), BinaryHeap<Point>> = HashMap::new();
    for (i, row) in input.data.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '#' {
                // Swapping the x and y here so that it is easier to think about the quadrant
                let diff_x = j as i32 - y as i32;
                let diff_y = x as i32 - i as i32;

                match (diff_x, diff_y) {
                    (0, 0) => (),
                    (0, dy) if dy > 0 => {
                        let key: (bool, i32) = (true, std::i32::MAX);
                        // https://stackoverflow.com/a/41418147/1035008
                        let heap = map.entry(key).or_insert(BinaryHeap::new());
                        heap.push(Point { x: diff_x, y: diff_y });
                    }
                    (0, dy) if dy < 0 => {
                        let key = (true, std::i32::MIN);
                        let heap = map.entry(key).or_insert(BinaryHeap::new());
                        heap.push(Point { x: diff_x, y: diff_y });
                    }

                    (dx, dy) => {
                        let value: i32 = ((dy as f32 / dx as f32) * 10000.0) as i32;
                        let key = (dx > 0, value);
                        let heap = map.entry(key).or_insert(BinaryHeap::new());
                        heap.push(Point { x: diff_x, y: diff_y });
                    }
                }
            }
        }
    }
    let mut keys: Vec<(bool, i32)> = map.keys().map(|v| v.clone()).collect();
    keys.sort_by_key(|&k| std::cmp::Reverse(k));

    let mut index = 0;
    let mut vaporized_count = 0;
    let mut last_point = Point { x: 0, y: 0 };

    // Looking for the 200th asteroid to be vaporized
    while vaporized_count < 200 && index < 10000 {
        let key = keys[index % keys.len()];

        let heap = map.get_mut(&key).unwrap();
        if heap.len() > 0 {
            last_point = heap.pop().unwrap();
            vaporized_count += 1;
        };
        index += 1;
    }

    Answer { question: input.question, result: (last_point.x + y as i32) * 100 + (x as i32 - last_point.y) }
}
