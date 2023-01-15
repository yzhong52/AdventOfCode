use itertools::Either;
use itertools::Itertools;

use std::cmp::Ordering;
use std::fmt;
use std::fmt::Display;
use std::fs;

pub fn day13() -> (String, String) {
    let content = fs::read_to_string("input/day13").unwrap();
    run(content)
}

#[derive(Debug, Eq, Clone)]
struct Packet {
    children: Vec<Either<Packet, i32>>,
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        for i in 0..usize::min(self.children.len(), other.children.len()) {
            let compare_result = match (&self.children[i], &other.children[i]) {
                (Either::Left(package1), Either::Left(package2)) => package1.cmp(&package2),
                (Either::Right(number1), Either::Right(number2)) => number1.cmp(&number2),
                (Either::Left(package1), Either::Right(number2)) => {
                    let package2 = Packet {
                        children: vec![Either::Right(*number2)],
                    };
                    package1.cmp(&package2)
                }
                (Either::Right(number1), Either::Left(package2)) => {
                    let package1 = Packet {
                        children: vec![Either::Right(*number1)],
                    };
                    package1.cmp(&package2)
                }
            };
            if compare_result != Ordering::Equal {
                return compare_result;
            }
        }

        if self.children.len() > other.children.len() {
            Ordering::Greater
        } else if self.children.len() < other.children.len() {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        if self.children.len() != other.children.len() {
            return false;
        }

        for i in 0..usize::min(self.children.len(), other.children.len()) {
            let compare_result = match (&self.children[i], &other.children[i]) {
                (Either::Left(package1), Either::Left(package2)) => package1.eq(&package2),
                (Either::Right(number1), Either::Right(number2)) => number1.eq(&number2),
                (Either::Left(package1), Either::Right(number2)) => {
                    let package2 = Packet {
                        children: vec![Either::Right(*number2)],
                    };
                    package1.eq(&package2)
                }
                (Either::Right(number1), Either::Left(package2)) => {
                    let package1 = Packet {
                        children: vec![Either::Right(*number1)],
                    };
                    package1.eq(&package2)
                }
            };
            if !compare_result {
                return false;
            }
        }
        true
    }
}

fn parse(pairs: Vec<char>) -> Packet {
    let mut children: Vec<String> = vec![];
    let mut last_index = 1;
    let mut counter = 0;

    for i in 1..pairs.len() - 1 {
        if pairs[i] == '[' {
            counter += 1;
        } else if pairs[i] == ']' {
            counter -= 1;
        } else if pairs[i] == ',' && counter == 0 {
            let slice = &pairs[last_index..i];
            let child: String = slice.iter().collect();
            children.push(child);
            last_index = i + 1;
        }
    }

    // Last element, [1,2,3]
    //                    ^
    // `if` statement for handling edge case []
    if last_index < pairs.len() - 1 {
        let slice = &pairs[last_index..pairs.len() - 1];
        let child: String = slice.iter().collect();
        children.push(child);
    }

    let children: Vec<Either<Packet, i32>> = children
        .iter()
        .map(|child| {
            if child.starts_with("[") {
                let packet = parse(child.chars().collect_vec());
                Either::Left(packet)
            } else {
                let number = child.parse::<i32>().unwrap();
                Either::Right(number)
            }
        })
        .collect_vec();

    Packet { children }
}

impl Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut children_str = self.children.iter().map(|child| format!("{}", child));
        let children_str = children_str.join(",");
        write!(f, "[{}]", children_str)
    }
}

fn run(content: String) -> (String, String) {
    let mut lines = content.trim().split("\n").map(|line| line.trim());
    let content = lines.join("\n");

    let groups: Vec<&str> = content.split("\n\n").collect();

    let part1: usize = groups
        .iter()
        .enumerate()
        .map(|(index, group)| {
            let pairs: Vec<&str> = group.split("\n").collect();
            let left = parse(pairs[0].chars().collect_vec());
            let right = parse(pairs[1].chars().collect_vec());
            if left <= right {
                index + 1
            } else {
                0
            }
        })
        .sum();

    let mut packages: Vec<_> = content
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|x| parse(x.chars().collect_vec()))
        .collect();

    let divider1 = parse("[[2]]".chars().collect_vec());
    let divider2 = parse("[[6]]".chars().collect_vec());

    packages.push(divider1.clone());
    packages.push(divider2.clone());

    packages.sort();

    let index1 = packages.iter().position(|r| r == &divider1).unwrap() + 1;
    let index2 = packages.iter().position(|r| r == &divider2).unwrap() + 1;

    let part2 = index1 * index2;

    println!("day13 part1: {}", part1);
    println!("day13 part2: {}", part2);
    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day13_example_test() {
        let input = "
            [1,1,3,1,1]
            [1,1,5,1,1]
            
            [[1],[2,3,4]]
            [[1],4]
            
            [9]
            [[8,7,6]]
            
            [[4,4],4,4]
            [[4,4],4,4,4]
            
            [7,7,7,7]
            [7,7,7]
            
            []
            [3]
            
            [[[]]]
            [[]]
            
            [1,[2,[3,[4,[5,6,7]]]],8,9]
            [1,[2,[3,[4,[5,6,0]]]],8,9]"
            .to_string();

        let (part1, part2) = run(input);
        assert_eq!(part1, "13");
        assert_eq!(part2, "140");
    }

    #[test]
    fn day13_test() {
        let (part1, part2) = day13();
        assert_eq!(part1, "6656");
        assert_eq!(part2, "19716");
    }
}
