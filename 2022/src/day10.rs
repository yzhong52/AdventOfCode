use std::{fs, vec};

pub fn day10() -> (String, String) {
    let content = fs::read_to_string("input/day10").unwrap();
    run(content)
}

fn run(content: String) -> (String, String) {
    let lines: Vec<&str> = content.trim().split("\n").map(|line| line.trim()).collect();

    let mut cycle = 0;
    let mut pos: i32 = 1; // Sprite position
    let mut key_cycles = vec![20, 60, 100, 140, 180, 220, i32::MAX];
    key_cycles.reverse();

    let mut part1 = 0;
    let mut buffer: Vec<char> = vec!['.'; 40 * 6];

    for line in lines {
        let mut split = line.split(" ");
        let operation = split.next().unwrap();

        if operation == "noop" {
            if pos - 1 <= cycle % 40 && cycle % 40 <= pos + 1 {
                buffer[cycle as usize] = '#';
            }
            cycle += 1;
        } else if operation == "addx" {
            let value: i32 = split
                .next()
                .unwrap()
                .parse()
                .expect("unable to parse value");

            for _ in 0..2 {
                if pos - 1 <= cycle % 40 && cycle % 40 <= pos + 1 {
                    buffer[cycle as usize] = '#';
                }
                cycle += 1;
            }

            if cycle >= *key_cycles.last().unwrap() {
                part1 += pos * key_cycles.pop().unwrap();
            }
            pos += value;
        }
    }

    let part2: String = buffer
        .chunks(40)
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");

    println!("day10 part1: {:?}", part1);
    println!("day10 part2: \n{}", part2);

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10_test() {
        let (part1, part2) = day10();
        assert_eq!(part1, "17020");
        assert_eq!(
            part2,
            "
###..#....####.####.####.#.....##..####.
#..#.#....#.......#.#....#....#..#.#....
#..#.#....###....#..###..#....#....###..
###..#....#.....#...#....#....#.##.#....
#.#..#....#....#....#....#....#..#.#....
#..#.####.####.####.#....####..###.####."
                .trim()
        );
    }

    #[test]
    fn day10_example_test() {
        let input = "
            addx 15
            addx -11
            addx 6
            addx -3
            addx 5
            addx -1
            addx -8
            addx 13
            addx 4
            noop
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx -35
            addx 1
            addx 24
            addx -19
            addx 1
            addx 16
            addx -11
            noop
            noop
            addx 21
            addx -15
            noop
            noop
            addx -3
            addx 9
            addx 1
            addx -3
            addx 8
            addx 1
            addx 5
            noop
            noop
            noop
            noop
            noop
            addx -36
            noop
            addx 1
            addx 7
            noop
            noop
            noop
            addx 2
            addx 6
            noop
            noop
            noop
            noop
            noop
            addx 1
            noop
            noop
            addx 7
            addx 1
            noop
            addx -13
            addx 13
            addx 7
            noop
            addx 1
            addx -33
            noop
            noop
            noop
            addx 2
            noop
            noop
            noop
            addx 8
            noop
            addx -1
            addx 2
            addx 1
            noop
            addx 17
            addx -9
            addx 1
            addx 1
            addx -3
            addx 11
            noop
            noop
            addx 1
            noop
            addx 1
            noop
            noop
            addx -13
            addx -19
            addx 1
            addx 3
            addx 26
            addx -30
            addx 12
            addx -1
            addx 3
            addx 1
            noop
            noop
            noop
            addx -9
            addx 18
            addx 1
            addx 2
            noop
            noop
            addx 9
            noop
            noop
            noop
            addx -1
            addx 2
            addx -37
            addx 1
            addx 3
            noop
            addx 15
            addx -21
            addx 22
            addx -6
            addx 1
            noop
            addx 2
            addx 1
            noop
            addx -10
            noop
            noop
            addx 20
            addx 1
            addx 2
            addx 2
            addx -6
            addx -11
            noop
            noop
            noop"
            .to_string();

        let (part1, part2) = run(input);
        assert_eq!(part1, "13140");
        assert_eq!(
            part2,
            "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
                .trim()
                .to_owned()
        );
    }
}
