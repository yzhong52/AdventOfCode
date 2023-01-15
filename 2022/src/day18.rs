use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

type Point = (i32, i32, i32);

fn run(content: String) -> (String, String) {
    let points = content
        .trim()
        .split("\n")
        .map(|row| {
            let (x, y, z) = row
                .trim()
                .split(",")
                .map(|value| value.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap();
            (x, y, z)
        })
        .collect_vec();

    let points_set: HashSet<Point> = HashSet::from_iter(points.clone());

    let offsets = [
        (-1, 0, 0),
        (0, -1, 0),
        (0, 0, -1),
        (1, 0, 0),
        (0, 1, 0),
        (0, 0, 1),
    ];

    let mut part1 = 0;
    for (x, y, z) in &points {
        for (dx, dy, dz) in offsets {
            let x1 = x + dx;
            let y1 = y + dy;
            let z1 = z + dz;

            if !points_set.contains(&(x1, y1, z1)) {
                part1 += 1;
            }
        }
    }

    let mut part2 = 0;
    let max_x = points.iter().map(|p| p.0).max().unwrap() + 1;
    let min_x = points.iter().map(|p| p.0).min().unwrap() - 1;
    let max_y = points.iter().map(|p| p.1).max().unwrap() + 1;
    let min_y = points.iter().map(|p| p.1).min().unwrap() - 1;
    let max_z = points.iter().map(|p| p.2).max().unwrap() + 1;
    let min_z = points.iter().map(|p| p.2).min().unwrap() - 1;
    let start = (min_x, min_y, min_z);
    let mut stack = vec![start];
    let mut visited = HashSet::new();
    while stack.len() > 0 {
        let (x, y, z) = stack.pop().unwrap();

        if visited.contains(&((x, y, z))) {
            // already visited
            continue;
        }
        if x < min_x || x > max_x || y < min_y || y > max_y || z < min_z || z > max_z {
            // out side of the bounding box
            continue;
        }

        visited.insert((x, y, z));

        for (dx, dy, dz) in offsets {
            let x1 = x + dx;
            let y1 = y + dy;
            let z1 = z + dz;

            if points_set.contains(&(x1, y1, z1)) {
                // reach lava
                part2 += 1;
            } else {
                stack.push((x1, y1, z1));
            }
        }
    }

    println!("day18 part1: {}", part1);
    println!("day18 part2: {}", part2);
    (part1.to_string(), part2.to_string())
}

pub fn day18() -> (String, String) {
    let content = fs::read_to_string("input/day18").unwrap();
    run(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day18_example1_test() {
        let input = "
            1,1,1
            2,1,1"
            .to_string();

        let (part1, part2) = run(input);
        assert_eq!(part1, "10");
        assert_eq!(part2, "10");
    }

    #[test]
    fn day18_example_test() {
        let input = "
            2,2,2
            1,2,2
            3,2,2
            2,1,2
            2,3,2
            2,2,1
            2,2,3
            2,2,4
            2,2,6
            1,2,5
            3,2,5
            2,1,5
            2,3,5"
            .to_string();

        let (part1, part2) = run(input);
        assert_eq!(part1, "64");
        assert_eq!(part2, "58");
    }

    #[test]
    fn day18_test() {
        let (part1, part2) = day18();
        assert_eq!(part1, "4282");
        assert_eq!(part2, "2452");
    }
}
