use std::fs;

// https://github.com/rust-lang/rfcs/issues/1024
pub trait Puzzle<Input, Output> where Input: std::fmt::Debug, Output: std::fmt::Display {
    fn day(&self) -> i8;
    fn parser(&self) -> fn(filename: String) -> Input;
    fn part1(&self, input: &Input) -> Output;
    fn part2(&self, input: &Input) -> Output;

    fn save(&self, suffix: &str, result: Output) {
        let filename: String = format!("src/y2019/day{}.output.{}.txt", self.day(), suffix);
        println!("Result for question 2019 day {day} {part} is: {result}",
                 day = self.day(),
                 part = suffix,
                 result = result);
        println!("Save to file: {}\n", filename);
        fs::write(filename, format!("{}\n", result)).expect("Unable to write file");
    }

    fn run(&self) {
        let input = self.parser()(format!("src/y2019/day{}.input.txt", self.day()));

        let result1 = self.part1(&input);
        self.save("part1", result1);

        let result2 = self.part2(&input);
        self.save("part2", result2);
    }
}
