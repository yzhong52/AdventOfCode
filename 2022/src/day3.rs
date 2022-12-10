use std::collections::HashSet;
use std::fs;
use std::slice::Chunks;

pub fn day3() -> (u32, u32) {
    let content = fs::read_to_string("input/day3").unwrap();
    day3_run(content)
}

fn score(char: char) -> u32 {
    if char.is_uppercase() {
        char as u32 - 'A' as u32 + 27
    } else {
        char as u32 - 'a' as u32 + 1
    }
}

fn day3_run(content: String) -> (u32, u32) {
    let rows: Vec<&str> = content.split("\n").filter(|row| row.len() > 0).collect();

    let rucksacks: Vec<_> = rows
        .iter()
        .map(|row| (&row[0..row.len() / 2], &row[row.len() / 2..]))
        .collect();

    let priorities: Vec<_> = rucksacks
        .iter()
        .map(|(left, right)| {
            let left_chars: HashSet<char> = left.chars().collect();
            let right_chars: HashSet<char> = right.chars().collect();

            // There is should be only one intersect element
            let intersect = left_chars.intersection(&right_chars);
            let char = *intersect.last().unwrap();

            score(char)
        })
        .collect();

    let part1: u32 = priorities.into_iter().sum();

    println!("Day 3 Part 1: {:?}", part1);

    // Convert each row to HashSet
    let set_rows = rows
        .into_iter()
        .map(|row| row.chars().collect::<HashSet<_>>())
        .collect::<Vec<_>>();

    // Break into chunks of 3
    let chunks: Chunks<HashSet<char>> = set_rows.chunks(3);

    let groups: Vec<u32> = chunks
        .map(|chunk| {
            let first = chunk[0].clone();

            let output_set = &chunk[1..]
                .iter()
                .fold(first, |acc, x| acc.intersection(x).map(|x| *x).collect());

            // There is should be only one intersect element
            let ch = *output_set.iter().next().unwrap();

            score(ch)
        })
        .collect();

    let part2 = groups.iter().sum();

    println!("Day 3 Part 2: {:?}", part2);

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3_test() {
        let (part1, part2) = day3();
        assert_eq!(part1, 8109);
        assert_eq!(part2, 2738);
    }

    #[test]
    fn day3_example_test() {
        let (part1, part2) = day3_run(
            r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#
                .to_string(),
        );
        assert_eq!(part1, 157);
        assert_eq!(part2, 70);
    }
}
