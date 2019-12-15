use std::fs;

use super::models::*;

#[derive(Copy, Clone)]
pub struct Question {
    pub year: i16,
    pub day: i8,
}

impl Question {
    pub fn y2018(day: i8) -> Question {
        Question { year: 2018, day }
    }

    pub fn y2019(day: i8) -> Question {
        Question { year: 2019, day }
    }
}

pub struct Answer<T> {
    pub question: Question,
    pub result: T,
}

pub struct Input<T> {
    pub question: Question,
    pub data: T,
}

fn read_raw_by(question: Question, pat: char) -> Vec<String> {
    let filename: String = format!("input/{}:day:{}:input.txt", question.year, question.day);
    println!("Reading file from {}", filename);
    let contents: String = fs::read_to_string(filename).expect("file not found");
    let result: Vec<&str> = contents.split(pat).collect();

    // Have to call iter() to get back a `Iter` type in order for `map` and `filter` to work
    // We `map` them from `str` to `String` and also `filter` out empty ones.
    // Finally, `collect` them back to a `Vec`.
    return result.iter().map(|x| x.to_string()).filter(|x| !x.is_empty()).collect();
}

fn read_raw_by_line(question: Question) -> Vec<String> {
    read_raw_by(question, '\n')
}

pub fn read_numbers_by<T>(question: Question, pat: char) -> Input<Vec<T>>
    where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::fmt::Debug {
    let data: Vec<T> = read_raw_by(question, pat).iter()
        .filter(|x| *x != "\n")
        .map(|x| {
            // Filter out dummy special chars
            let s: String = x.chars().filter(|c| *c != '\n' && *c != ' ').collect();
            s.parse::<T>().unwrap()
        }).collect();
    return Input { question, data };
}

pub fn read_numbers_by_comma<T>(question: Question) -> Input<Vec<T>>
    where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::fmt::Debug {
    return read_numbers_by(question, ',');
}

pub fn read_numbers_space<T>(question: Question) -> Input<Vec<T>>
    where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::fmt::Debug {
    return read_numbers_by(question, ' ');
}

pub fn read_numbers_by_line<T>(question: Question) -> Input<Vec<T>>
    where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::fmt::Debug {
    return read_numbers_by(question, '\n');
}

pub fn read_strings(question: Question) -> Input<Vec<String>> {
    return Input { question, data: read_raw_by_line(question) };
}

pub fn read_single_string(question: Question) -> Input<String> {
    return Input { question, data: read_raw_by_line(question).first().unwrap().to_string() };
}

pub fn read_points(question: Question) -> Input<Vec<Point>> {
    let points: Vec<Point> = read_raw_by_line(question)
        .iter()
        .map(|input| {
            let p: Vec<i32> = input.split(", ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.to_string().parse::<i32>().unwrap())
                .collect();

            Point { x: p[0], y: p[1] }
        })
        .collect();

    Input { question, data: points }
}

pub fn read_grid(question: Question) -> Input<Vec<Vec<char>>> {
    let grid: Vec<Vec<char>> = read_raw_by_line(question)
        .iter()
        .map(|input| {
            input.chars().collect::<Vec<char>>()
        })
        .collect();

    Input { question, data: grid }
}

impl<T> Answer<T> where T: std::fmt::Display {
    pub fn save_part1(&self) {
        self.save_as("part1")
    }

    pub fn save_part2(&self) {
        self.save_as("part2")
    }

    fn save_as(&self, suffix: &str) {
        let filename: String = format!("output/{}:day:{}:{}.txt", self.question.year, self.question.day, suffix);
        println!("Result for question {year} day {day} {part} is: {result}",
                 year = self.question.year,
                 day = self.question.day,
                 part = suffix,
                 result = self.result);
        println!("Saving file to {}\n", filename);
        fs::write(filename, format!("{}\n", self.result)).expect("Unable to write file");
    }
}

// Not sure how to make the templating working with
// use std::str::pattern::Pattern;
pub(crate) fn extract_between(line: &str, left: char, right: char) -> String {
    let p1 = line.split(left).last().unwrap();
    return p1.split(right).next().unwrap().to_string();
}

pub fn extract_between_plus(line: &str, left: &str, right: &str) -> String {
    let p1 = line.split(left).last().unwrap();
    return p1.split(right).next().unwrap().to_string();
}
