use std::{fs, vec};

pub fn day11() -> (String, String) {
    let content = fs::read_to_string("input/day11").unwrap();
    run(content)
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    operation: String,
    test_divisible: i64,
    throw_to_monkey_if_true: usize,
    throw_to_monkey_if_false: usize,
}

impl Monkey {
    fn inspect(&self, item: &i64) -> i64 {
        if self.operation.starts_with("new = old * old") {
            item * item
        } else if self.operation.starts_with("new = old + ") {
            item + self
                .operation
                .split(" ")
                .last()
                .unwrap()
                .parse::<i64>()
                .unwrap()
        } else if self.operation.starts_with("new = old * ") {
            item * self
                .operation
                .split(" ")
                .last()
                .unwrap()
                .parse::<i64>()
                .unwrap()
        } else {
            0
        }
    }
}

fn solve(
    mut monkeys: Vec<Monkey>,
    iterations: i32,
    worry_level_reducer: impl Fn(i64) -> i64,
) -> usize {
    let mut inspected_counts = vec![0; monkeys.len()];
    for _ in 0..iterations {
        for monkey_id in 0..monkeys.len() {
            let items = monkeys[monkey_id].items.clone();
            inspected_counts[monkey_id] += items.len();

            monkeys[monkey_id].items = vec![];
            for item in items {
                let inspected = worry_level_reducer(monkeys[monkey_id].inspect(&item));
                let monkey_id2 = if inspected % monkeys[monkey_id].test_divisible == 0 {
                    monkeys[monkey_id].throw_to_monkey_if_true
                } else {
                    monkeys[monkey_id].throw_to_monkey_if_false
                };
                monkeys[monkey_id2].items.push(inspected);
            }
        }
    }
    inspected_counts.sort_by(|a, b| b.cmp(a));

    inspected_counts[0] * inspected_counts[1]
}

fn run(content: String) -> (String, String) {
    let lines: Vec<&str> = content
        .split("\n")
        .map(|line| line.trim())
        .filter(|line| line.len() > 0)
        .collect();

    let mut monkeys = vec![];
    for i in (0..lines.len()).step_by(6) {
        // Each monkey has 6 lines
        let items_line = lines[i + 1]["Starting items: ".len()..].split(", ");
        let items: Vec<_> = items_line
            .map(|item| item.parse::<i64>().unwrap())
            .collect();

        let operation_line = lines[i + 2]["Operation: ".len()..].to_owned();
        let test_divisible = lines[i + 3]
            .split(" ")
            .last()
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let throw_to_monkey_if_true = lines[i + 4]
            .split(" ")
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let throw_to_monkey_if_false = lines[i + 5]
            .split(" ")
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let monkey = Monkey {
            items,
            operation: operation_line,
            test_divisible: test_divisible,
            throw_to_monkey_if_true: throw_to_monkey_if_true,
            throw_to_monkey_if_false: throw_to_monkey_if_false,
        };
        monkeys.push(monkey)
    }

    let part1_worry_level_reducer = move |worry_level: i64| worry_level / 3;

    let divider = monkeys.iter().fold(1, |acc, x| acc * x.test_divisible);
    let part2_worry_level_reducer = move |worry_level: i64| worry_level % divider;

    let part1 = solve(monkeys.clone(), 20, part1_worry_level_reducer);
    let part2 = solve(monkeys, 10000, part2_worry_level_reducer);

    println!("day11 part1: {:?}", part1);
    println!("day11 part2: {}", part2);

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11_test() {
        let (part1, part2) = day11();
        assert_eq!(part1, "107822");
        assert_eq!(part2, "27267163742");
    }

    #[test]
    fn day11_example_test() {
        let input = "
    Monkey 0:
        Starting items: 79, 98
        Operation: new = old * 19
        Test: divisible by 23
          If true: throw to monkey 2
          If false: throw to monkey 3
      
      Monkey 1:
        Starting items: 54, 65, 75, 74
        Operation: new = old + 6
        Test: divisible by 19
          If true: throw to monkey 2
          If false: throw to monkey 0
      
      Monkey 2:
        Starting items: 79, 60, 97
        Operation: new = old * old
        Test: divisible by 13
          If true: throw to monkey 1
          If false: throw to monkey 3
      
      Monkey 3:
        Starting items: 74
        Operation: new = old + 3
        Test: divisible by 17
          If true: throw to monkey 0
          If false: throw to monkey 1
"
        .to_string();

        let (part1, part2) = run(input);
        assert_eq!(part1, "10605");
        assert_eq!(part2, "2713310158");
    }
}
