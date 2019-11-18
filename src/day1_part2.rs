use std::collections::HashSet;
use std::fs;

pub fn day1_part2() {
    let filenames = [
        "2018:day:1:input.txt",
        // "2018:day:1:input_test1.txt",
        // "2018:day:1:input_test2.txt",
    ];

    for filename in filenames.iter() {
        let contents: String =
            fs::read_to_string(filename).expect("Something went wrong reading the file");
        let mut total = 0;
        let mut seens: HashSet<i32> = HashSet::new();
        seens.insert(0);
        loop {
            let mut found = false;
            for number_string in contents.split("\n") {
                if number_string.len() != 0 {
                    // println!(
                    //     "Parsing number {}, current total is {}",
                    //     number_string, total
                    // );
                    total += number_string.parse::<i32>().unwrap();

                    if seens.contains(&total) {
                        println!("Already seen this frequency: {}", total);
                        found = true;
                        break;
                    } else {
                        println!("New total {}", total);
                        seens.insert(total);
                    }
                }
            }

            if found {
                break;
            }

            println!("Loop finished. Restart");
        }
    }
}
