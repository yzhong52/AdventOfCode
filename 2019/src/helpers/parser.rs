use std::fs;

#[deprecated]
#[derive(Copy, Clone)]
pub struct Question {
    pub year: i16,
    pub day: i8,
}

fn read_file_by(filename: String, pat: char) -> Vec<String> {
    let contents: String = fs::read_to_string(filename).expect("file not found");
    let lines: Vec<&str> = contents.split(pat).collect();
    return lines.iter()
        .map(|x| x.to_string()) // convert `&str` to `String`
        .filter(|x| !x.is_empty()) // filter out EOF
        .collect();
}

pub fn read_strings_by_line(filename: String) -> Vec<String> {
    read_file_by(filename, '\n')
}

fn read_numbers_by<T>(filename: String, pat: char) -> Vec<T>
    where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::fmt::Debug {
    let data: Vec<T> = read_file_by(filename, pat).iter()
        .filter(|x| *x != "\n")
        .map(|x| {
            // Filter out dummy special chars
            let s: String = x.chars().filter(|c| *c != '\n' && *c != ' ').collect();
            s.parse::<T>().unwrap()
        }).collect();
    data
}

pub fn read_numbers_by_comma<T>(filename: String) -> Vec<T>
    where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::fmt::Debug {
    return read_numbers_by(filename, ',');
}

pub fn read_numbers_by_line<T>(filename: String) -> Vec<T>
    where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::fmt::Debug {
    return read_numbers_by(filename, '\n');
}

pub fn read_single_string(filename: String) -> String {
    read_strings_by_line(filename).first().unwrap().to_string()
}

pub fn read_grid(filename: String) -> Vec<Vec<char>> {
    let grid: Vec<Vec<char>> = read_strings_by_line(filename)
        .iter()
        .map(|input| {
            input.chars().collect::<Vec<char>>()
        })
        .collect();

    grid
}

pub fn extract_between_plus(line: &str, left: &str, right: &str) -> String {
    let p1 = line.split(left).last().unwrap();
    return p1.split(right).next().unwrap().to_string();
}
