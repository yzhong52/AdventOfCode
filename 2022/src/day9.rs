use itertools::Itertools;
use std::fs;

pub fn day9() -> (String, String) {
    let content = fs::read_to_string("input/day9").unwrap();
    run(content)
}

fn run(content: String) -> (String, String) {
    let lines: Vec<&str> = content.trim().split("\n").map(|line| line.trim()).collect();

    for line in lines {
        let mut splitted = line.split(" ");
        let dir = splitted.next().unwrap();
        let steps = splitted.next().unwrap().parse::<u32>().unwrap();
        println!("{} {}", dir, steps);
    }

    let part1 = 0;
    let part2 = 0;
    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day9_test() {
        let (part1, part2) = day9();
        assert_eq!(part1, "1818");
        assert_eq!(part2, "368368");
    }

    #[test]
    fn day9_example_test() {
        let input = "
            30373
            25512
            65332
            33549
            35390"
            .to_string();

        let (part1, part2) = run(input);
        assert_eq!(part1, "21");
        assert_eq!(part2, "8");
    }
}
