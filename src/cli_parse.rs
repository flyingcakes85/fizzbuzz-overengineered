use std::{env, vec};

#[derive(Debug)]
pub struct ProgramData {
    nums: Vec<usize>,
    words: Vec<&'static str>,
}

impl ProgramData {
    fn new() -> Self {
        Self {
            nums: vec![3, 5],
            words: vec!["Fizz", "Buzz"],
        }
    }
}

pub fn parse() -> ProgramData {
    let mut program_data: ProgramData = ProgramData::new();

    if env::args().len() > 1 {
        // parse args bruh
        program_data.nums = vec![0];
        program_data.words = vec!["todo"];
    }

    program_data
}
