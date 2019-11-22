use super::helpers::Input;
use super::helpers::Answer;
use std::collections::HashMap;

pub fn day2(input: Input<Vec<String>>) -> Answer<i32> {
    let mut twos = 0;
    let mut threes = 0;

    for st in input.data {
        println!("{:?}", st);
        let mut map: HashMap<char, i32> = HashMap::new();
        println!("{:?}", map);

        for ch in st.chars() {
            let count = map.get(&ch).unwrap_or(&0) + 1;
            map.insert(ch, count);
        }

        let mut has_two = false;
        let mut has_three = false;

        for count in map.values() {
            match count {
                2 => {
                    has_two = true;
                }
                3 => {
                    has_three = true;
                }
                _ => { },
            }
        }

        if has_two {
            twos += 1
        }

        if has_three {
            threes += 1
        }

        println!("{:?} {} {}", &map, has_two, has_three);
        println!("==> {}x{}\n\n\n", twos, threes);
    }

    println!("{:?}", twos * threes);

    return Answer { question: input.question, result: twos * threes };
}
