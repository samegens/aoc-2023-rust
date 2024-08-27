use std::fs;
use std::str::Lines;

/// Because we need something to be the owner of the strings that is read from the file,
/// we create a struct that will hold the string and offer a function to create an iterator on it.
pub struct InputReader {
    input: String,
}

impl InputReader {
    pub fn new(day_nr: u8) -> Self {
        let path = format!("./input/{:02}.txt", day_nr);
        let input = fs::read_to_string(&path).unwrap();
        InputReader { input }
    }

    pub fn lines(&self) -> Lines {
        self.input.lines()
    }
}

pub fn create_input_reader(day_nr: u8) -> InputReader {
    InputReader::new(day_nr)
}
