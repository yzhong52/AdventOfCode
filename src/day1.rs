use std::fs;

pub fn fn day1_part1() {
    let filenames = [
        // "2018:day:1:input.txt",
        "2018:day:1:input_test1.txt"
    ]

    for filename in filenames {
        let contents: String =
        fs::read_to_string(filename).expect("Something went wrong reading the file");
        let mut total = 0;
        for number_string in contents.split("\n") {
            if number_string.len() != 0 {
                println!("Parsing number {}", number_string);
                total += number_string.parse::<i32>().unwrap();
            }
        }
        println!("{}", total)
    }
}
