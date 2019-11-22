struct Answer<T> {
    value: T,
}

impl<T> Answer<T> where T: std::fmt::Debug {
    pub fn save(&self, day: i8) {
        let filename = format!("2018:day:{}:output.txt", day);
        fs::write(filename, format!("{:#?}", self.value)).expect("Unable to write file");
    }
}
