use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashSet, fs};

pub fn day15() -> (String, String) {
    let content = fs::read_to_string("input/day15").unwrap();
    run(content, 2000000, (0, 4000000))
}

fn sorted_ranges(lines: &Vec<(i32, i32, i32, i32)>, target_y: i32) -> Vec<(i32, i32)> {
    let mut ranges = lines
        .iter()
        .filter_map(|(sx, sy, bx, by)| {
            let total_distance = i32::abs(sx - bx) + i32::abs(sy - by);
            let vertical_distance = i32::abs(target_y - sy);
            if vertical_distance <= total_distance {
                let diff = total_distance - vertical_distance;
                Some((sx - diff, sx + diff))
            } else {
                None
            }
        })
        .collect_vec();

    ranges.sort();
    ranges
}

fn solve_part1(lines: &Vec<(i32, i32, i32, i32)>, target_y: i32) -> i32 {
    let ranges = sorted_ranges(&lines, target_y);

    let mut last_pos = i32::MIN;
    let mut part1 = 0;
    for (left, right) in ranges {
        part1 += i32::max(0, right - i32::max(left - 1, last_pos));
        last_pos = i32::max(right, last_pos);
    }

    // minus the number of beacon on this row
    let beacons_on_row: HashSet<(i32, i32)> =
        HashSet::from_iter(lines.iter().flat_map(|(_, _, bx, by)| {
            if *by == target_y {
                Some((*bx, *by))
            } else {
                None
            }
        }));
    part1 -= beacons_on_row.len() as i32;

    part1
}

fn solve_part2(lines: &Vec<(i32, i32, i32, i32)>, search_range: (i32, i32)) -> usize {
    for y in search_range.0..=search_range.1 {
        let sorted_ranges = sorted_ranges(&lines, y);

        let mut x_last = 0;
        for (x_left, x_right) in sorted_ranges {
            if x_left > x_last {
                return (x_last as usize) * 4000000 as usize + y as usize;
            }
            x_last = x_last.max(x_right + 1);
        }
    }
    0
}

fn run(content: String, target_y: i32, search_range: (i32, i32)) -> (String, String) {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"
        )
        .unwrap();
    }
    let lines: Vec<(i32, i32, i32, i32)> = content
        .trim()
        .split("\n")
        .map(|line| -> (i32, i32, i32, i32) {
            let capture = RE.captures(line.trim()).unwrap();
            (1..=4)
                .into_iter()
                .map(|i| {
                    let c1 = capture.get(i).unwrap();
                    c1.as_str().parse::<i32>().unwrap()
                })
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let part1 = solve_part1(&lines, target_y);
    let part2 = solve_part2(&lines, search_range);

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day15_example_test() {
        let input = "
            Sensor at x=2, y=18: closest beacon is at x=-2, y=15
            Sensor at x=9, y=16: closest beacon is at x=10, y=16
            Sensor at x=13, y=2: closest beacon is at x=15, y=3
            Sensor at x=12, y=14: closest beacon is at x=10, y=16
            Sensor at x=10, y=20: closest beacon is at x=10, y=16
            Sensor at x=14, y=17: closest beacon is at x=10, y=16
            Sensor at x=8, y=7: closest beacon is at x=2, y=10
            Sensor at x=2, y=0: closest beacon is at x=2, y=10
            Sensor at x=0, y=11: closest beacon is at x=2, y=10
            Sensor at x=20, y=14: closest beacon is at x=25, y=17
            Sensor at x=17, y=20: closest beacon is at x=21, y=22
            Sensor at x=16, y=7: closest beacon is at x=15, y=3
            Sensor at x=14, y=3: closest beacon is at x=15, y=3
            Sensor at x=20, y=1: closest beacon is at x=15, y=3
            "
        .to_string();

        let (part1, part2) = run(input, 10, (0, 20));
        assert_eq!(part1, "26");
        assert_eq!(part2, "56000011");
    }

    #[test]
    fn day15_test() {
        let (part1, part2) = day15();
        assert_eq!(part1, "5112034");
        assert_eq!(part2, "13172087230812");
    }
}
