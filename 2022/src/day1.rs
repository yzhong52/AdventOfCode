use std::fs;

pub fn day1() -> (i32, i32) {
    let content = fs::read_to_string("input/day1").unwrap();

    let rows: Vec<_> = content.split("\n").collect::<Vec<_>>();

    let grouped_numbers: Vec<Vec<_>> = rows.iter().fold(vec![vec![]], |mut acc, x| {
        if x.len() == 0 {
            acc.push(vec![]);
        } else {
            let value: i32 = x.parse().unwrap();
            acc.last_mut().unwrap().push(value);
        }
        acc
    });

    let mut nums = grouped_numbers
        .into_iter()
        .map(|group| group.iter().sum::<i32>())
        .collect::<Vec<i32>>();

    nums.sort();

    let max3: i32 = nums[nums.len() - 3..].iter().sum();

    let part1: i32 = *nums.last().unwrap();
    let part2: i32 = max3;
    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_test() {
        let (part1, part2) = day1();
        assert_eq!(part1, 69281);
        assert_eq!(part2, 201524);
    }
}
