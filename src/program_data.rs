#[derive(Debug)]
pub struct ProgramData {
    pub nums: Vec<usize>,
    pub words: Vec<&'static str>,
}

impl ProgramData {
    pub fn new() -> Self {
        Self {
            nums: vec![3, 5],
            words: vec!["Fizz", "Buzz"],
        }
    }
}
