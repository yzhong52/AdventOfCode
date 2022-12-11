use itertools::Itertools;
use std::{collections::HashSet, fs};

pub fn day6() -> (String, String) {
    let content = fs::read_to_string("input/day6").unwrap();
    run(content)
}

fn run(content: String) -> (String, String) {
    // remove any trailing '\n'
    let content = content.trim();

    let part1 = find_distinct(content, 4);
    let part2 = find_distinct(content, 14);

    println!("day6 part1: {:?}", part1);
    println!("day6 part2: {:?}", part2);

    (part1.to_string(), part2.to_string())
}

fn find_distinct(content: &str, distinct_count: usize) -> usize {
    (distinct_count..content.len())
        .find_or_first(|idx| {
            let last_4 = &content.as_bytes()[*idx - distinct_count..*idx];
            let set: HashSet<_> = HashSet::from_iter(last_4);
            set.len() == distinct_count
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day6_test() {
        let (part1, part2) = day6();
        assert_eq!(part1, "1582");
        assert_eq!(part2, "3588");
    }

    #[test]
    fn day6_example_test() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string();

        let (part1, part2) = run(input);
        assert_eq!(part1, "11");
        assert_eq!(part2, "26");
    }
}
