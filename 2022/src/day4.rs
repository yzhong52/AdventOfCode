use std::fs;

pub fn day4() -> (usize, usize) {
    let content = fs::read_to_string("input/day4").unwrap();
    day4_run(content)
}

fn day4_run(content: String) -> (usize, usize) {
    let rows: Vec<&str> = content.split("\n").filter(|row| row.len() > 0).collect();

    let row_pairs: Vec<_> = rows
        .iter()
        .map(|row| {
            let parsed_rows: Vec<_> = row
                .split(",")
                .map(|pair| {
                    let pair: Vec<_> = pair
                        .split("-")
                        .map(|value| value.parse::<i32>().unwrap())
                        .collect();
                    pair
                })
                .collect();
            parsed_rows
        })
        .collect();

    // For each row of pairs, count the number of contains
    let part1 = row_pairs
        .iter()
        .filter(|&pairs| {
            let pair1 = &pairs[0];
            let pair2 = &pairs[1];

            pair1[0] <= pair2[0] && pair2[1] <= pair1[1]
                || pair2[0] <= pair1[0] && pair1[1] <= pair2[1]
        })
        .count();

    let part2 = row_pairs
        .iter()
        .filter(|&pairs| {
            let pair1 = &pairs[0];
            let pair2 = &pairs[1];

            let lower = i32::max(pair1[0], pair2[0]);
            let upper = i32::min(pair1[1], pair2[1]);

            lower <= upper
        })
        .count();

    println!("Day 4 part 1: {:?}", part1);
    println!("Day 4 part 2: {:?}", part2);

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day4_test() {
        let (part1, part2) = day4();
        assert_eq!(part1, 580);
        assert_eq!(part2, 895);
    }

    #[test]
    fn day4_example_test() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
            .to_string();

        let (part1, part2) = day4_run(input);
        assert_eq!(part1, 2);
        assert_eq!(part2, 4);
    }
}
