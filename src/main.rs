mod day1_part2;
mod Answer;

use std::fs;

fn read_strings(day: i8) -> Vec<String> {
    let filename = format!("2018:day:{}:input.txt", day);
    let contents: String = fs::read_to_string(filename).expect("file not found");
    let split = contents.split('\n');
    let result: Vec<&str> = split.collect();
    // Have to call iter() to get back a `Iter` type in order for `map` and `filter` to work
    // We `map` them to `String` and also `filter` out empty ones.
    // Finally `collect` them back to a `Vec`.
    return result.iter().map(|x| x.to_string()).filter(|x| !x.is_empty()).collect();
}

fn read_ints(day: i8) -> Vec<i32> {
    return read_strings(day).iter().map(|x| x.parse::<i32>().unwrap()).collect()
}



fn main() {
    day1_part2::day1_part2(read_ints(1));
}
