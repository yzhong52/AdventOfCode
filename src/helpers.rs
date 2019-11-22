use std::fs;

#[derive(Copy, Clone)]
pub struct Question {
    pub year: i16,
    pub day: i8,
}

pub struct Answer<T> {
    pub question: Question,
    pub result: T,
}

pub struct Input<T> {
    pub question: Question,
    pub data: T,
}

fn read_raw(question: Question) -> Vec<String> {
    let filename = format!("{}:day:{}:input.txt", question.year, question.day);
    let contents: String = fs::read_to_string(filename).expect("file not found");
    let result: Vec<&str> = contents.split('\n').collect();

    // Have to call iter() to get back a `Iter` type in order for `map` and `filter` to work
    // We `map` them to `String` and also `filter` out empty ones.
    // Finally `collect` them back to a `Vec`.
    return result.iter().map(|x| x.to_string()).filter(|x| !x.is_empty()).collect();
}

pub fn read_ints(question: Question) -> Input<Vec<i32>> {
    let data: Vec<i32> = read_raw(question).iter().map(|x| x.parse::<i32>().unwrap()).collect();
    return Input { question, data };
}

pub fn read_strings(question: Question) -> Input<Vec<String>> {
    return Input { question, data: read_raw(question) };
}

impl<T> Answer<T> where T: std::fmt::Debug {
    pub fn save_as(&self, suffix: &str) {
        let filename: String = format!("{}:day:{}:output_{}.txt", self.question.year, self.question.day, suffix);
        fs::write(filename, format!("{:#?}\n", self.result)).expect("Unable to write file");
    }

    pub fn save(&self) {
        let filename: String = format!("{}:day:{}:output.txt", self.question.year, self.question.day);
        fs::write(filename, format!("{:#?}\n", self.result)).expect("Unable to write file");
    }
}
