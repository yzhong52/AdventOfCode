use std::collections::HashSet;
use std::fs;

pub fn day1_part2() {
    let filename = "2018:day:1:input.txt";
    let contents: String =
        fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut total = 0;
    let mut set: HashSet<i32> = HashSet::new();
    for number_string in contents.split("\n") {
        if number_string.len() != 0 {
            println!(
                "Parsing number {}, current total is {}",
                number_string, total
            );
            total += number_string.parse::<i32>().unwrap();

            if set.contains(&total) {
                println!("Already seen this frequency: {}", total);
                break;
            } else {
                println!("New total {}", total);
                set.insert(total);
            }
        }
    }
}
